pub use uniswapv3_mod::*;

#[allow(clippy::too_many_arguments)]
mod uniswapv3_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]

    use ethers::core::types::*;
    use ethers::providers::Middleware;

    pub static UNISWAPV3_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json::from_str("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_factory\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_WETH9\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH9\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactInputParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactInputSingleParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactInputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactOutputParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMaximum\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactOutput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ISwapRouter.ExactOutputSingleParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountInMaximum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactOutputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"results\",\"type\":\"bytes[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"refundETH\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"selfPermit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"selfPermitAllowed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"selfPermitAllowedIfNecessary\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"selfPermitIfNecessary\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sweepToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeBips\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"feeRecipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sweepTokenWithFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"amount0Delta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount1Delta\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"uniswapV3SwapCallback\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"unwrapWETH9\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeBips\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"feeRecipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"unwrapWETH9WithFee\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]").expect("invalid abi")
        });

    #[derive(Clone)]
    pub struct UniswapV3<M>(ethers::contract::Contract<M>);

    impl<M> std::ops::Deref for UniswapV3<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<M: Middleware> std::fmt::Debug for UniswapV3<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapV3))
                .field(&self.address())
                .finish()
        }
    }

    impl<'a, M: Middleware> UniswapV3<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>>(address: T, client: std::sync::Arc<M>) -> Self {
            ethers::contract::Contract::new(address.into(), UNISWAPV3_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `WETH9` (0x4aa4a4fc) function"]
        pub fn weth9(&self) -> ethers::contract::builders::ContractCall<M, Address> {
            self.0
                .method_hash([74, 164, 164, 252], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactInput` (0xc04b8d59) function"]
        pub fn exact_input(
            &self,
            params: ExactInputParams,
        ) -> ethers::contract::builders::ContractCall<M, U256> {
            self.0
                .method_hash([192, 75, 141, 89], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactInputSingle` (0x414bf389) function"]
        pub fn exact_input_single(
            &self,
            params: ExactInputSingleParams,
        ) -> ethers::contract::builders::ContractCall<M, U256> {
            self.0
                .method_hash([65, 75, 243, 137], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactOutput` (0xf28c0498) function"]
        pub fn exact_output(
            &self,
            params: ExactOutputParams,
        ) -> ethers::contract::builders::ContractCall<M, U256> {
            self.0
                .method_hash([242, 140, 4, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactOutputSingle` (0xdb3e2198) function"]
        pub fn exact_output_single(
            &self,
            params: ExactOutputSingleParams,
        ) -> ethers::contract::builders::ContractCall<M, U256> {
            self.0
                .method_hash([219, 62, 33, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(&self) -> ethers::contract::builders::ContractCall<M, Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multicall` (0xac9650d8) function"]
        pub fn multicall(
            &self,
            data: Vec<Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, Vec<Bytes>> {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refundETH` (0x12210e8a) function"]
        pub fn refund_eth(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermit` (0xf3995c67) function"]
        pub fn self_permit(
            &self,
            token: Address,
            value: U256,
            deadline: U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 153, 92, 103], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermitAllowed` (0x4659a494) function"]
        pub fn self_permit_allowed(
            &self,
            token: Address,
            nonce: U256,
            expiry: U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 89, 164, 148], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermitAllowedIfNecessary` (0xa4a78f0c) function"]
        pub fn self_permit_allowed_if_necessary(
            &self,
            token: Address,
            nonce: U256,
            expiry: U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 167, 143, 12], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermitIfNecessary` (0xc2e3140a) function"]
        pub fn self_permit_if_necessary(
            &self,
            token: Address,
            value: U256,
            deadline: U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 227, 20, 10], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweepToken` (0xdf2ab5bb) function"]
        pub fn sweep_token(
            &self,
            token: Address,
            amount_minimum: U256,
            recipient: Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 42, 181, 187], (token, amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweepTokenWithFee` (0xe0e189a0) function"]
        pub fn sweep_token_with_fee(
            &self,
            token: Address,
            amount_minimum: U256,
            recipient: Address,
            fee_bips: U256,
            fee_recipient: Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 225, 137, 160],
                    (token, amount_minimum, recipient, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function"]
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: I256,
            amount_1_delta: I256,
            data: Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWETH9` (0x49404b7c) function"]
        pub fn unwrap_weth9(
            &self,
            amount_minimum: U256,
            recipient: Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 64, 75, 124], (amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWETH9WithFee` (0x9b2c0a37) function"]
        pub fn unwrap_weth9_with_fee(
            &self,
            amount_minimum: U256,
            recipient: Address,
            fee_bips: U256,
            fee_recipient: Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [155, 44, 10, 55],
                    (amount_minimum, recipient, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
    }

    impl<M: Middleware> From<ethers::contract::Contract<M>> for UniswapV3<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }

    #[doc = "Container type for all input parameters for the `WETH9`function with signature `WETH9()` and selector `[74, 164, 164, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(name = "WETH9", abi = "WETH9()")]
    pub struct Weth9Call;

    #[doc = "Container type for all input parameters for the `exactInput`function with signature `exactInput((bytes,address,uint256,uint256,uint256))` and selector `[192, 75, 141, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "exactInput",
        abi = "exactInput((bytes,address,uint256,uint256,uint256))"
    )]
    pub struct ExactInputCall {
        pub params: ExactInputParams,
    }

    #[doc = "Container type for all input parameters for the `exactInputSingle`function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `[65, 75, 243, 137]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "exactInputSingle",
        abi = "exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactInputSingleCall {
        pub params: ExactInputSingleParams,
    }

    #[doc = "Container type for all input parameters for the `exactOutput`function with signature `exactOutput((bytes,address,uint256,uint256,uint256))` and selector `[242, 140, 4, 152]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "exactOutput",
        abi = "exactOutput((bytes,address,uint256,uint256,uint256))"
    )]
    pub struct ExactOutputCall {
        pub params: ExactOutputParams,
    }

    #[doc = "Container type for all input parameters for the `exactOutputSingle`function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `[219, 62, 33, 152]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "exactOutputSingle",
        abi = "exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactOutputSingleCall {
        pub params: ExactOutputSingleParams,
    }

    #[doc = "Container type for all input parameters for the `factory`function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;

    #[doc = "Container type for all input parameters for the `multicall`function with signature `multicall(bytes[])` and selector `[172, 150, 80, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: Vec<Bytes>,
    }

    #[doc = "Container type for all input parameters for the `refundETH`function with signature `refundETH()` and selector `[18, 33, 14, 138]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(name = "refundETH", abi = "refundETH()")]
    pub struct RefundETHCall;

    #[doc = "Container type for all input parameters for the `selfPermit`function with signature `selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[243, 153, 92, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "selfPermit",
        abi = "selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitCall {
        pub token: Address,
        pub value: U256,
        pub deadline: U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }

    #[doc = "Container type for all input parameters for the `selfPermitAllowed`function with signature `selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[70, 89, 164, 148]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "selfPermitAllowed",
        abi = "selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedCall {
        pub token: Address,
        pub nonce: U256,
        pub expiry: U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }

    #[doc = "Container type for all input parameters for the `selfPermitAllowedIfNecessary`function with signature `selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[164, 167, 143, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "selfPermitAllowedIfNecessary",
        abi = "selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedIfNecessaryCall {
        pub token: Address,
        pub nonce: U256,
        pub expiry: U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }

    #[doc = "Container type for all input parameters for the `selfPermitIfNecessary`function with signature `selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[194, 227, 20, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "selfPermitIfNecessary",
        abi = "selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitIfNecessaryCall {
        pub token: Address,
        pub value: U256,
        pub deadline: U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }

    #[doc = "Container type for all input parameters for the `sweepToken`function with signature `sweepToken(address,uint256,address)` and selector `[223, 42, 181, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(name = "sweepToken", abi = "sweepToken(address,uint256,address)")]
    pub struct SweepTokenCall {
        pub token: Address,
        pub amount_minimum: U256,
        pub recipient: Address,
    }

    #[doc = "Container type for all input parameters for the `sweepTokenWithFee`function with signature `sweepTokenWithFee(address,uint256,address,uint256,address)` and selector `[224, 225, 137, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "sweepTokenWithFee",
        abi = "sweepTokenWithFee(address,uint256,address,uint256,address)"
    )]
    pub struct SweepTokenWithFeeCall {
        pub token: Address,
        pub amount_minimum: U256,
        pub recipient: Address,
        pub fee_bips: U256,
        pub fee_recipient: Address,
    }

    #[doc = "Container type for all input parameters for the `uniswapV3SwapCallback`function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `[250, 70, 30, 51]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: I256,
        pub amount_1_delta: I256,
        pub data: Bytes,
    }

    #[doc = "Container type for all input parameters for the `unwrapWETH9`function with signature `unwrapWETH9(uint256,address)` and selector `[73, 64, 75, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(name = "unwrapWETH9", abi = "unwrapWETH9(uint256,address)")]
    pub struct UnwrapWETH9Call {
        pub amount_minimum: U256,
        pub recipient: Address,
    }

    #[doc = "Container type for all input parameters for the `unwrapWETH9WithFee`function with signature `unwrapWETH9WithFee(uint256,address,uint256,address)` and selector `[155, 44, 10, 55]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers::contract::EthCall,
        ethers::contract::EthDisplay,
    )]
    #[ethcall(
        name = "unwrapWETH9WithFee",
        abi = "unwrapWETH9WithFee(uint256,address,uint256,address)"
    )]
    pub struct UnwrapWETH9WithFeeCall {
        pub amount_minimum: U256,
        pub recipient: Address,
        pub fee_bips: U256,
        pub fee_recipient: Address,
    }

    #[derive(Debug, Clone, PartialEq, Eq, ethers::contract::EthAbiType)]
    pub enum UniswapV3Calls {
        Weth9(Weth9Call),
        ExactInput(ExactInputCall),
        ExactInputSingle(ExactInputSingleCall),
        ExactOutput(ExactOutputCall),
        ExactOutputSingle(ExactOutputSingleCall),
        Factory(FactoryCall),
        Multicall(MulticallCall),
        RefundETH(RefundETHCall),
        SelfPermit(SelfPermitCall),
        SelfPermitAllowed(SelfPermitAllowedCall),
        SelfPermitAllowedIfNecessary(SelfPermitAllowedIfNecessaryCall),
        SelfPermitIfNecessary(SelfPermitIfNecessaryCall),
        SweepToken(SweepTokenCall),
        SweepTokenWithFee(SweepTokenWithFeeCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
        UnwrapWETH9(UnwrapWETH9Call),
        UnwrapWETH9WithFee(UnwrapWETH9WithFeeCall),
    }

    impl ethers::core::abi::AbiDecode for UniswapV3Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <Weth9Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Weth9(decoded));
            }
            if let Ok(decoded) =
                <ExactInputCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::ExactInput(decoded));
            }
            if let Ok(decoded) =
                <ExactInputSingleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::ExactInputSingle(decoded));
            }
            if let Ok(decoded) =
                <ExactOutputCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::ExactOutput(decoded));
            }
            if let Ok(decoded) =
                <ExactOutputSingleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::ExactOutputSingle(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Factory(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::Multicall(decoded));
            }
            if let Ok(decoded) =
                <RefundETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::RefundETH(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::SelfPermit(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::SelfPermitAllowed(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedIfNecessaryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV3Calls::SelfPermitAllowedIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitIfNecessaryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::SelfPermitIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SweepTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::SweepToken(decoded));
            }
            if let Ok(decoded) =
                <SweepTokenWithFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::SweepTokenWithFee(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3SwapCallbackCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::UniswapV3SwapCallback(decoded));
            }
            if let Ok(decoded) =
                <UnwrapWETH9Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::UnwrapWETH9(decoded));
            }
            if let Ok(decoded) =
                <UnwrapWETH9WithFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3Calls::UnwrapWETH9WithFee(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }

    impl ethers::core::abi::AbiEncode for UniswapV3Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapV3Calls::Weth9(element) => element.encode(),
                UniswapV3Calls::ExactInput(element) => element.encode(),
                UniswapV3Calls::ExactInputSingle(element) => element.encode(),
                UniswapV3Calls::ExactOutput(element) => element.encode(),
                UniswapV3Calls::ExactOutputSingle(element) => element.encode(),
                UniswapV3Calls::Factory(element) => element.encode(),
                UniswapV3Calls::Multicall(element) => element.encode(),
                UniswapV3Calls::RefundETH(element) => element.encode(),
                UniswapV3Calls::SelfPermit(element) => element.encode(),
                UniswapV3Calls::SelfPermitAllowed(element) => element.encode(),
                UniswapV3Calls::SelfPermitAllowedIfNecessary(element) => element.encode(),
                UniswapV3Calls::SelfPermitIfNecessary(element) => element.encode(),
                UniswapV3Calls::SweepToken(element) => element.encode(),
                UniswapV3Calls::SweepTokenWithFee(element) => element.encode(),
                UniswapV3Calls::UniswapV3SwapCallback(element) => element.encode(),
                UniswapV3Calls::UnwrapWETH9(element) => element.encode(),
                UniswapV3Calls::UnwrapWETH9WithFee(element) => element.encode(),
            }
        }
    }

    impl std::fmt::Display for UniswapV3Calls {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                UniswapV3Calls::Weth9(element) => element.fmt(f),
                UniswapV3Calls::ExactInput(element) => element.fmt(f),
                UniswapV3Calls::ExactInputSingle(element) => element.fmt(f),
                UniswapV3Calls::ExactOutput(element) => element.fmt(f),
                UniswapV3Calls::ExactOutputSingle(element) => element.fmt(f),
                UniswapV3Calls::Factory(element) => element.fmt(f),
                UniswapV3Calls::Multicall(element) => element.fmt(f),
                UniswapV3Calls::RefundETH(element) => element.fmt(f),
                UniswapV3Calls::SelfPermit(element) => element.fmt(f),
                UniswapV3Calls::SelfPermitAllowed(element) => element.fmt(f),
                UniswapV3Calls::SelfPermitAllowedIfNecessary(element) => element.fmt(f),
                UniswapV3Calls::SelfPermitIfNecessary(element) => element.fmt(f),
                UniswapV3Calls::SweepToken(element) => element.fmt(f),
                UniswapV3Calls::SweepTokenWithFee(element) => element.fmt(f),
                UniswapV3Calls::UniswapV3SwapCallback(element) => element.fmt(f),
                UniswapV3Calls::UnwrapWETH9(element) => element.fmt(f),
                UniswapV3Calls::UnwrapWETH9WithFee(element) => element.fmt(f),
            }
        }
    }

    impl From<Weth9Call> for UniswapV3Calls {
        fn from(var: Weth9Call) -> Self {
            UniswapV3Calls::Weth9(var)
        }
    }

    impl From<ExactInputCall> for UniswapV3Calls {
        fn from(var: ExactInputCall) -> Self {
            UniswapV3Calls::ExactInput(var)
        }
    }

    impl From<ExactInputSingleCall> for UniswapV3Calls {
        fn from(var: ExactInputSingleCall) -> Self {
            UniswapV3Calls::ExactInputSingle(var)
        }
    }

    impl From<ExactOutputCall> for UniswapV3Calls {
        fn from(var: ExactOutputCall) -> Self {
            UniswapV3Calls::ExactOutput(var)
        }
    }

    impl From<ExactOutputSingleCall> for UniswapV3Calls {
        fn from(var: ExactOutputSingleCall) -> Self {
            UniswapV3Calls::ExactOutputSingle(var)
        }
    }

    impl From<FactoryCall> for UniswapV3Calls {
        fn from(var: FactoryCall) -> Self {
            UniswapV3Calls::Factory(var)
        }
    }

    impl From<MulticallCall> for UniswapV3Calls {
        fn from(var: MulticallCall) -> Self {
            UniswapV3Calls::Multicall(var)
        }
    }

    impl From<RefundETHCall> for UniswapV3Calls {
        fn from(var: RefundETHCall) -> Self {
            UniswapV3Calls::RefundETH(var)
        }
    }

    impl From<SelfPermitCall> for UniswapV3Calls {
        fn from(var: SelfPermitCall) -> Self {
            UniswapV3Calls::SelfPermit(var)
        }
    }

    impl From<SelfPermitAllowedCall> for UniswapV3Calls {
        fn from(var: SelfPermitAllowedCall) -> Self {
            UniswapV3Calls::SelfPermitAllowed(var)
        }
    }

    impl From<SelfPermitAllowedIfNecessaryCall> for UniswapV3Calls {
        fn from(var: SelfPermitAllowedIfNecessaryCall) -> Self {
            UniswapV3Calls::SelfPermitAllowedIfNecessary(var)
        }
    }

    impl From<SelfPermitIfNecessaryCall> for UniswapV3Calls {
        fn from(var: SelfPermitIfNecessaryCall) -> Self {
            UniswapV3Calls::SelfPermitIfNecessary(var)
        }
    }

    impl From<SweepTokenCall> for UniswapV3Calls {
        fn from(var: SweepTokenCall) -> Self {
            UniswapV3Calls::SweepToken(var)
        }
    }

    impl From<SweepTokenWithFeeCall> for UniswapV3Calls {
        fn from(var: SweepTokenWithFeeCall) -> Self {
            UniswapV3Calls::SweepTokenWithFee(var)
        }
    }

    impl From<UniswapV3SwapCallbackCall> for UniswapV3Calls {
        fn from(var: UniswapV3SwapCallbackCall) -> Self {
            UniswapV3Calls::UniswapV3SwapCallback(var)
        }
    }

    impl From<UnwrapWETH9Call> for UniswapV3Calls {
        fn from(var: UnwrapWETH9Call) -> Self {
            UniswapV3Calls::UnwrapWETH9(var)
        }
    }

    impl From<UnwrapWETH9WithFeeCall> for UniswapV3Calls {
        fn from(var: UnwrapWETH9WithFeeCall) -> Self {
            UniswapV3Calls::UnwrapWETH9WithFee(var)
        }
    }

    #[doc = "`ExactInputParams(bytes,address,uint256,uint256,uint256)`"]
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers::contract::EthAbiType)]
    pub struct ExactInputParams {
        pub path: Bytes,
        pub recipient: Address,
        pub deadline: U256,
        pub amount_in: U256,
        pub amount_out_minimum: U256,
    }

    #[doc = "`ExactInputSingleParams(address,address,uint24,address,uint256,uint256,uint256,uint160)`"]
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers::contract::EthAbiType)]
    pub struct ExactInputSingleParams {
        pub token_in: Address,
        pub token_out: Address,
        pub fee: u32,
        pub recipient: Address,
        pub deadline: U256,
        pub amount_in: U256,
        pub amount_out_minimum: U256,
        pub sqrt_price_limit_x96: U256,
    }

    #[doc = "`ExactOutputParams(bytes,address,uint256,uint256,uint256)`"]
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers::contract::EthAbiType)]
    pub struct ExactOutputParams {
        pub path: Bytes,
        pub recipient: Address,
        pub deadline: U256,
        pub amount_out: U256,
        pub amount_in_maximum: U256,
    }

    #[doc = "`ExactOutputSingleParams(address,address,uint24,address,uint256,uint256,uint256,uint160)`"]
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers::contract::EthAbiType)]
    pub struct ExactOutputSingleParams {
        pub token_in: Address,
        pub token_out: Address,
        pub fee: u32,
        pub recipient: Address,
        pub deadline: U256,
        pub amount_out: U256,
        pub amount_in_maximum: U256,
        pub sqrt_price_limit_x96: U256,
    }
}
