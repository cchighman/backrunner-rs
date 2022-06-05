use std::str::FromStr;
use crate::contracts::bindings::uniswap_v2_pair::UniswapV2Pair;
use anyhow;
use anyhow::Result;
use ethers::core::types::transaction::eip2718::TypedTransaction;
use ethers::prelude::*;
use ethers::prelude::{Address, U256};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use super::uniswap_providers::*;



pub fn get_valid_timestamp(future_millis: U256) -> U256 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let time_millis = since_epoch
        .as_millis()
        .checked_add(u128::try_from(future_millis).unwrap())
        .unwrap();

    return U256::from(time_millis);
}

pub async fn flash_swap_v2(
    pair_id: Address,
    in_amt: U256,
    out_amt: U256,
    calldata: Bytes,
) -> Result<TypedTransaction> {
    
    let pair_contract = UniswapV2Pair::new(
        pair_id, mainnet::client.clone());
    
    let contract_call = pair_contract.swap(
        in_amt,
        out_amt,
        mainnet::flash_contract.clone(),
        calldata);

    Ok(contract_call.tx)
}
/*
#[test]
pub fn test() {
    Abigen::new("UniswapV3", "./uniswapv3.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("contracts/bindings/uniswap_v3_router");
}
#[cfg(test)]
mod tests {
    use super::*;

    async fn swap_eth_for_exact() {
        // Ropsten Uniswap v2
        // Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
        // Factory: 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f
        // Uniswapv2 Library: 0x87a02D44e56942F77642F93c2DBEE5455881Ef87
        // Uniswapv2 Liquidity Math: 0x68226785e5BDcE5967C70372d1B447f6cd6F0724

        //  Mnemonic: unveil spoon stable govern diesel park glory visa lucky teach aspect spy
        // Tether: 0x110a13FC3efE6A245B50102D2d79B3E76125Ae83
        // WETH: 0xc778417E063141139Fce010982780140Aa0cD5Ab
        // Pair: 0xE5133CA897f1c5cdd273775EEFB950f3055F125D
        dbg!("[777]");
        println!(
            "Reserves to Price -> {}",
            reserves_to_amount(417345652584387960934_u128, 6, 53231624430_u128, 18).to_string()
        );
        let provider = Provider::<Http>::try_from(
            "https://ropsten.infura.io/v3/7b15aafb575849f4ab4eaccc2725b4a7",
        )
        .unwrap();

        let phrase = "unveil spoon stable govern diesel park glory visa lucky teach aspect spy";
        let index = 0u32;

        // Access mnemonic phrase with password
        /*
                let chain_id = 1u64;

        let wallet: Wallet<SigningKey> =
            "4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318".parse().unwrap();
        let wallet = wallet.with_chain_id(chain_id);

        let sig = wallet.sign_transaction(&tx).await.unwrap();
         */

        // Child key at derivation path: m/44'/60'/0'/0/{index}
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .index(index)
            .unwrap()
            // Use this if your mnemonic is encrypted
            // .password(password)
            .build()
            .unwrap();
        let wallet = wallet.with_chain_id(3u64);

        dbg!(&wallet);

        let client = Arc::new(SignerMiddleware::new(provider, wallet));
        let pair_contract = UniswapV2Pair::new(
            Address::from_str("0xE5133CA897f1c5cdd273775EEFB950f3055F125D").unwrap(),
            Arc::clone(&client),
        );
        let weth_token = IERC20::new(
            Address::from_str("0xc778417E063141139Fce010982780140Aa0cD5Ab").unwrap(),
            client.clone(),
        );

        let usdt_token = IERC20::new(
            Address::from_str("0x110a13FC3efE6A245B50102D2d79B3E76125Ae83").unwrap(),
            client.clone(),
        );

        let weth_approve = weth_token
            .approve(
                Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap(),
                U256::from_dec_str("999999999999999999999999999").unwrap(),
            )
            .call()
            .await
            .unwrap();

        println!("Approve? {}", weth_approve);

        let addy = Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap();
        let path = vec![
            Address::from_str("0xc778417E063141139Fce010982780140Aa0cD5Ab").unwrap(),
            Address::from_str("0x110a13FC3efE6A245B50102D2d79B3E76125Ae83").unwrap(),
        ];
        let timestamp = get_valid_timestamp(U256::from(30000));
        /* Amount out is amount we want then multiply by 10^(decimals) */
        let call = router_contract.swap_tokens_for_exact_tokens(
            U256::from_dec_str("1").unwrap(),
            U256::from_dec_str("").unwrap(),
            path.clone(),
            addy,
            timestamp,
        );

        println!("CallData: {}", call.calldata().unwrap());
        let result = call.send().await.unwrap().await.unwrap();
        dbg!("Timestamp:  {#:?}", timestamp);
        dbg!("Result:  {#:?}", result);
        let reserves = pair_contract.get_reserves().call().await.unwrap();

        let amount_in =
            router_contract.get_amounts_in(U256::from_dec_str("1").unwrap(), path.clone());

        let amt = amount_in.call().await.unwrap();
        let amount_out =
            router_contract.get_amounts_out(U256::from_dec_str("130").unwrap(), path.clone());

        let amt1 = amount_out.call().await.unwrap();
        dbg!(
            "amt in - {#:?}, amt out - {#:?},  reserves: {#:?}, {#:?}",
            amt[0].to_string(),
            amt1[0].to_string(),
            reserves.0,
            reserves.1
        );
        /*
        let result1 = pair_contract.swap(
            U256::from_dec_str("1").unwrap(),
            U256::from_dec_str("127").unwrap(),
            addy,
            Default::default(),
        );
        dbg!("Result:  {#:?}", result1.call().await);
        */
    }
    }
 */
fn test() {
let call = router_contract.swap_tokens_for_exact_tokens(
U256::from_dec_str("1").unwrap(),
U256::from_dec_str("").unwrap(),
path.clone(),
addy,
timestamp,
}
