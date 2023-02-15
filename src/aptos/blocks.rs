use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::config::Config;

use aptos_api_types::Block;

use super::{core_get, AptosClient};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlocksByHeight {
    block_height: u64,
    /// If set to true, include all transactions in the block
    ///
    ///If not provided, no transactions will be retrieved
    with_transactions: Option<bool>,
}

impl Display for BlocksByHeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/blocks/by_height/{}", self.block_height)?;
        if let Some(wiwith_transactions) = self.with_transactions {
            write!(f, "?with_transactions={}", wiwith_transactions)?;
        }
        write!(f, "")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlocksByVersion {
    /// Ledger version to lookup block information for.
    version: u64,
    /// If set to true, include all transactions in the block
    ///
    ///If not provided, no transactions will be retrieved
    with_transactions: Option<bool>,
}

impl Display for BlocksByVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/blocks/by_version/{}", self.version)?;
        if let Some(wiwith_transactions) = self.with_transactions {
            write!(f, "?with_transactions={}", wiwith_transactions)?;
        }
        write!(f, "")
    }
}

impl AptosClient {
    /// This endpoint allows you to get the transactions in a block and the corresponding block information.
    ///
    /// Transactions are limited by max default transactions size. If not all transactions are present, the user will need to query for the rest of the transactions via the get transactions API.
    ///
    /// If the block is pruned, it will return a 410
    pub async fn get_blocks_by_height(
        &self,
        request: BlocksByHeight,
        config: Config,
    ) -> anyhow::Result<Block> {
        core_get::<BlocksByHeight, Block>(request, &config.rpc_endpoint).await
    }

    ///This endpoint allows you to get the transactions in a block and the corresponding block information given a version in the block.
    ///
    /// Transactions are limited by max default transactions size. If not all transactions are present, the user will need to query for the rest of the transactions via the get transactions API.
    ///
    /// If the block has been pruned, it will return a 410
    pub async fn get_blocks_by_version(
        &self,
        request: BlocksByVersion,
        config: Config,
    ) -> anyhow::Result<Block> {
        core_get::<BlocksByVersion, Block>(request, &config.rpc_endpoint).await
    }
}

#[tokio::test]
async fn test_get_blocks_by_height() {
    let config = Config::default_config();
    let request = BlocksByHeight {
        block_height: 1000,
        with_transactions: None,
    };

    let aptos_client = AptosClient;

    let result = aptos_client.get_blocks_by_height(request, config).await;

    println!("result: {:#?}", result);
}

#[tokio::test]
async fn test_get_blocks_by_version() {
    let config = Config::default_config();
    let request = BlocksByVersion {
        version: 1,
        with_transactions: None,
    };

    let aptos_client = AptosClient;

    let result = aptos_client.get_blocks_by_version(request, config).await;

    println!("result: {:#?}", result);
}
