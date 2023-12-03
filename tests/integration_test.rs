mod common;

use testcontainers_example::Product;

#[tokio::test]
async fn count_total_records() -> anyhow::Result<()> {
    let container = common::setup_mssql_container().await?;
    let mut client = get_mssql_client(container.get_host_port_ipv4(common::MSSQL_PORT)).await?;

    let r = client
        .simple_query("SELECT count(*) AS count FROM products")
        .await?
        .into_row()
        .await?
        .unwrap();

    assert_eq!(r.get("count"), Some(10));

    Ok(())
}

#[tokio::test]
async fn query_product() -> anyhow::Result<()> {
    let container = common::setup_mssql_container().await?;
    let mut client = get_mssql_client(container.get_host_port_ipv4(common::MSSQL_PORT)).await?;

    let id = 1;
    let r = client
        .query("SELECT * FROM products WHERE id = @P1", &[&id])
        .await?
        .into_row()
        .await?
        .unwrap();

    let product = Product::deserialize(&r)?;
    assert_eq!(product.to_string(), "1: product 1 - 1000円");

    Ok(())
}

async fn get_mssql_client(port: u16) -> anyhow::Result<common::MssqlClient> {
    let mut config = common::mssql_config();
    config.database("test");
    config.port(port);

    common::connect_to_mssql_container(config).await
}
