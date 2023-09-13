pub use strings_2::*;
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
pub mod strings_2 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("toHexString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toHexString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static STRINGS2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x049a\0:`\x0B\x82\x82\x829\x80Q`\0\x1A`s\x14a\0-WcNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[0`\0R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x005W`\x005`\xE0\x1C\x80c\xC97'\x93\x14a\0:W[`\0\x80\xFD[a\0Ma\0H6`\x04a\x02ZV[a\0cV[`@Qa\0Z\x91\x90a\x03\x0BV[`@Q\x80\x91\x03\x90\xF3[```\x01a\0t`\x02`\0\x19a\x03oV[a\0~\x91\x90a\x03\x91V[\x82Q\x10a\0\x8AW`\0\x80\xFD[\x81Qo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90`\0\x90a\0\xAF\x90`\x02a\x03\xAAV[a\0\xBA\x90`\x02a\x03\xC1V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xD2Wa\0\xD2a\x02DV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\0\xFCW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x01\x17Wa\x01\x17a\x03\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x01FWa\x01Fa\x03\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x83Q`\x02\x90`\0[\x81\x81\x10\x15a\x029W`\0\x87\x82\x81Q\x81\x10a\x01\x81Wa\x01\x81a\x03\xD4V[\x01` \x01Q`\xF8\x81\x90\x1C\x91P\x86\x90`\xFC\x1C`\x10\x81\x10a\x01\xA2Wa\x01\xA2a\x03\xD4V[\x1A`\xF8\x1B\x85\x85a\x01\xB1\x81a\x03\xEAV[\x96P\x81Q\x81\x10a\x01\xC3Wa\x01\xC3a\x03\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x85\x81`\x0F\x16`\x10\x81\x10a\x01\xEFWa\x01\xEFa\x03\xD4V[\x1A`\xF8\x1B\x85\x85a\x01\xFE\x81a\x03\xEAV[\x96P\x81Q\x81\x10a\x02\x10Wa\x02\x10a\x03\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SPP\x80a\x022\x90a\x03\xEAV[\x90Pa\x01eV[P\x91\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x02lW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x84W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x02\x98W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\xAAWa\x02\xAAa\x02DV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02\xD2Wa\x02\xD2a\x02DV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x02\xEBW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x038W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x03\x1CV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x03\x8CWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x03\xA4Wa\x03\xA4a\x03YV[\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xA4Wa\x03\xA4a\x03YV[\x80\x82\x01\x80\x82\x11\x15a\x03\xA4Wa\x03\xA4a\x03YV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x03\xFCWa\x03\xFCa\x03YV[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xCF\x10\xD0\xB9\xD0\x8A\xF7\x0C\x18\xB0\x06%i,\xA8}?=\xFF8X\r[\xB5\xA45G6\x8B\xE2\xD4\xF9dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static STRINGS2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R`\x046\x10a\x005W`\x005`\xE0\x1C\x80c\xC97'\x93\x14a\0:W[`\0\x80\xFD[a\0Ma\0H6`\x04a\x02ZV[a\0cV[`@Qa\0Z\x91\x90a\x03\x0BV[`@Q\x80\x91\x03\x90\xF3[```\x01a\0t`\x02`\0\x19a\x03oV[a\0~\x91\x90a\x03\x91V[\x82Q\x10a\0\x8AW`\0\x80\xFD[\x81Qo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90`\0\x90a\0\xAF\x90`\x02a\x03\xAAV[a\0\xBA\x90`\x02a\x03\xC1V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xD2Wa\0\xD2a\x02DV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\0\xFCW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x01\x17Wa\x01\x17a\x03\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x01FWa\x01Fa\x03\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x83Q`\x02\x90`\0[\x81\x81\x10\x15a\x029W`\0\x87\x82\x81Q\x81\x10a\x01\x81Wa\x01\x81a\x03\xD4V[\x01` \x01Q`\xF8\x81\x90\x1C\x91P\x86\x90`\xFC\x1C`\x10\x81\x10a\x01\xA2Wa\x01\xA2a\x03\xD4V[\x1A`\xF8\x1B\x85\x85a\x01\xB1\x81a\x03\xEAV[\x96P\x81Q\x81\x10a\x01\xC3Wa\x01\xC3a\x03\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x85\x81`\x0F\x16`\x10\x81\x10a\x01\xEFWa\x01\xEFa\x03\xD4V[\x1A`\xF8\x1B\x85\x85a\x01\xFE\x81a\x03\xEAV[\x96P\x81Q\x81\x10a\x02\x10Wa\x02\x10a\x03\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SPP\x80a\x022\x90a\x03\xEAV[\x90Pa\x01eV[P\x91\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x02lW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\x84W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x02\x98W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\xAAWa\x02\xAAa\x02DV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02\xD2Wa\x02\xD2a\x02DV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x02\xEBW`\0\x80\xFD[\x82` \x86\x01` \x83\x017`\0\x92\x81\x01` \x01\x92\x90\x92RP\x95\x94PPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x038W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x03\x1CV[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x03\x8CWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x03\xA4Wa\x03\xA4a\x03YV[\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xA4Wa\x03\xA4a\x03YV[\x80\x82\x01\x80\x82\x11\x15a\x03\xA4Wa\x03\xA4a\x03YV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x03\xFCWa\x03\xFCa\x03YV[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xCF\x10\xD0\xB9\xD0\x8A\xF7\x0C\x18\xB0\x06%i,\xA8}?=\xFF8X\r[\xB5\xA45G6\x8B\xE2\xD4\xF9dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static STRINGS2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Strings2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Strings2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Strings2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Strings2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Strings2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Strings2)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Strings2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STRINGS2_ABI.clone(),
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
                STRINGS2_ABI.clone(),
                STRINGS2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `toHexString` (0xc9372793) function
        pub fn to_hex_string(
            &self,
            input: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([201, 55, 39, 147], input)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Strings2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `toHexString` function with signature `toHexString(bytes)` and selector `0xc9372793`
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
    #[ethcall(name = "toHexString", abi = "toHexString(bytes)")]
    pub struct ToHexStringCall {
        pub input: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `toHexString` function with signature `toHexString(bytes)` and selector `0xc9372793`
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
    pub struct ToHexStringReturn(pub ::std::string::String);
}
