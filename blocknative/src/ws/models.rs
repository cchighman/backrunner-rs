use std::{collections::HashMap, fmt};

use chrono::Utc;
// Code adapted from: https://github.com/althea-net/guac_rs/tree/master/web3/src/jsonrpc
// use ethers_core::types::U256;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::models::Blockchain;

#[derive(Serialize, Deserialize, Debug, Clone, Error)]
/// A JSON-RPC 2.0 error
pub struct JsonRpcError {
    /// The error code
    pub code: i64,
    /// The error message
    pub message: String,
    /// Additional data
    pub data: Option<Value>,
}

impl fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(code: {}, message: {}, data: {:?})",
            self.code, self.message, self.data
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    time_stamp: String,
    dapp_id: String,
    version: String,
    blockchain: Blockchain,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxDescriptor {
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountDescriptor {
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionSubscribe {
    transaction: TxDescriptor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountSubscribe {
    account: AccountDescriptor,
}

impl TransactionSubscribe {
    pub fn new(hash: String) -> Self {
        Self {
            transaction: TxDescriptor { hash },
        }
    }
}

impl AccountSubscribe {
    pub fn account(address: String) -> Self {
        Self {
            account: AccountDescriptor { address },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// A JSON-RPC request
#[serde(rename_all = "camelCase")]
pub struct Request<'a, T> {
    #[serde(rename = "timeStamp")]
    timestamp: String,
    dapp_id: &'a str,
    blockchain: Blockchain,
    version: &'a str,
    category_code: String,
    event_code: String,
    #[serde(flatten)]
    params: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscription<R> {
    pub subscription: u64,
    pub result: R,
}

impl<'a, T> Request<'a, T> {
    // Creates a new JSON RPC request
    pub fn new(
        dapp_id: &'a str,
        blockchain: Blockchain,
        method: &'a str,
        event_code: &'a str,
        params: T,
    ) -> Self {
        Self {
            timestamp: Utc::now().to_string(),
            dapp_id,
            blockchain,
            version: "2",
            category_code: method.to_string(),
            event_code: event_code.to_string(),
            params,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GasInfo {
    #[serde(rename_all = "camelCase")]
    ERC1559 {
        base_fee_per_gas: Option<String>,
        max_fee_per_gas: String,
        // max_fee_per_gas_gwei: f64,
        max_priority_fee_per_gas: String,
        // max_priority_fee_per_gas_gwei: u64,
    },
    #[serde(rename_all = "camelCase")]
    Legacy {
        gas_price: String,
        // gas_price_gwei: f64,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub status: String,
    pub monitor_id: String,
    pub monitor_version: String,
    #[serde(flatten)]
    pub confirmed: Option<ConfirmedInfo>,
    pub pending: Option<PendingInfo>,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: u64,
    pub nonce: u64,
    pub v: String,
    pub r: String,
    pub s: String,
    pub input: String,
    #[serde(flatten)]
    pub gas_info: GasInfo,
    #[serde(rename = "type")]
    pub type_field: Option<i64>,
    pub asset: String,
    #[serde(flatten)]
    pub watch_info: Option<WatchedAddressInfo>,
}

#[cfg(feature = "ethers")]
impl From<Transaction> for ethers::prelude::Transaction {
    fn from(val: Transaction) -> Self {
        ethers::prelude::Transaction {
            from: val.from.parse().unwrap(),
            to: Some(val.to.parse().unwrap()),
            gas: val.gas.into(),
            gas_price: Some(val.gas_price.parse().unwrap()),
            value: val.value.parse().unwrap(),
            nonce: val.nonce.into(),
            block_hash: None,
            block_number: None,
            transaction_index: None,
            input: hex::decode(val.input.strip_prefix("0x").unwrap())
                .unwrap()
                .into(),
            v: val.v.parse().unwrap(),
            r: val.r.parse().unwrap(),
            s: val.s.parse().unwrap(),
            transaction_type: val.type_field.map(|n| n.into()),
            access_list: None,
            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
            chain_id: None,
            hash: val.hash.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingInfo {
    pub pending_time_stamp: String,
    pub pending_block_number: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmedInfo {
    pub time_pending: String,
    pub blocks_pending: i64,
    pub block_hash: String,
    pub block_number: i64,
    pub transaction_index: i64,
    pub block_time_stamp: String,
    pub gas_used: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchedAddressInfo {
    pub watched_address: String,
    pub direction: String,
    pub counterparty: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractCall {
    pub contract_type: String,
    pub contract_address: String,
    pub method_name: String,
    pub params: Value,
    pub contract_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub time_stamp: String,
    pub category_code: String,
    pub event_code: String,
    pub dapp_id: String,
    pub blockchain: Blockchain,
    pub contract_call: Option<ContractCall>,
    pub transaction: Option<Transaction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub version: u64,
    pub server_version: String,
    pub time_stamp: String,
    pub connection_id: String,
    pub status: String,
    pub raw: Option<String>,
    pub event: Option<Event>,
    pub reason: Option<String>,
    pub dispatch_timestamp: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelloMsg {
    pub version: i64,
    pub server_version: String,
    pub status: String,
    #[serde(rename = "showUX")]
    pub show_ux: bool,
    pub connection_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchConfig {
    pub scope: String,
    pub filters: Vec<HashMap<String, String>>,
    pub watch_address: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WatchRequest {
    pub config: WatchConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let json = r#"{"version":0,"serverVersion":"0.123.2","timeStamp":"2021-12-07T10:20:25.212Z","connectionId":"C4-bc4de41f-c42f-460a-af83-28ad95286ab0","status":"ok","event":{"timeStamp":"2021-12-07T10:20:25.212Z","categoryCode":"activeAddress","eventCode":"txConfirmed","dappId":"7d507b2c-48f2-48bb-bd79-fc16ced6f8cf","blockchain":{"system":"ethereum","network":"matic-main"},"contractCall":{"contractType":"Uniswap V2: Router 2","contractAddress":"0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff","methodName":"swapExactTokensForTokens","params":{"amountIn":"5000000000","amountOutMin":"180189367","path":["0xC250e9987A032ACAC293d838726C511E6E1C029d","0xa3Fa99A148fA48D14Ed51d610c367C61876997F1","0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174","0xc2132D05D31c914a87C6611C10748AEb04B58e8F"],"to":"0x21F3bB63e775ccDf0CC04559Be142971D241aB0E","deadline":"3277746025"},"contractName":"QuickSwap: Router"},"transaction":{"status":"confirmed","monitorId":"Geth_137_C_PROD","monitorVersion":"0.102.0","timePending":"3146","blocksPending":3,"pendingTimeStamp":"2021-12-07T10:20:22.066Z","pendingBlockNumber":22235980,"hash":"0xe0b1cf2bea578f49ba78cacd0d12d9c013f07cdd987936e71965edf6bd972b78","from":"0x21F3bB63e775ccDf0CC04559Be142971D241aB0E","to":"0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff","value":"0","gas":387473,"nonce":45,"blockHash":"0xa814777d863e89c2b565ad4947e37e48bc5d8407b4065303c6371de519980d89","blockNumber":22235983,"v":"0x136","r":"0xb1fa90713d69a05869823607cc4bc67de6c7d4599b9fe8b00c54d8bc902739f9","s":"0x297a6aba5a47be29475d037b41619ad4003048e82305f20a3b18927cbfe2a343","input":"0x38ed1739000000000000000000000000000000000000000000000000000000012a05f200000000000000000000000000000000000000000000000000000000000abd78b700000000000000000000000000000000000000000000000000000000000000a000000000000000000000000021f3bb63e775ccdf0cc04559be142971d241ab0e00000000000000000000000000000000000000000000000000000000c35e6f690000000000000000000000000000000000000000000000000000000000000004000000000000000000000000c250e9987a032acac293d838726c511e6e1c029d000000000000000000000000a3fa99a148fa48d14ed51d610c367c61876997f10000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa84174000000000000000000000000c2132d05d31c914a87c6611c10748aeb04b58e8f","gasPrice":"113000000000","gasPriceGwei":113,"gasUsed":"236672","transactionIndex":1,"asset":"","blockTimeStamp":"2021-12-07T10:20:25.000Z","watchedAddress":"0xa5e0829caced8ffdd4de3c43696c57f7d7a678ff","direction":"incoming","counterparty":"0x21F3bB63e775ccDf0CC04559Be142971D241aB0E"}},"dispatchTimestamp":"2021-12-07T10:20:25.247Z"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_decode_2() {
        let json = r#"{"version":0,"serverVersion":"0.127.0","timeStamp":"2022-02-05T00:14:59.330Z","connectionId":"c3-88afabe3-1df7-436d-b91e-15f485528494","status":"ok","event":{"timeStamp":"2022-02-05T00:14:59.330Z","categoryCode":"activeAddress","eventCode":"txPoolSimulation","dappId":"7d507b2c-48f2-48bb-bd79-fc16ced6f8cf","blockchain":{"system":"ethereum","network":"main"},"transaction":{"status":"pending-simulation","monitorId":"Geth_1_D_PROD","monitorVersion":"0.108.0","pendingTimeStamp":"2022-02-05T00:14:59.317Z","pendingBlockNumber":14142762,"hash":"0xebc639e7f6bd7a3c3d8ad5a58f805fa8e024c0308ee784179a7fb8716859e095","from":"0x38Ab1C0e1c3a185594792F7FD7212Eeb563F044C","to":"0x9011F2133A705Fe72226647B5B246086C6b72140","value":"0","gas":461111,"nonce":53,"blockHash":null,"blockNumber":null,"v":"0x0","r":"0xfc3419eb5467401484773f1449af01e606e2a49db885e189b72ddcee4f100969","s":"0x2f5106ee50c3f13169d9b13b8cae0faaae116a640e17c169a187451e731a0015","input":"0x4585e33b00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000060000000000000000000000000a4492fcda2520cb68657d220f4d4ae3116359c1004000000000000000000000061fdbe00000000000000113000000000000000000000000000000000000000000000000000000000000000000de0b6b3a7640000","type":2,"maxFeePerGas":"200638563976","maxFeePerGasGwei":201,"maxPriorityFeePerGas":"2500000000","maxPriorityFeePerGasGwei":2.5,"asset":"ETH","watchedAddress":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","direction":"","counterparty":"","internalTransactions":[{"type":"CALL","from":"0x9011f2133a705fe72226647b5b246086c6b72140","to":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","input":"0xb50e7ee304000000000000000000000061fdbe00000000000000113000000000000000000000000000000000000000000000000000000000000000000de0b6b3a7640000","gas":428734,"gasUsed":422208,"value":"0"},{"type":"DELEGATECALL","from":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","to":"0x09fde18700c82a8f3134a5c01dc58f6cb2396a40","input":"0xb50e7ee304000000000000000000000061fdbe00000000000000113000000000000000000000000000000000000000000000000000000000000000000de0b6b3a7640000","gas":413552,"gasUsed":413552,"value":"0"},{"type":"DELEGATECALL","from":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","to":"0x0f6e8ef18fb5bb61d545fee60f779d8aed60408f","input":"0xe101a89b000000000000000000000000000000000000000000000000ce29650ccf5b657000000000000000000000000000000000000000000007d32e5806f7ba38d4987e00000000000000000000000000000000000000000007d3925806f7ba38d4987e0000000000000000000000000000000000000000000000008000000000000000","gas":313699,"gasUsed":4910,"value":"0"},{"type":"CALL","from":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","to":"0x9abb27581c2e46a114f8c367355851e0580e9703","input":"0xedaf7d5b000000000000000000000000a73342309f77e7dd2ebb5893a651bec7c472aa6a000000000000000000000000a4492fcda2520cb68657d220f4d4ae3116359c100000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000176c3cdeaf06cd0a000000000000000000000000000000000000000000000000176c3cdeaf06cd0a0000000000000000000000000000000000000000000000275a4f8834009d4b7c","gas":111344,"gasUsed":39785,"value":"0"},{"type":"DELEGATECALL","from":"0x9abb27581c2e46a114f8c367355851e0580e9703","to":"0x1b890f72b21233cb38666fb81161c4bbe15f1f5d","input":"0xedaf7d5b000000000000000000000000a73342309f77e7dd2ebb5893a651bec7c472aa6a000000000000000000000000a4492fcda2520cb68657d220f4d4ae3116359c100000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000176c3cdeaf06cd0a000000000000000000000000000000000000000000000000176c3cdeaf06cd0a0000000000000000000000000000000000000000000000275a4f8834009d4b7c","gas":104505,"gasUsed":34566,"value":"0"}],"netBalanceChanges":[],"simDetails":{"blockNumber":14142762,"performanceProfile":{"breakdown":[{"label":"detected","timeStamp":"2022-02-05T00:14:59.330Z"},{"label":"traceStart","timeStamp":"2022-02-05T00:14:59.331Z"},{"label":"traceEnd","timeStamp":"2022-02-05T00:14:59.415Z"},{"label":"dispatch","timeStamp":"2022-02-05T00:14:59.426Z"}],"e2eMs":96}}}},"dispatchTimestamp":"2022-02-05T00:14:59.427Z"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_decode_3() {
        let json = r#"{"version":0,"serverVersion":"0.127.0","timeStamp":"2022-02-05T09:21:23.485Z","connectionId":"d4-bf0707bb-a594-478a-be8d-9cbe0bf9dc37","status":"ok","event":{"timeStamp":"2022-02-05T09:21:23.485Z","categoryCode":"activeAddress","eventCode":"txConfirmed","dappId":"7d507b2c-48f2-48bb-bd79-fc16ced6f8cf","blockchain":{"system":"ethereum","network":"main"},"transaction":{"status":"confirmed","monitorId":"Geth_1_A_PROD","monitorVersion":"0.108.0","timePending":"11018","blocksPending":3,"pendingTimeStamp":"2022-02-05T09:21:12.467Z","pendingBlockNumber":14145144,"hash":"0x7e5feb278e5739f1c6a8ff7938f0db8878d22fa3fbbb6776aecef41d5f970312","from":"0xB75aDA8cc810c8B1c32ef64Cf7DFF4B21B12305A","to":"0x9aBB27581c2E46A114F8C367355851e0580e9703","value":"0","gas":187393,"nonce":88,"blockHash":"0x49270d84a554ec2ba838890dee5ecd81010fc28223a0420333c9ca34be9cd119","blockNumber":14145147,"v":"0x0","r":"0x6d60a3ebef49b0906af779c6e91008c0006000ba1b474536b4010852a4b523bd","s":"0x4f116baeede8f5f5b85e59076adb6b45dad08de020a806bd82dfac697946d939","input":"0x34ee5573000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000020000000000000000000000001b63334f7bfdf0d753ab3101eb6d02b278db8852000000000000000000000000a4492fcda2520cb68657d220f4d4ae3116359c10000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001","gasPrice":"49495778843","gasPriceGwei":49.5,"type":2,"maxFeePerGas":"54090000000","maxFeePerGasGwei":54.1,"maxPriorityFeePerGas":"1500000000","maxPriorityFeePerGasGwei":1.5,"baseFeePerGas":"47995778843","baseFeePerGasGwei":48,"asset":"ETH","watchedAddress":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","direction":"","counterparty":"","internalTransactions":[{"type":"DELEGATECALL","from":"0x9abb27581c2e46a114f8c367355851e0580e9703","to":"0x1b890f72b21233cb38666fb81161c4bbe15f1f5d","input":"0x34ee5573000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000020000000000000000000000001b63334f7bfdf0d753ab3101eb6d02b278db8852000000000000000000000000a4492fcda2520cb68657d220f4d4ae3116359c10000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001","gas":156699,"gasUsed":134762,"value":"0"},{"type":"CALL","from":"0x9abb27581c2e46a114f8c367355851e0580e9703","to":"0x1b63334f7bfdf0d753ab3101eb6d02b278db8852","input":"0x491c011a000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a0000000000000000000000000000000000000000000000000000000000000001","gas":150048,"gasUsed":84531,"value":"0"},{"type":"DELEGATECALL","from":"0x1b63334f7bfdf0d753ab3101eb6d02b278db8852","to":"0xb6f767453b41d71112a910d3b3f7e35d7ff7231f","input":"0x491c011a000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a0000000000000000000000000000000000000000000000000000000000000001","gas":139221,"gasUsed":75875,"value":"0"},{"type":"CALL","from":"0x1b63334f7bfdf0d753ab3101eb6d02b278db8852","to":"0x9abb27581c2e46a114f8c367355851e0580e9703","input":"0xaef2be36000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a0000000000000000000000001b63334f7bfdf0d753ab3101eb6d02b278db8852000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010deea223","gas":131722,"gasUsed":70416,"value":"0"},{"type":"DELEGATECALL","from":"0x9abb27581c2e46a114f8c367355851e0580e9703","to":"0x1b890f72b21233cb38666fb81161c4bbe15f1f5d","input":"0xaef2be36000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a0000000000000000000000001b63334f7bfdf0d753ab3101eb6d02b278db8852000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010deea223","gas":128994,"gasUsed":69697,"value":"0"},{"type":"CALL","from":"0x9abb27581c2e46a114f8c367355851e0580e9703","to":"0x6399c842dd2be3de30bf99bc7d1bbf6fa3650e70","input":"0xa9059cbb000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a000000000000000000000000000000000000000000000006fed8833a5589b2f7","gas":88656,"gasUsed":28023,"value":"0","contractCall":{"contractType":"erc20","contractAddress":"0x6399c842dd2be3de30bf99bc7d1bbf6fa3650e70","methodName":"transfer","params":{"_to":"0xB75aDA8cc810c8B1c32ef64Cf7DFF4B21B12305A","_value":"129044036209426936567"},"contractAlias":"PREMIA","contractDecimals":18,"contractName":"Premia","decimalValue":"129.044036209426936567"}},{"type":"CALL","from":"0x9abb27581c2e46a114f8c367355851e0580e9703","to":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","input":"0x491c011a000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a0000000000000000000000000000000000000000000000000000000000000001","gas":63435,"gasUsed":42331,"value":"0"},{"type":"DELEGATECALL","from":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","to":"0xb6f767453b41d71112a910d3b3f7e35d7ff7231f","input":"0x491c011a000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a0000000000000000000000000000000000000000000000000000000000000001","gas":60852,"gasUsed":40675,"value":"0"},{"type":"CALL","from":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","to":"0x9abb27581c2e46a114f8c367355851e0580e9703","input":"0xaef2be36000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a000000000000000000000000a4492fcda2520cb68657d220f4d4ae3116359c10000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000040894b84960b5c4300000000000000000000000000000000000000000000000040894b84960b5c430000000000000000000000000000000000000000000000275a4f8834009d4b7c","gas":54577,"gasUsed":35216,"value":"0"},{"type":"DELEGATECALL","from":"0x9abb27581c2e46a114f8c367355851e0580e9703","to":"0x1b890f72b21233cb38666fb81161c4bbe15f1f5d","input":"0xaef2be36000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a000000000000000000000000a4492fcda2520cb68657d220f4d4ae3116359c10000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000040894b84960b5c4300000000000000000000000000000000000000000000000040894b84960b5c430000000000000000000000000000000000000000000000275a4f8834009d4b7c","gas":53054,"gasUsed":34497,"value":"0"},{"type":"CALL","from":"0x9abb27581c2e46a114f8c367355851e0580e9703","to":"0x6399c842dd2be3de30bf99bc7d1bbf6fa3650e70","input":"0xa9059cbb000000000000000000000000b75ada8cc810c8b1c32ef64cf7dff4b21b12305a0000000000000000000000000000000000000000000000111a55b388ddfbae99","gas":24239,"gasUsed":3323,"value":"0","contractCall":{"contractType":"erc20","contractAddress":"0x6399c842dd2be3de30bf99bc7d1bbf6fa3650e70","methodName":"transfer","params":{"_to":"0xB75aDA8cc810c8B1c32ef64Cf7DFF4B21B12305A","_value":"315492269471490092697"},"contractAlias":"PREMIA","contractDecimals":18,"contractName":"Premia","decimalValue":"315.492269471490092697"}}],"netBalanceChanges":[{"address":"0x9abb27581c2e46a114f8c367355851e0580e9703","balanceChanges":[{"delta":"-444536305680917029264","asset":{"type":"erc20","symbol":"PREMIA","contractAddress":"0x6399c842dd2be3de30bf99bc7d1bbf6fa3650e70"},"breakdown":[{"counterparty":"0xB75aDA8cc810c8B1c32ef64Cf7DFF4B21B12305A","amount":"-444536305680917029264"}]}]},{"address":"0xB75aDA8cc810c8B1c32ef64Cf7DFF4B21B12305A","balanceChanges":[{"delta":"444536305680917029264","asset":{"type":"erc20","symbol":"PREMIA","contractAddress":"0x6399c842dd2be3de30bf99bc7d1bbf6fa3650e70"},"breakdown":[{"counterparty":"0x9abb27581c2e46a114f8c367355851e0580e9703","amount":"444536305680917029264"}]}]}]}},"dispatchTimestamp":"2022-02-05T09:21:27.104Z"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_decode_4() {
        let json = r#"{"version":0,"serverVersion":"0.127.0","timeStamp":"2022-02-05T05:47:08.506Z","connectionId":"d4-bf0707bb-a594-478a-be8d-9cbe0bf9dc37","status":"ok","event":{"timeStamp":"2022-02-05T05:47:08.506Z","categoryCode":"activeAddress","eventCode":"txPool","dappId":"7d507b2c-48f2-48bb-bd79-fc16ced6f8cf","blockchain":{"system":"ethereum","network":"main"},"contractCall":{"methodName":"purchase","params":{"maturity":"1645171200","strike64x64":"55340232221128654848000","contractSize":"100000000000000000","isCall":false,"maxCost":"12399888696412000000"},"contractAddress":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","contractType":"customAbi"},"transaction":{"status":"pending","monitorId":"Geth_1_C2_PROD","monitorVersion":"0.108.0","pendingTimeStamp":"2022-02-05T05:47:08.506Z","pendingBlockNumber":14144167,"hash":"0x90702ff7d6ceac84f889e0520ef2373dbfc7046a686c63a99433fbe93905ffff","from":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0","to":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","value":"0","gas":1003544,"nonce":710,"blockHash":null,"blockNumber":null,"v":"0x1","r":"0x135a68dbc7fd639d8c1ec2b7f6a05e4748cb460a2662d852fd79c32cb846fca3","s":"0x11f6ee30bde1efdd58048b7179ddad945115979ffb077ed30b44af1be8317d25","input":"0x677956f100000000000000000000000000000000000000000000000000000000620f5200000000000000000000000000000000000000000000000bb80000000000000000000000000000000000000000000000000000000000000000016345785d8a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ac1541126c04cf00","type":2,"maxFeePerGas":"5000000000","maxFeePerGasGwei":5,"maxPriorityFeePerGas":"1500000000","maxPriorityFeePerGasGwei":1.5,"asset":"ETH","estimatedBlocksUntilConfirmed":null,"watchedAddress":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","direction":"incoming","counterparty":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0"}},"dispatchTimestamp":"2022-02-05T05:47:08.529Z"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_decode_5() {
        let json = r#"{"version":0,"serverVersion":"0.127.0","timeStamp":"2022-02-05T05:45:16.620Z","connectionId":"d4-bf0707bb-a594-478a-be8d-9cbe0bf9dc37","status":"ok","event":{"timeStamp":"2022-02-05T05:45:16.620Z","categoryCode":"activeAddress","eventCode":"txCancel","dappId":"7d507b2c-48f2-48bb-bd79-fc16ced6f8cf","blockchain":{"system":"ethereum","network":"main"},"contractCall":{"methodName":"purchase","params":{"maturity":"1645171200","strike64x64":"55340232221128654848000","contractSize":"100000000000000000","isCall":false,"maxCost":"12288749964831000000"},"contractAddress":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","contractType":"customAbi"},"transaction":{"status":"pending","monitorId":"Geth_1_B_PROD","monitorVersion":"0.108.0","timePending":"92861","blocksPending":5,"pendingTimeStamp":"2022-02-05T05:43:43.759Z","pendingBlockNumber":14144160,"hash":"0xebe0965743276c4e14fb07157b45e494c0f8ec11b1eca006c244c22af347d7f2","from":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0","to":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","value":"0","gas":1003591,"nonce":709,"blockHash":null,"blockNumber":null,"v":"0x1","r":"0x1d8ae7cd39ccf15052f55e0026d1dbee4980d72ecd18eeac365af831a46c1ad5","s":"0x67ae8305a5446319bf2b43756f9c747bd83448d2c76c5bd70ad0c8a50356d909","input":"0x677956f100000000000000000000000000000000000000000000000000000000620f5200000000000000000000000000000000000000000000000bb80000000000000000000000000000000000000000000000000000000000000000016345785d8a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000aa8a68fc035885c0","type":2,"maxFeePerGas":"5030000000","maxFeePerGasGwei":5.03,"maxPriorityFeePerGas":"1500000000","maxPriorityFeePerGasGwei":1.5,"asset":"ETH","watchedAddress":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","direction":"incoming","counterparty":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0","originalHash":"0xcbeb2975f43f6b8027b060b7267661fd5bfe3c1b55cfb2c26b2c23078ae2d48e"}},"dispatchTimestamp":"2022-02-05T05:45:16.631Z"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_decode_6() {
        let json = r#"{"version":0,"serverVersion":"0.127.0","timeStamp":"2022-02-05T05:43:43.759Z","connectionId":"d4-bf0707bb-a594-478a-be8d-9cbe0bf9dc37","status":"ok","event":{"timeStamp":"2022-02-05T05:43:43.759Z","categoryCode":"activeAddress","eventCode":"txPool","dappId":"7d507b2c-48f2-48bb-bd79-fc16ced6f8cf","blockchain":{"system":"ethereum","network":"main"},"contractCall":{"methodName":"purchase","params":{"maturity":"1645171200","strike64x64":"55340232221128654848000","contractSize":"100000000000000000","isCall":false,"maxCost":"12288749964831000000"},"contractAddress":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","contractType":"customAbi"},"transaction":{"status":"pending","monitorId":"Geth_1_C_PROD","monitorVersion":"0.108.0","pendingTimeStamp":"2022-02-05T05:43:43.759Z","pendingBlockNumber":14144160,"hash":"0xcbeb2975f43f6b8027b060b7267661fd5bfe3c1b55cfb2c26b2c23078ae2d48e","from":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0","to":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","value":"0","gas":1003591,"nonce":709,"blockHash":null,"blockNumber":null,"v":"0x1","r":"0x1d8ae7cd39ccf15052f55e0026d1dbee4980d72ecd18eeac365af831a46c1ad5","s":"0x67ae8305a5446319bf2b43756f9c747bd83448d2c76c5bd70ad0c8a50356d909","input":"0x677956f100000000000000000000000000000000000000000000000000000000620f5200000000000000000000000000000000000000000000000bb80000000000000000000000000000000000000000000000000000000000000000016345785d8a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000aa8a68fc035885c0","type":2,"maxFeePerGas":"5030000000","maxFeePerGasGwei":5.03,"maxPriorityFeePerGas":"1500000000","maxPriorityFeePerGasGwei":1.5,"asset":"ETH","estimatedBlocksUntilConfirmed":null,"watchedAddress":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","direction":"incoming","counterparty":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0"}},"dispatchTimestamp":"2022-02-05T05:43:43.872Z"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_decode_7() {
        let json = r#"{"version":0,"serverVersion":"0.127.0","timeStamp":"2022-02-05T05:34:51.275Z","connectionId":"d4-bf0707bb-a594-478a-be8d-9cbe0bf9dc37","status":"ok","event":{"timeStamp":"2022-02-05T05:34:51.275Z","categoryCode":"activeAddress","eventCode":"txCancel","dappId":"7d507b2c-48f2-48bb-bd79-fc16ced6f8cf","blockchain":{"system":"ethereum","network":"main"},"contractCall":{"methodName":"purchase","params":{"maturity":"1645171200","strike64x64":"55340232221128654848000","contractSize":"100000000000000000","isCall":false,"maxCost":"12288749964831000000"},"contractAddress":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","contractType":"customAbi"},"transaction":{"status":"pending","monitorId":"Geth_1_B_PROD","monitorVersion":"0.108.0","timePending":"80513","blocksPending":7,"pendingTimeStamp":"2022-02-05T05:33:30.762Z","pendingBlockNumber":14144116,"hash":"0x44fd2e12b5ce645fa41c1eb74ac2ecab5676548ae8587d67420c7fff9f6814ab","from":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0","to":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","value":"0","gas":1003494,"nonce":708,"blockHash":null,"blockNumber":null,"v":"0x1","r":"0xa7e9450ba733b0d8ce605bcc63bd15f81b8fc74f6fdec022ed1b93bef568b1dd","s":"0x368fcd8c2d89934f48d4b4526bf93358e4c2c4bbd3510b61487231e49a5c50ee","input":"0x677956f100000000000000000000000000000000000000000000000000000000620f5200000000000000000000000000000000000000000000000bb80000000000000000000000000000000000000000000000000000000000000000016345785d8a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000aa8a68fc035885c0","type":2,"maxFeePerGas":"6000000000","maxFeePerGasGwei":6,"maxPriorityFeePerGas":"1551000000","maxPriorityFeePerGasGwei":1.55,"asset":"ETH","watchedAddress":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","direction":"incoming","counterparty":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0","originalHash":"0x546aafe2efb7d5acb61303007a533ca39764f0787eb0c581c659ec44a896dad6"}},"dispatchTimestamp":"2022-02-05T05:34:51.300Z"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_decode_8() {
        let json = r#"{"version":0,"serverVersion":"0.127.0","timeStamp":"2022-02-05T05:33:30.675Z","connectionId":"d4-bf0707bb-a594-478a-be8d-9cbe0bf9dc37","status":"ok","event":{"timeStamp":"2022-02-05T05:33:30.675Z","categoryCode":"activeAddress","eventCode":"txSpeedUp","dappId":"7d507b2c-48f2-48bb-bd79-fc16ced6f8cf","blockchain":{"system":"ethereum","network":"main"},"contractCall":{"methodName":"purchase","params":{"maturity":"1645171200","strike64x64":"55340232221128654848000","contractSize":"100000000000000000","isCall":false,"maxCost":"12288749964831000000"},"contractAddress":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","contractType":"customAbi"},"transaction":{"status":"pending","monitorId":"Geth_1_B_PROD","monitorVersion":"0.108.0","timePending":"36838","blocksPending":0,"pendingTimeStamp":"2022-02-05T05:32:53.837Z","pendingBlockNumber":14144116,"hash":"0x546aafe2efb7d5acb61303007a533ca39764f0787eb0c581c659ec44a896dad6","from":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0","to":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","value":"0","gas":1003494,"nonce":708,"blockHash":null,"blockNumber":null,"v":"0x1","r":"0xacd250a48251b0a83e3d6fa5653c780aa67f838a338dbbcf06c09ddefe1aef35","s":"0x6b376e7ec18fcf302b87729b9155f9a078a70b0cc27b980ecf0249b0b0042f1f","input":"0x677956f100000000000000000000000000000000000000000000000000000000620f5200000000000000000000000000000000000000000000000bb80000000000000000000000000000000000000000000000000000000000000000016345785d8a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000aa8a68fc035885c0","type":2,"maxFeePerGas":"5000000000","maxFeePerGasGwei":5,"maxPriorityFeePerGas":"1410000000","maxPriorityFeePerGasGwei":1.41,"asset":"ETH","watchedAddress":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","direction":"incoming","counterparty":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0","originalHash":"0xd63c3f04c0f85f6bb5402644bbb09290148318c5acb9e9f17d0773e9a7492101"}},"dispatchTimestamp":"2022-02-05T05:33:30.687Z"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_decode_9() {
        let json = r#"{"version":0,"serverVersion":"0.127.0","timeStamp":"2022-02-05T05:32:53.837Z","connectionId":"d4-bf0707bb-a594-478a-be8d-9cbe0bf9dc37","status":"ok","event":{"timeStamp":"2022-02-05T05:32:53.837Z","categoryCode":"activeAddress","eventCode":"txPool","dappId":"7d507b2c-48f2-48bb-bd79-fc16ced6f8cf","blockchain":{"system":"ethereum","network":"main"},"contractCall":{"methodName":"purchase","params":{"maturity":"1645171200","strike64x64":"55340232221128654848000","contractSize":"100000000000000000","isCall":false,"maxCost":"12288749964831000000"},"contractAddress":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","contractType":"customAbi"},"transaction":{"status":"pending","monitorId":"Geth_1_C_PROD","monitorVersion":"0.108.0","pendingTimeStamp":"2022-02-05T05:32:53.837Z","pendingBlockNumber":14144116,"hash":"0xd63c3f04c0f85f6bb5402644bbb09290148318c5acb9e9f17d0773e9a7492101","from":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0","to":"0xa4492fcDa2520cB68657d220f4D4aE3116359C10","value":"0","gas":1003494,"nonce":708,"blockHash":null,"blockNumber":null,"v":"0x1","r":"0xacd250a48251b0a83e3d6fa5653c780aa67f838a338dbbcf06c09ddefe1aef35","s":"0x6b376e7ec18fcf302b87729b9155f9a078a70b0cc27b980ecf0249b0b0042f1f","input":"0x677956f100000000000000000000000000000000000000000000000000000000620f5200000000000000000000000000000000000000000000000bb80000000000000000000000000000000000000000000000000000000000000000016345785d8a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000aa8a68fc035885c0","type":2,"maxFeePerGas":"5000000000","maxFeePerGasGwei":5,"maxPriorityFeePerGas":"1410000000","maxPriorityFeePerGasGwei":1.41,"asset":"ETH","estimatedBlocksUntilConfirmed":null,"watchedAddress":"0xa4492fcda2520cb68657d220f4d4ae3116359c10","direction":"incoming","counterparty":"0x1FF60C59246A7b6B4A5090218881Af7f844458b0"}},"dispatchTimestamp":"2022-02-05T05:32:53.849Z"}"#;
        let resp: Response = serde_json::from_str(json).unwrap();
    }
}
