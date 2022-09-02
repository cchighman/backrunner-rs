use color_eyre::eyre;
use std::ops::{Add, Sub};

pub(crate) use ethabi::{Event, EventParam, RawLog};
use ethereum_types::U64;
use ethers::core::abi::AbiDecode;
use ethers::core::types::{Filter, ValueOrArray};
use ethers::core::utils::keccak256;
use ethers::prelude::k256::ecdsa::signature;
use ethers::prelude::{Address, BlockNumber, Ws, H256, U256};
use ethers::providers::{Middleware, Provider};

use super::contracts::bindings::uniswap_v2_pair::UniswapV2Pair;
use super::contracts::bindings::uniswap_v2_pair::UniswapV2PairEvents;
use super::transaction_log_utils;
use super::transaction_log_utils::*;
use crate::transaction_utils::{tx_flow, tx_receipt};
use ethers::prelude::*;
use std::sync::Arc;
use std::{collections::HashMap, fs::File, io, thread, time};

use super::contracts::bindings::uniswap_v2_pair;
use crate::crypto_pair::CryptoPair;

use super::uniswap_providers::*;

use ethereum_types::H160;
use ethers::{prelude::*, utils::Ganache};
use std::time::Duration;

#[tokio::test]
async fn test_main() -> eyre::Result<()> {
    let ganache = Ganache::new().block_time(1u64).spawn();
    let ws = Ws::connect(ganache.ws_endpoint()).await?;
    let provider = Provider::new(ws).interval(Duration::from_millis(2000));
    let mut stream = provider.watch_blocks().await?.take(5);
    while let Some(block) = stream.next().await {
        let block = provider.get_block(block).await?.unwrap();
        println!(
            "Ts: {:?}, block number: {} -> {:?}",
            block.timestamp,
            block.number.unwrap(),
            block.hash.unwrap()
        );
    }

    Ok(())
}

