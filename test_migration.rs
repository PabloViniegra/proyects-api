use proyects_api::state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new("sqlite:test_indexes.db?mode=rwc")
        .await
        .expect("Failed to create state");
    
    // Query to list all indexes
    let indexes: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%' ORDER BY name"
    )
    .fetch_all(&state.db)
    .await
    .expect("Failed to query indexes");
    
    println!("✅ Migration successful! Created {} indexes:", indexes.len());
    for (name,) in indexes {
        println!("  - {}", name);
    }
}
