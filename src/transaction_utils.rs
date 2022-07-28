use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{
    Action, Address, CallType, Trace, Transaction, TransactionReceipt, TxHash, U256,
};
use ethers::utils::hex;
use std::convert::TryFrom;

const ARCHIVE_RPC: &str = "https://dashboard.flashbots.net/eth-sJrVNk4Xoa"; // archive node rpc
const TEST_TX: &str = "0x5ab21bfba50ad3993528c2828c63e311aafe93b40ee934790e545e150cb6ca73"; // Test tx to verify token flows
const WEI_IN_ETHER: U256 = U256([0x0de0b6b3a7640000, 0x0, 0x0, 0x0]);

// Relevant contracts
const WETH: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const USDC: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const USDT: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
const DAI: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";

pub async fn tx_data<M: Middleware + Clone + 'static>(
    provider: M,
    tx: TxHash,
) -> Result<Transaction, anyhow::Error> {
    Ok(provider.get_transaction(tx).await.unwrap().unwrap())
}

pub async fn tx_traces<M: Middleware + Clone + 'static>(
    provider: M,
    tx: TxHash,
) -> Result<Vec<Trace>, anyhow::Error> {
    Ok(provider.trace_transaction(tx).await.unwrap())
}

pub async fn tx_receipt<M: Middleware + Clone + 'static>(
    provider: M,
    tx: TxHash,
) -> Result<TransactionReceipt, anyhow::Error> {
    Ok(provider.get_transaction_receipt(tx).await.unwrap().unwrap())
}

// A bot can have a contract (that it initially calls) AND a proxy contract (that the initial contract triggers via DelegateCall)
// that engage in extracting MEV, We find the proxy implementation (if any) to see tokenflows on them
fn proxy_impl(tx_traces: Vec<Trace>, contract: Address) -> Address {
    let mut proxy_impl: Address = Address::zero();
    for trace in tx_traces.iter() {
        match &trace.action {
            Action::Call(call) => {
                if proxy_impl == Address::zero()
                    && call.call_type == CallType::DelegateCall
                    && call.from == contract
                {
                    proxy_impl = call.to; // TODO: handle the edge case of multiple proxies
                }
            }
            _ => continue, // we skip over other action types as we only care about the proxy (if any)
        }
    }
    return proxy_impl;
}

fn crop_address(s: &mut String, pos: usize) {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => {
            s.drain(..pos);
        }
        None => {
            s.clear();
        }
    }
}

