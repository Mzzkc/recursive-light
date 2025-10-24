//! Test utilities for setting up in-memory database and test fixtures

use sqlx::SqlitePool;

/// Creates an in-memory SQLite database with all migrations applied
pub async fn setup_test_db() -> Result<SqlitePool, sqlx::Error> {
    // Use in-memory database
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    // Run all migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_setup_test_db() {
        let pool = setup_test_db().await.unwrap();

        // Verify we can query the database
        let result: (i64,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await.unwrap();

        assert_eq!(result.0, 1);
    }
}
