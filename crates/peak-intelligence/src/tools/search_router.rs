use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;

#[async_trait]
pub trait SearchProvider: Send + Sync {
    fn name(&self) -> &'static str;
    async fn search(&self, query: &str) -> Result<Value>;
    fn is_available(&self) -> bool;
}

// 1. BRAVE SEARCH (PRIMARY/PAID)
pub struct BraveSearchProvider;

#[async_trait]
impl SearchProvider for BraveSearchProvider {
    fn name(&self) -> &'static str { "Brave Search" }
    
    fn is_available(&self) -> bool {
        env::var("BRAVE_SEARCH_API_KEY").is_ok()
    }

    async fn search(&self, query: &str) -> Result<Value> {
        let api_key = env::var("BRAVE_SEARCH_API_KEY")?;
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.search.brave.com/res/v1/web/search")
            .query(&[("q", query)])
            .header("X-Subscription-Token", api_key)
            .header("Accept", "application/json")
            .send()
            .await?;

        let data: Value = res.json().await?;
        
        // Transform Brave Results to standard format
        let mut standard_results = Vec::new();
        if let Some(web) = data["web"]["results"].as_array() {
            for item in web {
                standard_results.push(json!({
                    "title": item["title"],
                    "snippet": item["description"],
                    "link": item["url"]
                }));
            }
        }
        
        Ok(json!(standard_results))
    }
}

// 2. TAVILY SEARCH (SECONDARY/HYBRID)
pub struct TavilySearchProvider;

#[async_trait]
impl SearchProvider for TavilySearchProvider {
    fn name(&self) -> &'static str { "Tavily Search" }
    
    fn is_available(&self) -> bool {
        env::var("TAVILY_API_KEY").is_ok()
    }

    async fn search(&self, query: &str) -> Result<Value> {
        let api_key = env::var("TAVILY_API_KEY")?;
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.tavily.com/search")
            .json(&json!({
                "api_key": api_key,
                "query": query,
                "search_depth": "basic",
                "max_results": 5
            }))
            .send()
            .await?;

        let data: Value = res.json().await?;
        
        // Transform Tavily Results to standard format
        let mut standard_results = Vec::new();
        if let Some(results) = data["results"].as_array() {
            for item in results {
                standard_results.push(json!({
                    "title": item["title"],
                    "snippet": item["content"],
                    "link": item["url"]
                }));
            }
        }
        
        Ok(json!(standard_results))
    }
}

// 3. DUCKDUCKGO SCRAPER (FALLBACK/FREE)
pub struct DuckDuckGoScraper;

#[async_trait]
impl SearchProvider for DuckDuckGoScraper {
    fn name(&self) -> &'static str { "DuckDuckGo Scraper (Free)" }
    fn is_available(&self) -> bool { true } // Always free

    async fn search(&self, query: &str) -> Result<Value> {
        crate::tools::web_search(query).await
    }
}

// 4. THE ROUTER
pub struct SearchRouter {
    providers: Vec<Box<dyn SearchProvider>>,
}

impl SearchRouter {
    pub fn new() -> Self {
        Self {
            providers: vec![
                Box::new(BraveSearchProvider),
                Box::new(TavilySearchProvider),
                Box::new(DuckDuckGoScraper),
            ],
        }
    }

    pub async fn execute(&self, query: &str) -> Result<Value> {
        for provider in &self.providers {
            if provider.is_available() {
                log::info!("🔍 Attempting search with: {}", provider.name());
                match provider.search(query).await {
                    Ok(results) => {
                        // Ensure it's not an empty result array from an API error
                        if results.as_array().map(|a| !a.is_empty()).unwrap_or(false) {
                            return Ok(results);
                        }
                        log::warn!("⚠️ Provider '{}' returned empty results, falling back...", provider.name());
                    }
                    Err(e) => {
                        log::warn!("❌ Provider '{}' failed: {}. Falling back...", provider.name(), e);
                    }
                }
            }
        }
        
        Err(anyhow::anyhow!("All search providers failed or were unavailable."))
    }
}