fn ether_flows(
    tx_traces: Vec<Trace>,
    eoa: Address,
    contract: Address,
    proxy: Address,
) -> [U256; 2] {
    let mut eth_inflow = U256::zero();
    let mut eth_outflow = U256::zero();
    for trace in tx_traces.iter() {
        match &trace.action {
            Action::Call(call) => {
                // ETH_GET
                // Check if the call transfers value, isn't from WETH (to avoid double counting) and transfers to one of the relevant addresses
                if call.value > U256::zero()
                    && call.call_type != CallType::DelegateCall
                    && call.from != WETH.parse::<Address>().unwrap()
                    && (call.to == eoa || call.to == contract || call.to == proxy)
                {
                    eth_inflow += call.value;
                }

                // ETH_GIVE
                // Check if the call transfers value, isn't from WETH and transfers ETH out of one of the relevant addresses
                if call.value > U256::zero()
                    && call.call_type != CallType::DelegateCall
                    && call.to != WETH.parse::<Address>().unwrap()
                    && (call.from == eoa || call.from == contract || call.from == proxy)
                {
                    eth_outflow += call.value;
                }

                // WETH_GET1 & WETH_GET2
                // WETH_GIVE1 & WETH_GIVE2
                if call.to == WETH.parse::<Address>().unwrap() {
                    let data = call.input.as_ref();
                    if data.len() == 68 {
                        // 4 bytes of function identifier + 64 bytes of params
                        let fn_signature = hex::encode(&data[..4]);
                        if fn_signature == "a9059cbb" {
                            // transfer(address to,uint256 value )
                            let value = U256::from(&data[36..68]); // WETH amount
                            let mut address = hex::encode(&data[4..36]);
                            crop_address(&mut address, 24);
                            let prefix: &str = "0x";
                            let final_address =
                                format!("{}{}", prefix, address).parse::<Address>().unwrap();
                            if final_address == eoa
                                || final_address == contract
                                || final_address == proxy
                            {
                                // Might have to exclude direct calls to uniswap router
                                // Once we confirm that the traces contain a WETH transfer to one of the relevant address
                                // we count that towards the inflow
                                eth_inflow += value;
                            } else if call.from == eoa
                                || call.from == contract
                                || call.from == proxy
                            {
                                // If the WETH flows from the searchers accounts/contracts.
                                eth_outflow += value;
                            }
                        }
                    }
                    if data.len() == 100 {
                        let fn_signature = hex::encode(&data[..4]);
                        if fn_signature == "23b872dd" {
                            // transferFrom(address from,address to,uint256 value )
                            let value = U256::from(&data[68..100]); // WETH amount
                            let mut to_address = hex::encode(&data[36..68]);
                            let mut from_address = hex::encode(&data[4..36]);
                            crop_address(&mut to_address, 24);
                            crop_address(&mut from_address, 24);
                            let prefix: &str = "0x";
                            let final_to_address = format!("{}{}", prefix, to_address)
                                .parse::<Address>()
                                .unwrap();
                            let final_from_address = format!("{}{}", prefix, to_address)
                                .parse::<Address>()
                                .unwrap();
                            if final_to_address == eoa
                                || final_to_address == contract
                                || final_to_address == proxy
                            {
                                // Might have to exclude direct calls to uniswap router
                                // Once we confirm that the traces contain a WETH transfer to one of the relevant address
                                // we count that towards the inflow
                                eth_inflow += value;
                            } else if final_from_address == eoa
                                || final_from_address == contract
                                || final_from_address == proxy
                            {
                                // Vice versa
                                eth_outflow += value;
                            }
                        }
                    }
                }
            }
            // ETH_SELFDESTRUCT
            Action::Suicide(suicide) => {
                // The OP code was renamed to "Self-destruct" but OpenEthereum still uses the old ref
                // If a trace calls self destruct, transferring the funds to either the eoa/contract/proxy
                // we count the ETH transferred out towards our net inflows
                if suicide.refund_address == eoa
                    || suicide.refund_address == contract
                    || suicide.refund_address == proxy
                {
                    eth_inflow += suicide.balance;
                }

                // What if they transfer the funds out to an arbitrary address that's not any of the addresses?
                // i.e If a searcher uses a cold storage address to transfer out the arb profits
            }
            _ => {
                // we ignore the case for action type Create/Reward as it doesn't pertain to eth inflows or outflows
                continue;
            }
        }
    }
    if eth_outflow > U256::zero() && eth_inflow > U256::zero() {
        return [eth_inflow, eth_outflow];
    }
    return [U256::zero(), U256::zero()];
}

