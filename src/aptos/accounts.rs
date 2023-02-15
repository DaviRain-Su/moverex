use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::config::Config;
use aptos_api_types::{AccountData, MoveModuleBytecode, MoveResource};

use super::{core_get, AptosClient};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetAccountRequest {
    /// Address of account with or without a 0x prefix
    ///
    /// Example: 0x88fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1
    address: String,
    /// Ledger version to get state of account
    ///
    /// If not provided, it will be the latest version
    ///
    /// Example: 32425224034
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_version: Option<u64>,
}
impl GetAccountRequest {
    pub fn new(address: String) -> GetAccountRequest {
        Self {
            address,
            ledger_version: None,
        }
    }
}
impl Display for GetAccountRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/accounts/{}", self.address)?;
        if let Some(ledger_version) = self.ledger_version {
            write!(f, "?ledger_version={}", ledger_version)?;
        }
        write!(f, "")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetAccountResourcesRequest {
    /// Address of account with or without a 0x prefix
    ///
    /// Example: 0x88fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1
    address: String,
    /// Ledger version to get state of account
    /// If not provided, it will be the latest version
    ///
    /// Example: 32425224034
    ledger_version: Option<u64>,
    /// Max number of account resources to retrieve
    /// If not provided, defaults to default page size.
    ///
    limit: Option<u64>,
    /// Cursor specifying where to start for pagination
    /// This cursor cannot be derived manually client-side.
    /// Instead, you must call this endpoint once without this query parameter specified,
    /// and then use the cursor returned in the X-Aptos-Cursor header in the response.
    ///
    /// Example: 0000000000000000000000000000000000000000000000000000000000000000012f0000000000000000000000000000000000000000000000000000000000000000010d7374616b696e675f70726f7879
    start: Option<String>,
}

impl GetAccountResourcesRequest {
    pub fn new(address: String) -> Self {
        Self {
            address,
            ledger_version: None,
            limit: None,
            start: None,
        }
    }
}

impl Display for GetAccountResourcesRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut url = format!("/accounts/{}/resources", self.address);

        if let Some(ledger_version) = self.ledger_version {
            url.push_str(&format!("?ledger_version={}", ledger_version));
        }

        if let Some(limit) = self.limit {
            if url.contains("?") {
                url.push_str(&format!("&limit={}", limit));
            } else {
                url.push_str(&format!("?limit={}", limit));
            }
        }

        if let Some(start) = &self.start {
            if url.contains("?") {
                url.push_str(&format!("&start={}", start));
            } else {
                url.push_str(&format!("?start={}", start));
            }
        }

        write!(f, "{}", url)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetAccountModulesRequest {
    /// Address of account with or without a 0x prefix
    ///
    /// Example: 0x88fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1
    address: String,
    /// Ledger version to get state of account
    /// If not provided, it will be the latest version
    ///
    /// Example: 32425224034
    ledger_version: Option<u64>,
    /// Max number of account resources to retrieve
    /// If not provided, defaults to default page size.
    ///
    limit: Option<u64>,
    /// Cursor specifying where to start for pagination
    /// This cursor cannot be derived manually client-side.
    /// Instead, you must call this endpoint once without this query parameter specified,
    /// and then use the cursor returned in the X-Aptos-Cursor header in the response.
    ///
    /// Example: 0000000000000000000000000000000000000000000000000000000000000000012f0000000000000000000000000000000000000000000000000000000000000000010d7374616b696e675f70726f7879
    start: Option<String>,
}

impl Display for GetAccountModulesRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut url = format!("/accounts/{}/modules", self.address);

        if let Some(ledger_version) = self.ledger_version {
            url.push_str(&format!("?ledger_version={}", ledger_version));
        }

        if let Some(limit) = self.limit {
            if url.contains("?") {
                url.push_str(&format!("&limit={}", limit));
            } else {
                url.push_str(&format!("?limit={}", limit));
            }
        }

        if let Some(start) = &self.start {
            if url.contains("?") {
                url.push_str(&format!("&start={}", start));
            } else {
                url.push_str(&format!("?start={}", start));
            }
        }

        write!(f, "{}", url)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetAccountResourceRequest {
    /// Address of account with or without a 0x prefix
    /// Example: 0x88fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1
    address: String,
    /// Name of struct to retrieve e.g. 0x1::account::Account
    ///
    /// Example: 0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>
    /// 
    /// Match pattern: ^0x[0-9a-zA-Z:_<>]+$
    resource_type: String,
    /// Ledger version to get state of account
    ///
    /// If not provided, it will be the latest version
    ///
    /// Example: 32425224034
    ledger_version: Option<u64>,
}

