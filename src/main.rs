use structopt::StructOpt;

pub mod aptos;
pub mod config;
use config::Config;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "get_balance")]
    /// Get the balance of an Ethereum address
    GetBalance {
        #[structopt(name = "ADDRESS")]
        /// The Ethereum address to check the balance of
        address: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "moverex", about = "Interact with Move network")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _config = Config::default_config();

    let opt = Opt::from_args();
    match opt.command {
        Command::GetBalance { address } => {
            println!("Getting balance of address: {}", address);
        }
    }

    Ok(())
}
