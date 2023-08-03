use clap::{Parser, ValueEnum};

pub mod client;
pub mod server;
pub mod utils;

#[derive(Parser)]
#[command(name = "Async Client/Server Networking")]
#[command(author = "KNTH")]
#[command(version = "0.1.0")]
#[command(about = "Starts TCP clients and servers, sync or async.", long_about = None)]
pub struct Cli {
    /// What mode to run the program in
    #[arg(value_enum)]
    mode: Mode,

    /// sync or async
    #[arg(value_enum)]
    sync_mode: SyncMode,

    /// Network port to use
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,

    /// Sleep the thread
    #[arg(short, long, value_parser = clap::value_parser!(u64).range(1..), default_value_t = 2000)]
    delay: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Spin up a server
    Server,
    /// Connect to a server
    Client,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SyncMode {
    Sync,
    Async,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Server => match cli.sync_mode {
            SyncMode::Sync => server::mode_sync::start(&cli),
            SyncMode::Async => server::mode_async::start(&cli).await,
        },
        Mode::Client => {
            println!("Client");
        }
    }
}
