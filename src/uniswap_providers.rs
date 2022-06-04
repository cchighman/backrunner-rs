<<<<<<< HEAD

=======
pub mod UniswapProviders {
>>>>>>> parent of 6c40e7b... Optimize
    use std::convert::TryFrom;
    use std::fmt;
    use std::ops::Deref;
    use std::str::FromStr;
    use std::sync::Arc;

    //use crate::contracts::bindings::uniswap_v2_router_02::UniswapV2Router02;
    use anyhow;
    use ethers::contract::Lazy;
    use ethers::core::k256::ecdsa::SigningKey;
    use ethers::middleware::SignerMiddleware;
    use ethers::prelude::*;
    use ethers::prelude::{Address, U256};
    use ethers::providers::Middleware;
    use ethers::providers::{Http, Provider};
    use ethers::signers::Signer;
    use ethers::signers::Wallet;
    use ethers_flashbots::FlashbotsMiddleware;
    use url::Url;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Dex {
        Balancer,
        Curve,
        PancakeSwap,
        SushiSwap,
        QuickSwap,
        UniswapV2,
        UniswapV3,
    }

    impl fmt::Display for Dex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

<<<<<<< HEAD
    pub static UNISWAP_PROVIDERS: Lazy<Arc<UniswapProviders>> =
    Lazy::new(|| {
        Arc::new(UniswapProviders::new())});


    #[derive(Clone)]
    pub struct UniswapProviders {
        pub CONTRACT_ADDRESS :Address,
        pub FROM_ADDRESS : Address,
        pub TIMESTAMP_SEED : u128,
        pub MAX_AMOUNT : U256,
        pub TO_ADDRESS : Address,
        pub GOERLI_WALLET: Wallet<SigningKey>,
        pub MAINNET_BOT_SIGNER:Wallet<SigningKey>,
        pub MAINNET_BUNDLE_SIGNER : Wallet<SigningKey>>,
        pub MAINNET_ROUTER_V2_ADDY :Address,
        pub MAINNET_PROVIDER :  Provider<Http>,
        pub MAINNET_MIDDLEWARE :  SignerMiddleware<Provider<Http>, Wallet<SigningKey>> ,
        pub MAINNET_FLASHBOTS_CLIENT : SignerMiddleware<FlashbotsMiddleware<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>, Wallet<SigningKey>>, Wallet<SigningKey>>,
        pub MAINNET_ETH_CLIENT : SignerMiddleware<Provider<Http>, Wallet<SigningKey>>
 }

    impl UniswapProviders {
        pub fn new() -> Self {
            let timestamp_seed =  30000_u128;
            let to_address =Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap();
            let contact_address = Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap();
            let from_address = Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap();
            let mainnet_router_v2_addy =  Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap();
            let max_amount =
            U256::from_str("9999999999999999999999999999999999").unwrap();
            let mainnet_bot_signer = {
                    let private_key = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";
                    let wallet = private_key.parse::<LocalWallet>().unwrap();
                    let wallet = wallet.with_chain_id(1u64);
                    wallet
                };
            }

            let mainnet_bot_signer =  {
                let private_key = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";
                let wallet = private_key.parse::<LocalWallet>()?;
                let wallet = wallet.with_chain_id(1u64);
                wallet
            };

            let mainnet_provider = {
                Provider::<Http>::try_from("https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3")
                .unwrap();
            };

            let mainnet_middleware =  {
                SignerMiddleware::new(
                Provider::<Http>::try_from(
                    "https://ropsten.infura.io/v3/7b15aafb575849f4ab4eaccc2725b4a7",
                )
                .unwrap(),
                (mainnet_bot_signer).clone())
            };


            let mainnet_flashbots_client = SignerMiddleware::new(
                FlashbotsMiddleware::new(
                    mainnet_middlewear,
                    Url::parse("https://relay.flashbots.net").unwrap(),
                    *mainnet_bundle_signer,
                ),
                mainnet_bot_signer
            );

            let mainnet_eth_client = SignerMiddleware::new(mainnet_provider, mainnet_bot_signer);

            Self {
                CONTRACT_ADDRESS: contact_address,
                FROM_ADDRESS: from_address,
                TIMESTAMP_SEED:timestamp_seed,
                MAX_AMOUNT: max_amount,

                TO_ADDRESS: to_address,
                MAINNET_BOT_SIGNER: ptr1,
                MAINNET_BUNDLE_SIGNER: mainnet_bundle_signer,
                MAINNET_ROUTER_V2_ADDY: mainnet_router_v2_addy,
                MAINNET_PROVIDER: mainnet_provider,
                MAINNET_MIDDLEWARE: mainnet_middleware,
                MAINNET_FLASHBOTS_CLIENT: mainnet_flashbots_client,
                MAINNET_ETH_CLIENT: mainnet_eth_client,
            }
            Self
        }


=======
    /*
    #[derive(Clone)]
    pub struct UniswapProviders {
        pub providers: u8
    }

    impl UniswapProviders {
        pub fn new() -> Self {
            Self {
                providers: 5,
            }
        }
    }
    */
>>>>>>> parent of 6c40e7b... Optimize
    //  Mainnet
    // https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3
    // wss://mainnet.infura.io/ws/v3/20ca45667c5d4fa6b259b9a36babe5c3

