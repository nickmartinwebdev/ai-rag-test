use meilisearch_sdk::client::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Document {
    id: String,
    title: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AI RAG Test - Meilisearch Integration");

    // Get Meilisearch configuration from environment variables
    let meilisearch_url =
        env::var("MEILISEARCH_URL").unwrap_or_else(|_| "http://localhost:7700".to_string());
    let api_key = env::var("MEILISEARCH_API_KEY")
        .unwrap_or_else(|_| "your-master-key-change-this-in-production".to_string());

    // Create Meilisearch client
    let client = Client::new(&meilisearch_url, Some(&api_key))?;

    // Test connection
    match client.health().await {
        Ok(_) => println!("✅ Connected to Meilisearch at {}", meilisearch_url),
        Err(e) => {
            eprintln!("❌ Failed to connect to Meilisearch: {}", e);
            return Err(e.into());
        }
    }

    // Create or get index
    let index_name = "documents";
    let index = client.index(index_name);

    // Check if index exists, create if not
    match client.get_index(index_name).await {
        Ok(_) => println!("📚 Using existing index: {}", index_name),
        Err(_) => {
            println!("🔨 Creating new index: {}", index_name);
            match client.create_index(index_name, Some("id")).await {
                Ok(_) => println!("✅ Index created successfully"),
                Err(e) => {
                    eprintln!("❌ Failed to create index: {}", e);
                    return Err(e.into());
                }
            }
        }
    }

    // Sample documents for testing
    let documents = vec![
        Document {
            id: "1".to_string(),
            title: "Introduction to RAG".to_string(),
            content: "Retrieval-Augmented Generation (RAG) is a technique that combines information retrieval with text generation to create more accurate and contextual responses.".to_string(),
        },
        Document {
            id: "2".to_string(),
            title: "Meilisearch Overview".to_string(),
            content: "Meilisearch is a powerful, fast, open-source search engine that delivers flexible search and discovery experiences.".to_string(),
        },
        Document {
            id: "3".to_string(),
            title: "Rust Programming".to_string(),
            content: "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.".to_string(),
        },
    ];

    // Index documents
    println!("📝 Indexing sample documents...");
    match index.add_documents(&documents, Some("id")).await {
        Ok(task) => {
            println!("⏳ Indexing documents...");

            // Wait for indexing to complete with timeout
            match client.wait_for_task(task, None, None).await {
                Ok(_) => {
                    println!("✅ Documents indexed successfully");
                }
                Err(e) => {
                    eprintln!("❌ Indexing task failed: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to start indexing: {}", e);
        }
    }

    // Add a small delay to ensure indexing is complete
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    // Perform a search
    println!("\n🔍 Searching for 'RAG'...");
    match index
        .search()
        .with_query("RAG")
        .with_limit(5)
        .execute::<Document>()
        .await
    {
        Ok(search_results) => {
            println!("📊 Found {} results:", search_results.hits.len());
            for (i, hit) in search_results.hits.iter().enumerate() {
                println!("  {}. {} (ID: {})", i + 1, hit.result.title, hit.result.id);
                println!("     Content: {}", hit.result.content);
            }
        }
        Err(e) => {
            eprintln!("❌ Search failed: {}", e);
        }
    }

    // Perform another search
    println!("\n🔍 Searching for 'fast'...");
    match index
        .search()
        .with_query("fast")
        .with_limit(5)
        .execute::<Document>()
        .await
    {
        Ok(search_results) => {
            println!("📊 Found {} results:", search_results.hits.len());
            for (i, hit) in search_results.hits.iter().enumerate() {
                println!("  {}. {} (ID: {})", i + 1, hit.result.title, hit.result.id);
                println!("     Content: {}", hit.result.content);
            }
        }
        Err(e) => {
            eprintln!("❌ Search failed: {}", e);
        }
    }

    // List all documents in the index
    println!("\n📋 Listing all documents in index...");
    match index.search().execute::<Document>().await {
        Ok(all_results) => {
            println!("📊 Total documents in index: {}", all_results.hits.len());
            for (i, hit) in all_results.hits.iter().enumerate() {
                println!("  {}. {} (ID: {})", i + 1, hit.result.title, hit.result.id);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list documents: {}", e);
        }
    }

    println!("\n🎉 Meilisearch integration test completed!");
    Ok(())
}