#[tokio::main]
pub async fn monitor_tx(pair_map: &mut HashMap<Address, Arc<CryptoPair>>) {
    use tokio::runtime::Runtime;
    println!("[777]");
    let rt = Runtime::new().unwrap();

    let handle = rt.handle();
    let client = Provider::<Ws>::connect(
        "wss://eth-mainnet.alchemyapi.io/v2/EcNibd4j6LaA9r9pJJRkyAVRvZ_MvKk7",
    )
    //Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/20ca45667c5d4fa6b259b9a36babe5c3")
    .await
    .unwrap();

    let client = Arc::new(client);

    let last_block = client
        .get_block(BlockNumber::Latest)
        .await
        .unwrap()
        .unwrap()
        .number
        .unwrap();
    println!("last_block: {}", last_block);

    /* Subscribe to Pairs */
    let addresses = vec![
        "0xc44676d5bce2dc078e50008f07c31e4aaebbf110",
        "0x9e251daeb17981477509779612dc2ffa8075aa8e",
        "0xb20bd5d04be54f870d5c0d3ca85d82b34b836405",
        "0x397ff1542f962076d0bfe58ea045ffa2d347aca0",
        "0x9bd82673c50acb4a3b883d61e070a3c8d9b08e10",
        "0x69b81152c5a8d35a67b32a4d3772795d96cae4da",
        "0x873056a02255872514f05249d93228d788fe4fb4",
        "0xf6c4e4f339912541d3f8ed99dba64a1372af5e5b",
        "0x55d31f68975e446a40a2d02ffa4b0e1bfb233c2f",
        "0xa5c475167f03b1556c054e0da78192cd2779087f",
        "0x6f118ecebc31a5ffe49b87c47ea80f93a2af0a8a",
        "0x1cd926f3e12f7b6c2833fbe7277ac53d529a794e",
        "0xa7a8edfda2b8bf1e5084e2765811effee21ef918",
        "0x3041cbd36888becc7bbcbc0045e3b1f144466f5f",
        "0xc0067d751fb1172dbab1fa003efe214ee8f419b6",
        "0x82917fb0dd65b0e5c85eea66e4f5c1ed484bc629",
        "0x0eee7f7319013df1f24f5eaf83004fcf9cf49245",
        "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852",
        "0x495f8ef80e13e9e1e77d60d2f384bb49694823ef",
        "0xd6f3768e62ef92a9798e5a8cedd2b78907cecef9",
        "0xc3d03e4f041fd4cd388c549ee2a29a9e5075882f",
        "0x23d15edceb5b5b3a23347fa425846de80a2e8e5c",
        "0xe5c5227d8105d8d5f26ff3634eb52e2d7cc15b50",
        "0xea9b00b169dda4cbc9fa0a64b0d1e4a6a23a3f34",
        "0xae461ca67b15dc8dc81ce7615e0320da1a9ab8d5",
        "0x06da0fd433c1a5d7a4faa01111c044910a184553",
        "0x072b999fc3d82f9ea08b8adbb9d63a980ff2b14d",
        "0xb079d6be3faf5771e354586dbc47d0a3d37c34fb",
        "0xa478c2975ab1ea89e8196811f51a7b7ade33eb11",
        "0xd6ef070951d008f1e6426ad9ca1c4fcf7220ee4d",
        "0x3fa5db0910afc2a1e6de45039ea217410fb8641d",
        "0x055475920a8c93cffb64d039a8205f7acc7722d3",
        "0xfcd13ea0b906f2f87229650b8d93a51b2e839ebd",
        "0x7924a818013f39cf800f5589ff1f1f0def54f31f",
        "0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc",
        "0xce7e98d4da6ebda6af474ea618c6b175729cd366",
        "0x3df70e5b6edead5277590d3de5731d17f46e043b",
        "0x68b3465833fb72a70ecdf485e0e4c7bd8665fc45",
        "0xd9e1ce17f2641f24ae83637ab66a2cca9c378b9f",
    ];
    let addy_next: Vec<Address> = addresses
        .iter()
        .map(|addy| addy.parse::<Address>().unwrap())
        .collect::<Vec<Address>>();

    let address_filter = Filter::new().address(ValueOrArray::Array(addy_next));
    let mut stream = client.subscribe_logs(&address_filter).await.unwrap();
    while let Some(log) = stream.next().await {
        let topic = log.topics[0];
        if !topic_map.contains_key(&topic) {
            continue;
        }
        /* Params */
        let trace = topic_map.get(&topic).unwrap();
        let method = trace.0;
        let signature = trace.1;

        let current_block = client
            .get_block(BlockNumber::Latest)
            .await
            .unwrap()
            .unwrap()
            .number
            .unwrap();
        let block_number = log.block_number.unwrap();
        let address = log.address;
        let tx_hash = log.transaction_hash;
        let status = if tx_hash.is_none() {
            "Pending"
        } else {
            "Confirmed"
        };
        let decoded_event = transaction_log_utils::decode_uniswap_event(
            (method.clone(), signature.clone()),
            log.topics.clone(),
            log.data.clone(),
        )
        .await;
        //let receipt = tx_receipt((*mainnet::alchemy_http_client).clone(), tx_hash.clone().unwrap()).await.unwrap();

        //println!("Current Block: {} Target Block: {}, Event: {} Address: {} Status: {} Tx Hash: {:#?} Params: {:#?}",current_block, block_number, &method, &address, &status,  tx_hash.clone().unwrap(), decoded_event);

        let pair_pre = pair_map.get_mut(&log.address);
        if pair_pre.is_none() {
            continue;
        }
        let pair = pair_pre.unwrap();
        if method.eq("Sync") {
            for param in decoded_event.params.iter() {
                let log_param: ethabi::LogParam = (*param).clone();
                if log_param.name.eq("reserve_0") {
                    pair.confirmed_left_reserves
                        .set(log_param.value.into_uint().unwrap());
                } else if log_param.name.eq("reserve_1") {
                    pair.confirmed_right_reserves
                        .set(log_param.value.into_uint().unwrap());
                }
            }
            println!(
                "[Reserves Event] - [Sync] - Block: {} Pair: {} New Left Reserve: {}, New Right Reserve: {}",
                current_block,
                pair.pair_symbol(),
                pair.confirmed_left_reserves(),
                pair.confirmed_right_reserves()
            );
        } else if method.eq("Mint") {
            for param in decoded_event.params.iter() {
                let log_param: ethabi::LogParam = (*param).clone();
                if log_param.name.eq("amount_0") {
                    pair.confirmed_left_reserves
                        .set(log_param.value.into_uint().unwrap());
                } else if log_param.name.eq("amount_1") {
                    pair.confirmed_right_reserves
                        .set(log_param.value.into_uint().unwrap());
                }
                println!("[Reserves Event] - [Mint] - Block: {} Pair: {} New Left Reserve: {}, New Right Reserve: {}", current_block, pair.pair_symbol(),pair.confirmed_left_reserves(), pair.confirmed_right_reserves());
            }
        } else if method.eq("Burn") {
            for param in decoded_event.params.iter() {
                let log_param: ethabi::LogParam = (*param).clone();
                if log_param.name.eq("amount_0") {
                    pair.confirmed_left_reserves
                        .set(log_param.value.into_uint().unwrap());
                } else if log_param.name.eq("amount_1") {
                    pair.confirmed_right_reserves
                        .set(log_param.value.into_uint().unwrap());
                }
                println!("[Reserves Event] - [Burn] - Block: {} Pair: {} New Left Reserve: {}, New Right Reserve: {}", current_block, pair.pair_symbol(),pair.confirmed_left_reserves(), pair.confirmed_right_reserves());
            }
        } else if method.eq("Swap") {
            for param in decoded_event.params.iter() {
                let log_param: ethabi::LogParam = (*param).clone();
                if log_param.name.eq("amount_0_in") {
                    let amt_in_0 = log_param.value.into_uint().unwrap();
                    if amt_in_0.gt(&U256::zero()) {
                        let new_value = pair.confirmed_left_reserves.get_cloned().add(amt_in_0);
                        pair.confirmed_left_reserves.set(new_value);
                    }
                } else if log_param.name.eq("amount_1_in") {
                    let amt_in_1 = log_param.value.into_uint().unwrap();
                    if !amt_in_1.gt(&U256::zero()) {
                        let new_value = pair.confirmed_right_reserves.get_cloned().add(amt_in_1);
                        pair.confirmed_right_reserves.set(new_value);
                    }
                } else if log_param.name.eq("amount_0_out") {
                    let amt_0_out = log_param.value.into_uint().unwrap();
                    if amt_0_out.gt(&U256::zero()) {
                        let new_value = pair.confirmed_left_reserves.get_cloned().sub(amt_0_out);
                        pair.confirmed_right_reserves.set(new_value);
                    }
                } else if log_param.name.eq("amount_1_out") {
                    let amt_1_out = log_param.value.into_uint().unwrap();
                    if amt_1_out.gt(&U256::zero()) {
                        let new_value = pair.confirmed_right_reserves.get_cloned().sub(amt_1_out);
                        pair.confirmed_right_reserves.set(new_value);
                    }
                }
            }
            println!(
                "[Reserves Event] - [Swap] - Block: {} Pair: {} New Left Reserve: {}, New Right Reserve: {}",
                current_block,
                pair.pair_symbol(),
                pair.confirmed_left_reserves(),
                pair.confirmed_right_reserves()
            );
        }
    }
}
