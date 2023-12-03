use std::sync::OnceLock;
use testcontainers::{clients, Container};
use testcontainers_modules::mssql_server::MssqlServer;

pub static MSSQL_PORT: u16 = 1433;

static DOCKER: OnceLock<clients::Cli> = OnceLock::new();

pub type MssqlClient = tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>;

pub async fn setup_mssql_container<'d>() -> anyhow::Result<Container<'d, MssqlServer>> {
    let docker = DOCKER.get_or_init(clients::Cli::default);
    let container = docker.run(MssqlServer::default());

    create_database(&container).await?;
    load_schema(&container).await?;
    load_master_data(&container).await?;

    Ok(container)
}

pub async fn connect_to_mssql_container(config: tiberius::Config) -> anyhow::Result<MssqlClient> {
    use tokio_util::compat::TokioAsyncWriteCompatExt as _;

    let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    Ok(tiberius::Client::connect(config, tcp.compat_write()).await?)
}

pub fn mssql_config() -> tiberius::Config {
    let mut config = tiberius::Config::new();
    config.authentication(tiberius::AuthMethod::sql_server(
        "sa",
        "yourStrong(!)Password",
    ));
    config.trust_cert();

    config
}

async fn create_database(container: &Container<'_, MssqlServer>) -> anyhow::Result<()> {
    let mut config = mssql_config();
    config.port(container.get_host_port_ipv4(MSSQL_PORT));
    let mut client = connect_to_mssql_container(config).await.unwrap();

    client.simple_query("create database test").await?;

    Ok(())
}

async fn load_schema(container: &Container<'_, MssqlServer>) -> anyhow::Result<()> {
    let mut config = mssql_config();
    config.port(container.get_host_port_ipv4(MSSQL_PORT));
    config.database("test");
    let mut client = connect_to_mssql_container(config).await.unwrap();

    let ddl = include_str!("schema.sql");
    client.simple_query(ddl).await?.into_results().await?;

    Ok(())
}

async fn load_master_data(container: &Container<'_, MssqlServer>) -> anyhow::Result<()> {
    let mut config = mssql_config();
    config.port(container.get_host_port_ipv4(MSSQL_PORT));
    config.database("test");
    let mut client = connect_to_mssql_container(config).await.unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(include_str!("products.csv").as_bytes());

    for record in reader.records() {
        let r = record?;
        let name = r.get(0).unwrap().to_string();
        let price: i32 = r.get(1).unwrap().parse().unwrap();

        client
            .query(
                "INSERT INTO products (name, price) VALUES (@P1, @P2)",
                &[&name, &price],
            )
            .await?;
    }

    Ok(())
}
