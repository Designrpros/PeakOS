use peak_os_intelligence::tools;

#[tokio::main]
async fn main() {
    println!("🚀 Testing PeakWebSearch (DuckDuckGo Scraper)...");

    let query = "Rust programming language";
    println!("🔍 Searching for: '{}'", query);

    match tools::web_search_routed(query).await {
        Ok(results) => {
            println!("✅ Search successful!");
            if let Some(arr) = results.as_array() {
                println!("Found {} results:", arr.len());
                for (i, res) in arr.iter().enumerate() {
                    println!("\n  [Result {}]", i + 1);
                    println!("  Title:   {}", res["title"].as_str().unwrap_or("N/A"));
                    println!("  Snippet: {}", res["snippet"].as_str().unwrap_or("N/A"));
                    println!("  Link:    {}", res["link"].as_str().unwrap_or("N/A"));
                }
            } else {
                println!("Result is not an array: {:?}", results);
            }
        }
        Err(e) => {
            eprintln!("❌ Search failed: {}", e);
        }
    }
}
