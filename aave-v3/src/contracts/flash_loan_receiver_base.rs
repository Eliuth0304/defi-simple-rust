pub use flashloanreceiverbase_mod::*;
#[allow(clippy::too_many_arguments)]
mod flashloanreceiverbase_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "FlashLoanReceiverBase was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static FLASHLOANRECEIVERBASE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ADDRESSES_PROVIDER\",\"outputs\":[{\"internalType\":\"contract IPoolAddressesProvider\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"POOL\",\"outputs\":[{\"internalType\":\"contract IPool\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"assets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"premiums\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"initiator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeOperation\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static FLASHLOANRECEIVERBASE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| "0x".parse().expect("invalid bytecode"));
    #[derive(Clone)]
    pub struct FlashLoanReceiverBase<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for FlashLoanReceiverBase<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for FlashLoanReceiverBase<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FlashLoanReceiverBase))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> FlashLoanReceiverBase<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                FLASHLOANRECEIVERBASE_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                FLASHLOANRECEIVERBASE_ABI.clone(),
                FLASHLOANRECEIVERBASE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `ADDRESSES_PROVIDER` (0x0542975c) function"]
        pub fn addresses_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([5, 66, 151, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL` (0x7535d246) function"]
        pub fn pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeOperation` (0x920f5c84) function"]
        pub fn execute_operation(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            premiums: ::std::vec::Vec<ethers::core::types::U256>,
            initiator: ethers::core::types::Address,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [146, 15, 92, 132],
                    (assets, amounts, premiums, initiator, params),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for FlashLoanReceiverBase<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `ADDRESSES_PROVIDER`function with signature `ADDRESSES_PROVIDER()` and selector `[5, 66, 151, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ADDRESSES_PROVIDER", abi = "ADDRESSES_PROVIDER()")]
    pub struct AddressesProviderCall;
    #[doc = "Container type for all input parameters for the `POOL`function with signature `POOL()` and selector `[117, 53, 210, 70]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
    #[doc = "Container type for all input parameters for the `executeOperation`function with signature `executeOperation(address[],uint256[],uint256[],address,bytes)` and selector `[146, 15, 92, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "executeOperation",
        abi = "executeOperation(address[],uint256[],uint256[],address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub premiums: ::std::vec::Vec<ethers::core::types::U256>,
        pub initiator: ethers::core::types::Address,
        pub params: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum FlashLoanReceiverBaseCalls {
        AddressesProvider(AddressesProviderCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
    }
    impl ethers::core::abi::AbiDecode for FlashLoanReceiverBaseCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FlashLoanReceiverBaseCalls::AddressesProvider(decoded));
            }
            if let Ok(decoded) = <PoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(FlashLoanReceiverBaseCalls::Pool(decoded));
            }
            if let Ok(decoded) =
                <ExecuteOperationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FlashLoanReceiverBaseCalls::ExecuteOperation(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FlashLoanReceiverBaseCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                FlashLoanReceiverBaseCalls::AddressesProvider(element) => element.encode(),
                FlashLoanReceiverBaseCalls::Pool(element) => element.encode(),
                FlashLoanReceiverBaseCalls::ExecuteOperation(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for FlashLoanReceiverBaseCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FlashLoanReceiverBaseCalls::AddressesProvider(element) => element.fmt(f),
                FlashLoanReceiverBaseCalls::Pool(element) => element.fmt(f),
                FlashLoanReceiverBaseCalls::ExecuteOperation(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressesProviderCall> for FlashLoanReceiverBaseCalls {
        fn from(var: AddressesProviderCall) -> Self {
            FlashLoanReceiverBaseCalls::AddressesProvider(var)
        }
    }
    impl ::std::convert::From<PoolCall> for FlashLoanReceiverBaseCalls {
        fn from(var: PoolCall) -> Self {
            FlashLoanReceiverBaseCalls::Pool(var)
        }
    }
    impl ::std::convert::From<ExecuteOperationCall> for FlashLoanReceiverBaseCalls {
        fn from(var: ExecuteOperationCall) -> Self {
            FlashLoanReceiverBaseCalls::ExecuteOperation(var)
        }
    }
}