    // Goerli
    // https://goerli.infura.io/v3/0ab0b9c9d5bf44818399aea45b5ade51
    // wss://goerli.infura.io/ws/v3/0ab0b9c9d5bf44818399aea45b5ade51

    /*
<<<<<<< HEAD
    pub async fn GOERLI_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
=======
    pub static GOERLI_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
>>>>>>> parent of 6c40e7b... Optimize
        MnemonicBuilder::<English>::default()
            .phrase("unveil spoon stable govern diesel park glory visa lucky teach aspect spy")
            .index(0u32)
            .unwrap()
            .build()
            .unwrap()
            .with_chain_id(5_u64)
    });

<<<<<<< HEAD
    pub async fn FLASHBOTS_GOERLI_PROVIDER: Lazy<
=======
    pub static FLASHBOTS_GOERLI_PROVIDER: Lazy<
>>>>>>> parent of 6c40e7b... Optimize
        Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    > = Lazy::new(|| {
        Arc::new(SignerMiddleware::new(
            FlashbotsMiddleware::new(
                Provider::<Http>::try_from("https://mainnet.eth.aragon.network").unwrap(),
                Url::parse("https://relay.flashbots.net").unwrap(),
                LocalWallet::new(&mut thread_rng()),
            ),
            LocalWallet::new(&mut thread_rng()),
        ))
    });

    pub(crate) static ROPSTEN_WALLET: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
        MnemonicBuilder::<English>::default()
            .phrase("unveil spoon stable govern diesel park glory visa lucky teach aspect spy")
            .index(0u32)
            .unwrap()
            .build()
            .unwrap()
            .with_chain_id(3_u64)
    });

<<<<<<< HEAD
    pub async fn ROPSTEN_PROVIDER: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
=======
    pub static ROPSTEN_PROVIDER: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
>>>>>>> parent of 6c40e7b... Optimize
        Lazy::new(|| {
            Arc::new(SignerMiddleware::new(
                Provider::<Http>::try_from(
                    "https://ropsten.infura.io/v3/7b15aafb575849f4ab4eaccc2725b4a7",
                )
                .unwrap(),
                (*ROPSTEN_WALLET.deref()).clone(),
            ))
        });

    */
<<<<<<< HEAD
    // Ropsten Uniswap v2
    // Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D

/*
         pub async fn ROPSTEN_PROVIDER: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
        Lazy::new(|| {
            Arc::new(SignerMiddleware::new(
                Provider::<Http>::try_from(
                    "https://ropsten.infura.io/v3/7b15aafb575849f4ab4eaccc2725b4a7",
                )
                .unwrap(),
                (*ROPSTEN_WALLET.deref()).clone(),
            ))
        });
*/


    /*
    pub async fn ROUTER_CONTRACT: Lazy<u8
    > = Lazy::new(|| 7);
        }*/

=======

    pub static MAINNET_BOT_SIGNER: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
        let private_key = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";
        let wallet = private_key.parse::<LocalWallet>().unwrap();
        let wallet = wallet.with_chain_id(1u64);
        wallet
    });

    pub static MAINNET_BUNDLE_SIGNER: Lazy<Wallet<SigningKey>> = Lazy::new(|| {
        let private_key = "7005b56052be4776bffe00ff781879c65aa87ac3d5f8945c0452f27e11fa9236";
        let wallet = private_key.parse::<LocalWallet>().unwrap();
        let wallet = wallet.with_chain_id(1u64);
        wallet
    });

    pub static TIMESTAMP_SEED: u128 = 30000_u128;
    pub static MAX_AMOUNT: Lazy<U256> =
        Lazy::new(|| U256::from_str("9999999999999999999999999999999999").unwrap());

    pub static TO_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap());

    pub static CONTRACT_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap());

    pub static FROM_ADDRESS: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x5C1201e06F2EB55dDf656F0a82e57cF92F634273").unwrap());

    // Ropsten Uniswap v2
    // Router: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
    static MAINNET_ROUTER_V2_ADDY: Lazy<Address> =
        Lazy::new(|| Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap());

    pub static MAINNET_PROVIDER: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
        Lazy::new(|| {
            Arc::new(SignerMiddleware::new(
                Provider::<Http>::try_from(
                    "https://mainnet.infura.io/v3/20ca45667c5d4fa6b259b9a36babe5c3",
                )
                .unwrap(),
                (*MAINNET_BOT_SIGNER.deref()).clone(),
            ))
        });

    pub static MAINNET_FLASHBOTS_CLIENT: Lazy<
        Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    > = Lazy::new(|| {
        SignerMiddleware::new(
            FlashbotsMiddleware::new(
                MAINNET_PROVIDER,
                Url::parse("https://relay.flashbots.net")?,
                MAINNET_BUNDLE_SIGNER,
            ),
            MAINNET_BOT_SIGNER,
        )
    });

    pub static MAINNET_ETH_CLIENT: Lazy<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> =
        Lazy::new(|| Arc::new(SignerMiddleware::new(MAINNET_PROVIDER, MAINNET_BOT_SIGNER)));

    /*
    pub static ROUTER_CONTRACT: Lazy<u8
    > = Lazy::new(|| 7);
        }*/
}
>>>>>>> parent of 6c40e7b... Optimize
