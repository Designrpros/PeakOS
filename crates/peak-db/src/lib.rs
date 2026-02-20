//! PeakDB: The Model Layer of PeakOS
//! 
//! PeakDB provides a high-level, reactive, and graph-native data access layer
//! built on top of SQLx.

use async_trait::async_trait;

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

#[async_trait]
pub trait Model {
    fn table_name() -> &'static str;
}

pub struct PeakDB {
    pub pool: SqlitePool,
}

impl PeakDB {
    pub async fn connect(url: &str) -> Result<Self, String> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .map_err(|e| e.to_string())?;

        // Initialize schema
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS semantic_records (
                id TEXT PRIMARY KEY,
                collection TEXT,
                content TEXT,
                vector BLOB,
                metadata TEXT,
                timestamp INTEGER
            )"
        )
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Self { pool })
    }

    pub async fn save_record(
        &self,
        id: &str,
        collection: &str,
        content: &str,
        vector: Option<&[f32]>,
        metadata: &str,
        timestamp: u64,
    ) -> Result<(), String> {
        let vector_blob = vector.map(|v| {
            let mut bytes = Vec::with_capacity(v.len() * 4);
            for f in v {
                bytes.extend_from_slice(&f.to_le_bytes());
            }
            bytes
        });

        sqlx::query(
            "INSERT OR REPLACE INTO semantic_records (id, collection, content, vector, metadata, timestamp)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(collection)
        .bind(content)
        .bind(vector_blob)
        .bind(metadata)
        .bind(timestamp as i64)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn load_all_records(&self) -> Result<Vec<(String, String, String, Option<Vec<f32>>, String, u64)>, String> {
        let rows = sqlx::query("SELECT id, collection, content, vector, metadata, timestamp FROM semantic_records")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for row in rows {
            use sqlx::Row;
            let id: String = row.get(0);
            let collection: String = row.get(1);
            let content: String = row.get(2);
            let vector_blob: Option<Vec<u8>> = row.get(3);
            let metadata: String = row.get(4);
            let timestamp: i64 = row.get(5);

            let vector = vector_blob.map(|b| {
                b.chunks_exact(4)
                    .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
                    .collect()
            });

            results.push((id, collection, content, vector, metadata, timestamp as u64));
        }
        Ok(results)
    }
}
