// use ethers::providers::Provider;
// use ethers::types::U256;
use structopt::StructOpt;
use ethers_providers::{Middleware, Provider, Http};
use std::convert::TryFrom;

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
    GetBlock {
        /// this is block number
        block_number: u64,
    }

    // add more subcommands here
}

#[derive(Debug, StructOpt)]
#[structopt(name = "eth-cli", about = "Interact with Ethereum network")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::default_config();

    let opt = Opt::from_args();
    match opt.command {
        Command::GetBalance { address } => {
            println!("Getting balance of address: {}", address);
            // code to get balance
        }
        Command::GetBlock{ block_number } => {
            println!("ether net type is : {}", config.net_type);
            // add more commands here
            let provider = Provider::<Http>::try_from(
                config.rpc_endpoint
            ).expect("could not instantiate HTTP Provider");

            let block = provider.get_block(block_number).await?;
            println!("Got block: {}", serde_json::to_string_pretty(&block)?);
        }
    }

    Ok(())
}