fn stablecoin_flows(
    tx_traces: Vec<Trace>,
    eoa: Address,
    contract: Address,
    proxy: Address,
) -> [U256; 2] {
    let mut dollar_inflow = U256::zero();
    let mut dollar_outflow = U256::zero();
    for trace in tx_traces.iter() {
        match &trace.action {
            Action::Call(call) => {
                // USD_GET1 & USD_GET2
                // USD_GIVE1 & USD_GIVE2
                if call.to == USDC.parse::<Address>().unwrap()
                    || call.to == USDT.parse::<Address>().unwrap()
                    || call.to == DAI.parse::<Address>().unwrap()
                {
                    let data = call.input.as_ref();
                    if data.len() == 68 {
                        // 4 bytes of function identifier + 64 bytes of params
                        let fn_signature = hex::encode(&data[..4]);
                        if fn_signature == "a9059cbb" {
                            // transfer(address to,uint256 value )
                            let value = U256::from(&data[36..68]); // USD amount
                            let mut address = hex::encode(&data[4..36]);
                            crop_address(&mut address, 24);
                            let prefix: &str = "0x";
                            let final_address =
                                format!("{}{}", prefix, address).parse::<Address>().unwrap();
                            if final_address == eoa
                                || final_address == contract
                                || final_address == proxy
                            {
                                // Might have to exclude direct calls to uniswap router
                                // Once we confirm that the traces contain a USD transfer to one of the relevant address
                                // we count that towards the inflow
                                if call.to != DAI.parse::<Address>().unwrap() {
                                    // DAI has 18 digits while USDT/USDC have 6
                                    dollar_inflow += value / U256::from(1000000);
                                } else {
                                    //dollar_inflow += value/U256::from();
                                    dollar_inflow += value / WEI_IN_ETHER;
                                }
                            } else if call.from == eoa
                                || call.from == contract
                                || call.from == proxy
                            {
                                // If the USD flows from the searchers accounts/contracts.
                                if call.to != DAI.parse::<Address>().unwrap() {
                                    // DAI has 18 digits while USDT/USDC have 6
                                    dollar_outflow += value / U256::from(1000000);
                                } else {
                                    //dollar_inflow += value/U256::from();
                                    dollar_outflow += value / WEI_IN_ETHER;
                                }
                            }
                        }
                    }
                    if data.len() == 100 {
                        let fn_signature = hex::encode(&data[..4]);
                        if fn_signature == "23b872dd" {
                            // transferFrom(address from,address to,uint256 value )
                            let value = U256::from(&data[68..100]); // USD amount
                            let mut to_address = hex::encode(&data[36..68]);
                            let mut from_address = hex::encode(&data[4..36]);
                            crop_address(&mut to_address, 24);
                            crop_address(&mut from_address, 24);
                            let prefix: &str = "0x";
                            let final_to_address = format!("{}{}", prefix, to_address)
                                .parse::<Address>()
                                .unwrap();
                            let final_from_address = format!("{}{}", prefix, to_address)
                                .parse::<Address>()
                                .unwrap();
                            if final_to_address == eoa
                                || final_to_address == contract
                                || final_to_address == proxy
                            {
                                // Might have to exclude direct calls to uniswap router
                                // Once we confirm that the traces contain a USD transfer to one of the relevant address
                                // we count that towards the inflow
                                if call.to != DAI.parse::<Address>().unwrap() {
                                    // DAI has 18 digits while USDT/USDC have 6
                                    dollar_inflow += value / U256::from(1000000);
                                } else {
                                    dollar_inflow += value / WEI_IN_ETHER;
                                }
                            } else if final_from_address == eoa
                                || final_from_address == contract
                                || final_from_address == proxy
                            {
                                // Vice versa
                                if call.to != DAI.parse::<Address>().unwrap() {
                                    // DAI has 18 digits while USDT/USDC have 6
                                    dollar_outflow += value / U256::from(1000000);
                                } else {
                                    dollar_outflow += value / WEI_IN_ETHER;
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                // we ignore the case for action type Create/Reward/Self-destruct as it doesn't pertain to eth inflows or outflows
                continue;
            }
        }
    }
    if dollar_outflow > U256::zero() && dollar_inflow > U256::zero() {
        return [dollar_inflow, dollar_outflow];
    }
    return [U256::zero(), U256::zero()];
}

pub async fn tx_flow<M: Middleware + Clone + 'static>(provider: M, tx_hash: H256) {
    println!("tx_flow");
    let tx_data = tx_data(provider.clone(), tx_hash).await;
    if !tx_data.is_err() {
        let tx_receipt = tx_receipt(provider.clone(), tx_hash.clone()).await; // receipt to find of if it failed + gas used.
        if !tx_receipt.is_err() {
            let tx_data_impl = tx_data.unwrap();
            let tx_receipt_impl = tx_receipt.unwrap();
            dbg!(tx_data_impl.clone());
            dbg!(tx_receipt_impl.clone());

            let gas_used_in_wei = tx_receipt_impl.gas_used.unwrap();
            let cost_in_wei = gas_used_in_wei * tx_data_impl.gas_price.unwrap();
            let eoa = tx_data_impl.from; // searcher address
            let contract = tx_data_impl.to.unwrap(); // contract that does the atomic arb or simply arranges txs in a bundle
                                                     //let tx_traces = tx_traces(provider.clone(), tx_hash).await;
                                                     //let proxy = proxy_impl(tx_traces.clone(), contract);
            println!("EOA: {:?}", eoa);
            println!("Contract: {:?}", contract);
            //println!("Tx proxy: {:?}", proxy);
            //let ether_flows = ether_flows(tx_traces.clone(), eoa, contract, proxy);
            //println!("Stablecoins inflow/outflow: {:?}", stablecoin_flows(tx_traces.clone(), eoa, contract, proxy));
            //println!("Net ETH profit, Wei {:?}", (ether_flows[0] - ether_flows[1] - cost_in_wei));

            //TODO: Convert stablecoin profits to ETH based on historical price
            //TODO: Test cases for each function, especially around math precision
        }
    } else {
        println!("tx_flow error: {:#?}", tx_data.err());
    }
}
