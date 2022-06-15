use color_eyre::eyre;
use ethers::core::abi::AbiDecode;
use ethers::core::types::{Filter, ValueOrArray};
use ethers::core::utils::keccak256;
use ethereum_types::U64;
use ethers::prelude::{Address, H256, U256, BlockNumber, Ws};
use ethers::providers::{Middleware, Provider};
use std::sync::Arc;
use std::{collections::HashMap, fs::File, io, thread, time};
use super::uniswap_providers::*;

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
pub async fn test_sub()->Result<(), anyhow::Error> {
    let last_block = mainnet::client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    
    let erc20_transfer_filter = Filter::new()
    .from_block(last_block - 25)
    .topic0(ValueOrArray::Value(H256::from(keccak256("Transfer(address,address,uint256)"))));
            
    let wss_client =Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/20ca45667c5d4fa6b259b9a36babe5c3").await.unwrap();
            
    let mut stream = wss_client.subscribe_logs(&erc20_transfer_filter).await.unwrap().take(5);
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

#[tokio::test]
async fn test_sub_logs() -> Result<(), anyhow::Error> {
    let client =
        Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27")
            .await?;
    let client = Arc::new(client);

    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    println!("last_block: {}", last_block);
 
     
    let erc20_transfer_filter = Filter::new()
        .from_block(last_block - 25);
      //  .topic0(ValueOrArray::Value(H256::from(keccak256("swap(uint256,uint256,address,bytes)"))));

    let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?;

    while let Some(log) = stream.next().await {
        println!(
            "block: {:?}, tx: {:?}, token: {:?}, from: {:?} {:?}, to: {:?}, {:?} amount: {:?}",
            log.block_number,
            log.transaction_hash,
            log.address,
            log.topics,
            dbg!(log.clone()),
            Address::from(log.topics[1]),
            Address::from(log.topics[2]),
            U256::decode(log.data)
        );
    }
    // Abigen creates a SwapExactTokensForTokensCall struct that can be used to decode
// the call data for the swapExactTokensForTokens function in the IUniswapV2Router02 contract
abigen!(
    IUniswapV2Router02,
    r#"[
        swapExactTokensForTokens(uint256 amountIn, uint256 amountOutMin, address[] calldata path, address to, uint256 deadline)
    ]"#,
);  
    use std::str::FromStr;
/*     println!("Decoding https://etherscan.io/tx/0xd1b449d8b1552156957309bffb988924569de34fbf21b51e7af31070cc80fe9a");
    let tx_input = &b"0x38ed173900000000000000000000000000000000000000000001a717cc0a3e4f84c00000000000000000000000000000000000000000000000000000000000000283568400000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000201f129111c60401630932d9f9811bd5b5fff34e000000000000000000000000000000000000000000000000000000006227723d000000000000000000000000000000000000000000000000000000000000000200000000000000000000000095ad61b0a150d79219dcf64e1e6cc01f0b64c4ce000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7";
    let calldata: Bytes = bytes::Bytes::from(tx_input);
    let decoded = SwapExactTokensForTokensCall::decode(&calldata)?;
    let mut path = decoded.path.into_iter();
    let from = path.next().unwrap();
    let to = path.next().unwrap();
    println!(
        "Swapped {} of token {} for {} of token {}",
        decoded.amount_in, from, decoded.amount_out_min, to
    );
*/

    Ok(())
}

