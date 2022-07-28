use super::contracts::bindings::uniswap_v2_pair;
use super::contracts::bindings::uniswap_v2_pair::UniswapV2Pair;
//use crypto_bigint::generic_array::typenum::Zero;
use ethabi::{Event, EventParam, ParamType, RawLog};
use ethers::contract::Lazy;
use ethers::contract::{abigen, EthCall, EthEvent};
use ethers::core::{
    abi::{AbiDecode, AbiEncode, Address, Tokenizable},
    types::{transaction::eip2718::TypedTransaction, Eip1559TransactionRequest, U256},
};
use ethers::prelude::k256::ecdsa::signature;
use ethers::prelude::*;
use num_traits::FromPrimitive;
use rand::random;
use serde_json::from_str;
use std::collections::HashMap;
use std::str::FromStr;
use std::{convert::TryFrom, sync::Arc};

fn assert_codec<T: AbiDecode + AbiEncode>() {}
fn assert_tokenizeable<T: Tokenizable>() {}

pub static topic_map: Lazy<HashMap<H256, (&str, &str)>> =
    Lazy::new(|| -> HashMap<TxHash, (&str, &str)> {
        HashMap::from([
            (
                H256::from_str(
                    "0xdccd412f0b1252819cb1fd330b93224ca42612892bb3f4f789976e6d81936496",
                )
                .unwrap(),
                ("Burn", "Burn(address,uint256,uint256,address)"),
            ),
            (
                H256::from_str(
                    "0x4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f",
                )
                .unwrap(),
                ("Mint", "Mint(address,uint256,uint256)"),
            ),
            (
                H256::from_str(
                    "0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822",
                )
                .unwrap(),
                (
                    "Swap",
                    "Swap(address,uint256,uint256,uint256,uint256,address)",
                ),
            ),
            (
                H256::from_str(
                    "0x1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1",
                )
                .unwrap(),
                ("Sync", "Sync(uint112,uint112)"),
            ),
        ])
    });

use ethers::prelude::*;
use ethers::providers::{MockProvider, Provider};

pub async fn decode_uniswap_event(
    topic: (&str, &str),
    topics: Vec<H256>,
    decoded: Bytes,
) -> ethabi::Log {
    /*
     let call = SwapCall{a: Address::random(),b: U256::one(),c: U256::one(),d: U256::one(),e: U256::one(), f: Address::random()};
     let contract = Test::new(Address::random(), Arc::new(client));
     let encoded_call = contract.encode("Swap", (Address::random(),U256::one(),U256::one(), U256::one(),U256::one(), Address::random())).unwrap();
     let decoded_call = SwapCall::decode(encoded_call.as_ref()).unwrap();
     //assert_eq!(call, decoded_call);

     let contract_call = TestCalls::Swap(call);
     let decoded_enum = TestCalls::decode(encoded_call.as_ref()).unwrap();
     println!("abi: {} topic: {}", SwapCall::abi_signature(), topic_map.get(&H256::from_str("0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822").unwrap()).unwrap().1);
    // assert_eq!(decoded_enum, contract_call);
     */

    let method = topic.0;
    let signature = topic.1;
    decode_event_tx(method, signature, topics, decoded)
}

pub fn decode_event_tx(
    method: &str,
    signature: &str,
    topics: Vec<H256>,
    payload: Bytes,
) -> ethabi::Log {
    /*
    abigen!(
        Test,
         r#"[
             function Swap(address a,uint256 b,uint256 c,uint256 d,uint256 e,address f),
             function Sync(uint112,uint112),
             function Mint(address,uint256,uint256)
             function Burn(address,uint256,uint256,address)
         ]"#,
     );
     */
    let raw_log = RawLog {
        topics,
        data: payload.to_vec(),
    };
    let mut event = "";
    let mut params: Vec<EventParam> = Default::default();

    if method.eq("Swap") {
        params = vec![
            EventParam {
                name: "sender".to_string(),
                kind: ParamType::Address,
                indexed: true,
            },
            EventParam {
                name: "amount_0_in".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
            EventParam {
                name: "amount_1_in".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
            EventParam {
                name: "amount_0_out".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
            EventParam {
                name: "amount_1_out".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
            EventParam {
                name: "to".to_string(),
                kind: ParamType::Address,
                indexed: true,
            },
        ];
        event = "Swap";
    } else if method.eq("Sync") {
        params = vec![
            EventParam {
                name: "reserve_0".to_string(),
                kind: ParamType::Uint(112),
                indexed: false,
            },
            EventParam {
                name: "reserve_1".to_string(),
                kind: ParamType::Uint(112),
                indexed: false,
            },
        ];
        event = "Sync";
    } else if method.eq("Mint") {
        params = vec![
            EventParam {
                name: "sender".to_string(),
                kind: ParamType::Address,
                indexed: true,
            },
            EventParam {
                name: "amount_0".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
            EventParam {
                name: "amount_1".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
        ];

        event = "Mint";
    } else if method.eq("Burn") {
        params = vec![
            EventParam {
                name: "sender".to_string(),
                kind: ParamType::Address,
                indexed: true,
            },
            EventParam {
                name: "amount_0".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
            EventParam {
                name: "amount_1".to_string(),
                kind: ParamType::Uint(256),
                indexed: false,
            },
            EventParam {
                name: "to".to_string(),
                kind: ParamType::Address,
                indexed: true,
            },
        ];
        event = "Burn";
    }
    let log_event = Event {
        name: event.to_string(),
        inputs: params,
        anonymous: false,
    };
    log_event.parse_log(raw_log).unwrap()
}
