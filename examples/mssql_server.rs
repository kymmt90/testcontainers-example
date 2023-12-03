use testcontainers::clients;
use testcontainers_modules::mssql_server;

fn main() {
    // Docker CLIへのインタフェースを作成する
    let docker = clients::Cli::default();

    // デフォルト設定でSQL Serverのコンテナを起動する
    let mssql_server = docker.run(mssql_server::MssqlServer::default());

    // 起動したコンテナの情報を表示する
    println!("Container ID: {}", mssql_server.id());

    let mssql_default_port = 1433;
    println!(
        "Host port: {}",
        mssql_server.get_host_port_ipv4(mssql_default_port)
    );
}
