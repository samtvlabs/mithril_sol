pub use verify::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod verify {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("verify_stm"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify_stm"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("m"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("k"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("phi_f"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ms"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static VERIFY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02n\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80cE\x1F\x10c\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01UV[a\0WV[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0\x80\x86\x86\x86\x86\x86`@Q` \x01a\0s\x95\x94\x93\x92\x91\x90a\x02\x06V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\x10\x90P`@Q\x83\x81R` \x81\x85Q\x83\x85Z\xFA\x80a\0\xA3W=\x82\xFD[PQ\x99\x98PPPPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\0\xD9W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xF4Wa\0\xF4a\0\xB2V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x01\x1CWa\x01\x1Ca\0\xB2V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x015W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x01mW`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\x9AW`\0\x80\xFD[a\x01\xA6\x89\x83\x8A\x01a\0\xC8V[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a\x01\xBCW`\0\x80\xFD[Pa\x01\xC9\x88\x82\x89\x01a\0\xC8V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q`\0[\x81\x81\x10\x15a\x01\xF7W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x01\xDDV[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[\x85\x81R\x84` \x82\x01R\x83`@\x82\x01R`\0a\x02-a\x02'``\x84\x01\x86a\x01\xD6V[\x84a\x01\xD6V[\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 }\xD0\xF7.>\xCF\xFC\xF7#\xD2\xB4S\x991O\xDDv\x0E\xF4\x8A\x10~\xBA}2\xF2\xCD}\x85\xB5K\tdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static VERIFY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80cE\x1F\x10c\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01UV[a\0WV[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0\x80\x86\x86\x86\x86\x86`@Q` \x01a\0s\x95\x94\x93\x92\x91\x90a\x02\x06V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\x10\x90P`@Q\x83\x81R` \x81\x85Q\x83\x85Z\xFA\x80a\0\xA3W=\x82\xFD[PQ\x99\x98PPPPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\0\xD9W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xF4Wa\0\xF4a\0\xB2V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x01\x1CWa\x01\x1Ca\0\xB2V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x015W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x01mW`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\x9AW`\0\x80\xFD[a\x01\xA6\x89\x83\x8A\x01a\0\xC8V[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a\x01\xBCW`\0\x80\xFD[Pa\x01\xC9\x88\x82\x89\x01a\0\xC8V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q`\0[\x81\x81\x10\x15a\x01\xF7W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x01\xDDV[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[\x85\x81R\x84` \x82\x01R\x83`@\x82\x01R`\0a\x02-a\x02'``\x84\x01\x86a\x01\xD6V[\x84a\x01\xD6V[\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 }\xD0\xF7.>\xCF\xFC\xF7#\xD2\xB4S\x991O\xDDv\x0E\xF4\x8A\x10~\xBA}2\xF2\xCD}\x85\xB5K\tdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static VERIFY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Verify<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Verify<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Verify<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Verify<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Verify<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Verify)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Verify<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VERIFY_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                VERIFY_ABI.clone(),
                VERIFY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `verify_stm` (0x451f1063) function
        pub fn verify_stm(
            &self,
            m: ::ethers::core::types::U256,
            k: ::ethers::core::types::U256,
            phi_f: ::ethers::core::types::U256,
            ms: ::ethers::core::types::Bytes,
            msig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([69, 31, 16, 99], (m, k, phi_f, ms, msig))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Verify<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `verify_stm` function with signature `verify_stm(uint256,uint256,uint256,bytes,bytes)` and selector `0x451f1063`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "verify_stm",
        abi = "verify_stm(uint256,uint256,uint256,bytes,bytes)"
    )]
    pub struct VerifyStmCall {
        pub m: ::ethers::core::types::U256,
        pub k: ::ethers::core::types::U256,
        pub phi_f: ::ethers::core::types::U256,
        pub ms: ::ethers::core::types::Bytes,
        pub msig: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `verify_stm` function with signature `verify_stm(uint256,uint256,uint256,bytes,bytes)` and selector `0x451f1063`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VerifyStmReturn(pub bool);
}
