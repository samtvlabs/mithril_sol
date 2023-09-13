pub use bonsai_low_level_callback_receiver::*;
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
pub mod bonsai_low_level_callback_receiver {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("bonsaiLowLevelCallbackReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "bonsaiLowLevelCallbackReceiver",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bonsaiRelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bonsaiRelay"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IBonsaiRelay"),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("UnauthorizedCallbackSource"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UnauthorizedCallbackSource",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expected"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IBonsaiRelay"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("found"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IBonsaiRelay"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnexpectedImageId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnexpectedImageId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expected"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("found"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BONSAILOWLEVELCALLBACKRECEIVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct BonsaiLowLevelCallbackReceiver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BonsaiLowLevelCallbackReceiver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BonsaiLowLevelCallbackReceiver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BonsaiLowLevelCallbackReceiver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BonsaiLowLevelCallbackReceiver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BonsaiLowLevelCallbackReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BonsaiLowLevelCallbackReceiver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BONSAILOWLEVELCALLBACKRECEIVER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `bonsaiLowLevelCallbackReceiver` (0xae4ddde9) function
        pub fn bonsai_low_level_callback_receiver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([174, 77, 221, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bonsaiRelay` (0xe70ffd4b) function
        pub fn bonsai_relay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([231, 15, 253, 75], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BonsaiLowLevelCallbackReceiver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `UnauthorizedCallbackSource` with signature `UnauthorizedCallbackSource(address,address)` and selector `0x865c066e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "UnauthorizedCallbackSource",
        abi = "UnauthorizedCallbackSource(address,address)"
    )]
    pub struct UnauthorizedCallbackSource {
        pub expected: ::ethers::core::types::Address,
        pub found: ::ethers::core::types::Address,
    }
    ///Custom Error type `UnexpectedImageId` with signature `UnexpectedImageId(bytes32,bytes32)` and selector `0xbe5472b0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnexpectedImageId", abi = "UnexpectedImageId(bytes32,bytes32)")]
    pub struct UnexpectedImageId {
        pub expected: [u8; 32],
        pub found: [u8; 32],
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BonsaiLowLevelCallbackReceiverErrors {
        UnauthorizedCallbackSource(UnauthorizedCallbackSource),
        UnexpectedImageId(UnexpectedImageId),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BonsaiLowLevelCallbackReceiverErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <UnauthorizedCallbackSource as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnauthorizedCallbackSource(decoded));
            }
            if let Ok(decoded)
                = <UnexpectedImageId as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnexpectedImageId(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BonsaiLowLevelCallbackReceiverErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::UnauthorizedCallbackSource(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnexpectedImageId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BonsaiLowLevelCallbackReceiverErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <UnauthorizedCallbackSource as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnexpectedImageId as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for BonsaiLowLevelCallbackReceiverErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::UnauthorizedCallbackSource(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnexpectedImageId(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for BonsaiLowLevelCallbackReceiverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<UnauthorizedCallbackSource>
    for BonsaiLowLevelCallbackReceiverErrors {
        fn from(value: UnauthorizedCallbackSource) -> Self {
            Self::UnauthorizedCallbackSource(value)
        }
    }
    impl ::core::convert::From<UnexpectedImageId>
    for BonsaiLowLevelCallbackReceiverErrors {
        fn from(value: UnexpectedImageId) -> Self {
            Self::UnexpectedImageId(value)
        }
    }
    ///Container type for all input parameters for the `bonsaiLowLevelCallbackReceiver` function with signature `bonsaiLowLevelCallbackReceiver()` and selector `0xae4ddde9`
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
        name = "bonsaiLowLevelCallbackReceiver",
        abi = "bonsaiLowLevelCallbackReceiver()"
    )]
    pub struct BonsaiLowLevelCallbackReceiverCall;
    ///Container type for all input parameters for the `bonsaiRelay` function with signature `bonsaiRelay()` and selector `0xe70ffd4b`
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
    #[ethcall(name = "bonsaiRelay", abi = "bonsaiRelay()")]
    pub struct BonsaiRelayCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BonsaiLowLevelCallbackReceiverCalls {
        BonsaiLowLevelCallbackReceiver(BonsaiLowLevelCallbackReceiverCall),
        BonsaiRelay(BonsaiRelayCall),
    }
    impl ::ethers::core::abi::AbiDecode for BonsaiLowLevelCallbackReceiverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BonsaiLowLevelCallbackReceiverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BonsaiLowLevelCallbackReceiver(decoded));
            }
            if let Ok(decoded)
                = <BonsaiRelayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BonsaiRelay(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BonsaiLowLevelCallbackReceiverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BonsaiLowLevelCallbackReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BonsaiRelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BonsaiLowLevelCallbackReceiverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BonsaiLowLevelCallbackReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BonsaiRelay(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BonsaiLowLevelCallbackReceiverCall>
    for BonsaiLowLevelCallbackReceiverCalls {
        fn from(value: BonsaiLowLevelCallbackReceiverCall) -> Self {
            Self::BonsaiLowLevelCallbackReceiver(value)
        }
    }
    impl ::core::convert::From<BonsaiRelayCall> for BonsaiLowLevelCallbackReceiverCalls {
        fn from(value: BonsaiRelayCall) -> Self {
            Self::BonsaiRelay(value)
        }
    }
    ///Container type for all return fields from the `bonsaiLowLevelCallbackReceiver` function with signature `bonsaiLowLevelCallbackReceiver()` and selector `0xae4ddde9`
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
    pub struct BonsaiLowLevelCallbackReceiverReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `bonsaiRelay` function with signature `bonsaiRelay()` and selector `0xe70ffd4b`
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
    pub struct BonsaiRelayReturn(pub ::ethers::core::types::Address);
}
