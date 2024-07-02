use sqlx::postgres::PgPoolOptions;
use testcontainers_modules::{
    postgres::Postgres,
    clickhouse::ClickHouse,
    testcontainers::runners::AsyncRunner
};

#[tokio::test]
async fn postgres() -> Result<(), Box<dyn std::error::Error>> {
    let postgres_image = Postgres::default();
    let container = postgres_image.start().await?;

    let connection_string = &format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        container.get_host().await?,
        container.get_host_port_ipv4(5432).await?
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string).await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);


    let ch_image = ClickHouse::default();
    let ch_container = ch_image.start().await?;

    println!("did i?");
    Ok(())
}