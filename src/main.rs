use once_cell::sync::Lazy;
use std::env;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:localhost,1433;Database=OnlineApp;User ID=sa;Password=mssql1Ipw;TrustServerCertificate=true".to_owned()
    })
});

#[derive(Debug)]
struct Agent {
    first_name: String,
    middle_name: String,
    last_name: String,
}

#[cfg(not(all(windows, feature = "sql-browser-tokio")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_ado_string(&CONN_STR)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let rows = client
        .query("SELECT LastName, FirstName, MiddleName from Agents", &[])
        .await?
        .into_first_result()
        .await?;
    let mut agents = Vec::new();

    for row in rows {
        agents.push(Agent {
            first_name: row
                // example of how to use the `::<_>` to specify the type we want to get out of the column.
                .try_get::<&str, _>("FirstName")?
                .ok_or_else(|| anyhow::anyhow!("Unexpected null"))?
                .to_string(),
            middle_name: row
                // example of how to use the `::<_>` to specify the type we want to get out of the column.
                .try_get::<&str, _>("MiddleName")?
                .ok_or_else(|| anyhow::anyhow!("Unexpected null"))?
                .to_string(),
            last_name: row
                // example of how to use the `::<_>` to specify the type we want to get out of the column.
                .try_get::<&str, _>("LastName")?
                .ok_or_else(|| anyhow::anyhow!("Unexpected null"))?
                .to_string(),
        })
    }

    println!("{:?}", agents);
    Ok(())
}
