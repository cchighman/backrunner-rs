use ethers::core::abi::AbiDecode;
use ethers::core::types::{Filter, ValueOrArray};
use ethers::core::utils::keccak256;
use ethers::prelude::{Address, H256, U256};
use ethers::providers::{Middleware, Provider};
use std::sync::Arc;
use std::{collections::HashMap, fs::File, io, thread, time};

/*

pub async fn start() {
    let mut time = 0u64;

    /*
    let txs = match get_txs(provider, time).await {
        Ok(t) => t,
        Err(_) => panic!("Unable to retrieve transactions."),
    };
    dbg!(txs);
    */
}

#[tokio::test]
pub async fn test_sub() {
    let erc20_transfer_filter =
        Filter::new()
            .from_block(last_block - 25)
            .topic0(ValueOrArray::Value(H256::from(keccak256(
                "Transfer(address,address,uint256)",
            ))));

    let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?;

    while let Some(log) = stream.next().await {
        println!(
            "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, amount: {:?}",
            log.block_number,
            log.transaction_hash,
            log.address,
            Address::from(log.topics[1]),
            Address::from(log.topics[2]),
            U256::decode(log.data)
        );
    }

    Ok(())
}

#[tokio::test]
pub async fn test_tx() {
    let provider =
        get_provider("wss://ropsten.infura.io/v3/7b15aafb575849f4ab4eaccc2725b4a7").await;

    let t = get_txs(provider, time).await.unwrap();
    println!(t.len());
    let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?;

    while let Some(log) = stream.next().await {
        println!(
            "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, amount: {:?}",
            log.block_number,
            log.transaction_hash,
            log.address,
            Address::from(log.topics[1]),
            Address::from(log.topics[2]),
            U256::decode(log.data)
        );
    }
}
*/