impl Display for GetAccountResourceRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "/accounts/{}/resource/{}",
            self.address, self.resource_type
        )?;
        if let Some(ledger_version) = self.ledger_version {
            write!(f, "?ledger_version={}", ledger_version)?;
        }
        write!(f, "")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetAccountModuleRequest {
    /// Address of account with or without a 0x prefix
    /// Example: 0x88fbd33f54e1126269769780feb24480428179f552e2313fbe571b72e62a1ca1
    address: String,
    /// Name of module to retrieve e.g. coin
    module_name: String,
    /// Ledger version to get state of account
    ///
    /// If not provided, it will be the latest version
    ///
    /// Example: 32425224034
    ledger_version: Option<u64>,
}

impl Display for GetAccountModuleRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/accounts/{}/module/{}", self.address, self.module_name)?;
        if let Some(ledger_version) = self.ledger_version {
            write!(f, "?ledger_version={}", ledger_version)?;
        }
        write!(f, "")
    }
}

impl AptosClient {
    /// Return the authentication key and the sequence number for an account address.
    /// Optionally, a ledger version can be specified.
    ///
    /// If the ledger version is not specified in the request, the latest ledger version is used.
    pub async fn get_account(
        &self,
        request: GetAccountRequest,
        config: Config,
    ) -> anyhow::Result<AccountData> {
        core_get(request, &config.rpc_endpoint).await
    }

    /// Retrieves all account resources for a given account and a specific ledger version. If the ledger version is not specified in the request, the latest ledger version is used.
    /// The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.
    pub async fn get_account_resources(
        &self,
        request: GetAccountResourcesRequest,
        config: Config,
    ) -> anyhow::Result<Vec<MoveResource>> {
        core_get::<GetAccountResourcesRequest, Vec<MoveResource>>(request, &config.rpc_endpoint)
            .await
    }

    /// Retrieves all account modules' bytecode for a given account at a specific ledger version. If the ledger version is not specified in the request, the latest ledger version is used.
    ///
    /// The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.
    pub async fn get_account_modules(
        &self,
        request: GetAccountModulesRequest,
        config: Config,
    ) -> anyhow::Result<Vec<MoveModuleBytecode>> {
        core_get::<GetAccountModulesRequest, Vec<MoveModuleBytecode>>(request, &config.rpc_endpoint)
            .await
    }

    /// Retrieves an individual resource from a given account and at a specific ledger version. If the ledger version is not specified in the request, the latest ledger version is used.
    ///
    /// The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.
    pub async fn get_account_resource(
        &self,
        request: GetAccountResourceRequest,
        config: Config,
    ) -> anyhow::Result<MoveResource> {
        core_get::<GetAccountResourceRequest, MoveResource>(request, &config.rpc_endpoint).await
    }

    /// Retrieves an individual module from a given account and at a specific ledger version. If the ledger version is not specified in the request, the latest ledger version is used.
    ///
    /// The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.
    pub async fn get_account_module(
        &self,
        request: GetAccountModuleRequest,
        config: Config,
    ) -> anyhow::Result<MoveModuleBytecode> {
        core_get::<GetAccountModuleRequest, MoveModuleBytecode>(request, &config.rpc_endpoint).await
    }
}

#[tokio::test]
async fn test_get_account() {
    let config = Config::default_config();
    let request = GetAccountRequest {
        address: "0xaf8d8689e3d980e46bab36a7a0e99f1bd23618ab3c7b4109d81fed57671c837d".to_string(),
        ledger_version: Some(87798277),
    };

    let aptos_client = AptosClient;

    let result = aptos_client.get_account(request, config).await;

    println!("result: {:?}", result);
}

#[tokio::test]
async fn test_get_account_resources() {
    let config = Config::default_config();
    let request = GetAccountResourcesRequest {
        address: "0xaf8d8689e3d980e46bab36a7a0e99f1bd23618ab3c7b4109d81fed57671c837d".to_string(),
        ledger_version: Some(87798277),
        limit: None,
        start: None,
    };

    let aptos_client = AptosClient;

    let result = aptos_client.get_account_resources(request, config).await;

    println!("result: {:#?}", result);
}

#[tokio::test]
async fn test_get_account_modules() {
    let config = Config::default_config();
    let request = GetAccountModulesRequest {
        address: "0xaf8d8689e3d980e46bab36a7a0e99f1bd23618ab3c7b4109d81fed57671c837d".to_string(),
        ledger_version: None,
        limit: None,
        start: None,
    };

    let aptos_client = AptosClient;

    let result = aptos_client.get_account_modules(request, config).await;

    println!("result: {:#?}", result);
}

#[tokio::test]
async fn test_get_account_source() {
    let config = Config::default_config();
    let request = GetAccountResourceRequest {
        address: "0xaf8d8689e3d980e46bab36a7a0e99f1bd23618ab3c7b4109d81fed57671c837d".to_string(),
        resource_type: "0x1::coin::CoinStore<0x1::aptos_coin::AptosCoin>".to_string(),
        ledger_version: None,
    };

    let aptos_client = AptosClient;

    let result = aptos_client.get_account_resource(request, config).await;

    println!("result: {:#?}", result);
}
