pub use emodelogic_mod::*;
#[allow(clippy::too_many_arguments)]
mod emodelogic_mod {
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
    #[doc = "EModeLogic was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static EMODELOGIC_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"categoryId\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UserEModeSet\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static EMODELOGIC_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x61104b61003a600b82828239805160001a60731461002d57634e487b7160e01b600052600060045260246000fd5b30600052607381538281f3fe73000000000000000000000000000000000000000030146080604052600436106100355760003560e01c80635d5dc3131461003a575b600080fd5b81801561004657600080fd5b5061005a610055366004610e3b565b61005c565b005b60408051602081018252835481528251918301516100809289928992899290610127565b336000908152602084905260409081902080549183015160ff90811660ff198416179091551680156100e0576100dd878787866040518060200160405290816000820154815250503387604001518860000151896020015161024b565b50505b604080830151905160ff909116815233907fd728da875fc88944cbf17638bcbe4af0eedaef63becd1d1c57cc097eb4608d849060200160405180910390a250505050505050565b60ff81161580610152575060ff811660009081526020859052604090205462010000900461ffff1615155b6040518060400160405280600281526020016106a760f31b815250906101945760405162461bcd60e51b815260040161018b9190610f0b565b60405180910390fd5b5082516101a057610243565b60ff8116156102435760005b82811015610241576101be84826102f7565b1561023957600081815260208781526040808320546001600160a01b03168352898252918290208251918201909252905480825260ff8481169160a81c16146040518060400160405280600281526020016106a760f31b815250906102365760405162461bcd60e51b815260040161018b9190610f0b565b50505b6001016101ac565b505b505050505050565b6000806000806102988c8c8c6040518060a001604052808e81526020018b81526020018d6001600160a01b031681526020018a6001600160a01b031681526020018c60ff16815250610344565b9550955050505050670de0b6b3a764000082101560405180604001604052806002815260200161333560f01b815250906102e55760405162461bcd60e51b815260040161018b9190610f0b565b50909b909a5098505050505050505050565b6040805180820190915260028152610dcd60f21b6020820152600090608083106103345760405162461bcd60e51b815260040161018b9190610f0b565b50509051600191821b1c16151590565b60008060008060008061035a8760000151511590565b15610378575060009450849350839250829150600019905081610818565b61041a6040518061026001604052806000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160008152602001600081526020016000815260200160006001600160a01b031681526020016000151581526020016000151581525090565b608088015160ff161561045f57608088015160ff16600090815260208a905260409020606089015161044c9190610825565b6101808401526101c08301526101a08201525b87602001518160c00151101561073e5760c0810151885161047f916108d1565b6104935760c081018051600101905261045f565b60c0810151600090815260208b905260409020546001600160a01b031661020082018190526104cc5760c081018051600101905261045f565b6102008101516001600160a01b0316600090815260208c8152604091829020825180830190935280549283905260ff60a884901c81166101e0860152603084901c166060850181905261ffff601085901c811660a08701529093166080850152600a9290920a90830152610180820151158015906105555750816101e00151896080015160ff16145b6105d357606089015161020083015160405163b3596f0760e01b81526001600160a01b03918216600482015291169063b3596f0790602401602060405180830381865afa1580156105aa573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105ce9190610f60565b6105da565b8161018001515b825260a0820151158015906105fa575060c082015189516105fa91610921565b156106ea5761061789604001518284600001518560200151610970565b6040830181905261010083018051610630908390610f8f565b90525060808901516101e083015161064b9160ff1690610a29565b15156102408301526080820151156106a157816102400151610671578160800151610678565b816101a001515b82604001516106879190610fa7565b82610140018181516106999190610f8f565b9052506106aa565b60016102208301525b8161024001516106be578160a001516106c5565b816101c001515b82604001516106d49190610fa7565b82610160018181516106e69190610f8f565b9052505b60c082015189516106fa916102f7565b1561072d5761071789604001518284600001518560200151610a40565b82610120018181516107299190610f8f565b9052505b5060c081018051600101905261045f565b61010081015161074f57600061076a565b8061010001518161014001518161076857610768610fc6565b045b61014082015261010081015161078157600061079c565b8061010001518161016001518161079a5761079a610fc6565b045b610160820152610120810151156107de576107d98161012001516107d3836101600151846101000151610b7490919063ffffffff16565b90610b9a565b6107e2565b6000195b60e0820181905261010082015161012083015161014084015161016085015161022090950151929a509098509650919450925090505b9499939850945094509450565b8154600090819081908190660100000000000090046001600160a01b031680156108b65760405163b3596f0760e01b81526001600160a01b03828116600483015287169063b3596f0790602401602060405180830381865afa15801561088f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108b39190610f60565b91505b50945461ffff80821697620100009092041695945092505050565b6040805180820190915260028152610dcd60f21b60208201526000906080831061090e5760405162461bcd60e51b815260040161018b9190610f0b565b5050905160019190911b1c600316151590565b6040805180820190915260028152610dcd60f21b60208201526000906080831061095e5760405162461bcd60e51b815260040161018b9190610f0b565b50509051600191821b82011c16151590565b60008061097c85610bd1565b600486810154604051630ed1279f60e11b81526001600160a01b038a8116938201939093529293506000928792610a02928692911690631da24f3e90602401602060405180830381865afa1580156109d8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109fc9190610f60565b90610c29565b610a0c9190610fa7565b9050838181610a1d57610a1d610fc6565b04979650505050505050565b60008215801590610a3957508282145b9392505050565b6006830154604051630ed1279f60e11b81526001600160a01b0386811660048301526000928392911690631da24f3e90602401602060405180830381865afa158015610a90573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ab49190610f60565b90508015610ad257610acf610ac886610c6d565b8290610c29565b90505b60058501546040516370a0823160e01b81526001600160a01b038881166004830152909116906370a0823190602401602060405180830381865afa158015610b1e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b429190610f60565b610b4c9082610f8f565b9050610b588185610fa7565b9050828181610b6957610b69610fc6565b049695505050505050565b600081156113881983900484111517610b8c57600080fd5b506127109102611388010490565b60008115670de0b6b3a764000060028404190484111715610bba57600080fd5b50670de0b6b3a76400009190910260028204010490565b6003810154600090600160801b900464ffffffffff1642811415610c01575050600101546001600160801b031690565b6001830154610a39906001600160801b03808216916109fc91600160801b9091041684610cc5565b600081156b019d971e4fe8401e740000001983900484111517610c4b57600080fd5b506b033b2e3c9fd0803ce800000091026b019d971e4fe8401e74000000010490565b6003810154600090600160801b900464ffffffffff1642811415610c9d575050600201546001600160801b031690565b6002830154610a39906001600160801b03808216916109fc91600160801b9091041684610d0a565b600080610cd964ffffffffff841642610fdc565b610ce39085610fa7565b6301e1338090049050610d02816b033b2e3c9fd0803ce8000000610f8f565b949350505050565b6000610a39838342600080610d2664ffffffffff851684610fdc565b905080610d42576b033b2e3c9fd0803ce8000000915050610a39565b60001981016000808060028511610d5a576000610d5f565b600285035b925066038882915c4000610d738a80610c29565b81610d8057610d80610fc6565b0491506301e13380610d92838b610c29565b81610d9f57610d9f610fc6565b049050600082610daf8688610fa7565b610db99190610fa7565b60029004905060008285610dcd888a610fa7565b610dd79190610fa7565b610de19190610fa7565b60069004905080826301e13380610df88a8f610fa7565b610e029190610ff3565b610e18906b033b2e3c9fd0803ce8000000610f8f565b610e229190610f8f565b610e2c9190610f8f565b9b9a5050505050505050505050565b600080600080600080868803610100811215610e5657600080fd5b873596506020880135955060408801359450606080890135945060808901359350609f1982011215610e8757600080fd5b506040516060810181811067ffffffffffffffff82111715610eb957634e487b7160e01b600052604160045260246000fd5b60405260a0880135815260c08801356001600160a01b0381168114610edd57600080fd5b602082015260e088013560ff81168114610ef657600080fd5b80604083015250809150509295509295509295565b600060208083528351808285015260005b81811015610f3857858101830151858201604001528201610f1c565b81811115610f4a576000604083870101525b50601f01601f1916929092016040019392505050565b600060208284031215610f7257600080fd5b5051919050565b634e487b7160e01b600052601160045260246000fd5b60008219821115610fa257610fa2610f79565b500190565b6000816000190483118215151615610fc157610fc1610f79565b500290565b634e487b7160e01b600052601260045260246000fd5b600082821015610fee57610fee610f79565b500390565b60008261101057634e487b7160e01b600052601260045260246000fd5b50049056fea2646970667358221220261d310b1c3636368ae1797ef8ee2e6f685a906915ad31c50581c77f2a01787864736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct EModeLogic<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for EModeLogic<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for EModeLogic<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(EModeLogic))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> EModeLogic<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), EMODELOGIC_ABI.clone(), client).into()
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
                EMODELOGIC_ABI.clone(),
                EMODELOGIC_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Gets the contract's `UserEModeSet` event"]
        pub fn user_e_mode_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UserEModeSetFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UserEModeSetFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for EModeLogic<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "UserEModeSet", abi = "UserEModeSet(address,uint8)")]
    pub struct UserEModeSetFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub category_id: u8,
    }
}
