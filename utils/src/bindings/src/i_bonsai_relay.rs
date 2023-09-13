pub use i_bonsai_relay::*;
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
pub mod i_bonsai_relay {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("callbackIsAuthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "callbackIsAuthorized",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("journal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("auth"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct CallbackAuthorization",
                                        ),
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
                (
                    ::std::borrow::ToOwned::to_owned("invokeCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invokeCallback"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callback"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Callback"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("invokeCallbacks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invokeCallbacks"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callbacks"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Callback[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invocationResults"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requestCallback"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callbackContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("functionSelector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CallbackRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CallbackRequest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("imageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("callbackContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("functionSelector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gasLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IBONSAIRELAY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IBonsaiRelay<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IBonsaiRelay<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IBonsaiRelay<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IBonsaiRelay<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IBonsaiRelay<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IBonsaiRelay))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IBonsaiRelay<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IBONSAIRELAY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `callbackIsAuthorized` (0x59611e67) function
        pub fn callback_is_authorized(
            &self,
            image_id: [u8; 32],
            journal: ::ethers::core::types::Bytes,
            auth: CallbackAuthorization,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([89, 97, 30, 103], (image_id, journal, auth))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invokeCallback` (0x83a19d39) function
        pub fn invoke_callback(
            &self,
            callback: Callback,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 161, 157, 57], (callback,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invokeCallbacks` (0x13ef1412) function
        pub fn invoke_callbacks(
            &self,
            callbacks: ::std::vec::Vec<Callback>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([19, 239, 20, 18], callbacks)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestCallback` (0xe80802a2) function
        pub fn request_callback(
            &self,
            image_id: [u8; 32],
            input: ::ethers::core::types::Bytes,
            callback_contract: ::ethers::core::types::Address,
            function_selector: [u8; 4],
            gas_limit: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [232, 8, 2, 162],
                    (image_id, input, callback_contract, function_selector, gas_limit),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CallbackRequest` event
        pub fn callback_request_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CallbackRequestFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CallbackRequestFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IBonsaiRelay<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CallbackRequest",
        abi = "CallbackRequest(address,bytes32,bytes,address,bytes4,uint64)"
    )]
    pub struct CallbackRequestFilter {
        pub account: ::ethers::core::types::Address,
        pub image_id: [u8; 32],
        pub input: ::ethers::core::types::Bytes,
        pub callback_contract: ::ethers::core::types::Address,
        pub function_selector: [u8; 4],
        pub gas_limit: u64,
    }
    ///Container type for all input parameters for the `callbackIsAuthorized` function with signature `callbackIsAuthorized(bytes32,bytes,(bytes,bytes32))` and selector `0x59611e67`
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
        name = "callbackIsAuthorized",
        abi = "callbackIsAuthorized(bytes32,bytes,(bytes,bytes32))"
    )]
    pub struct CallbackIsAuthorizedCall {
        pub image_id: [u8; 32],
        pub journal: ::ethers::core::types::Bytes,
        pub auth: CallbackAuthorization,
    }
    ///Container type for all input parameters for the `invokeCallback` function with signature `invokeCallback(((bytes,bytes32),address,bytes,uint64))` and selector `0x83a19d39`
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
        name = "invokeCallback",
        abi = "invokeCallback(((bytes,bytes32),address,bytes,uint64))"
    )]
    pub struct InvokeCallbackCall {
        pub callback: Callback,
    }
    ///Container type for all input parameters for the `invokeCallbacks` function with signature `invokeCallbacks(((bytes,bytes32),address,bytes,uint64)[])` and selector `0x13ef1412`
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
        name = "invokeCallbacks",
        abi = "invokeCallbacks(((bytes,bytes32),address,bytes,uint64)[])"
    )]
    pub struct InvokeCallbacksCall {
        pub callbacks: ::std::vec::Vec<Callback>,
    }
    ///Container type for all input parameters for the `requestCallback` function with signature `requestCallback(bytes32,bytes,address,bytes4,uint64)` and selector `0xe80802a2`
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
        name = "requestCallback",
        abi = "requestCallback(bytes32,bytes,address,bytes4,uint64)"
    )]
    pub struct RequestCallbackCall {
        pub image_id: [u8; 32],
        pub input: ::ethers::core::types::Bytes,
        pub callback_contract: ::ethers::core::types::Address,
        pub function_selector: [u8; 4],
        pub gas_limit: u64,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IBonsaiRelayCalls {
        CallbackIsAuthorized(CallbackIsAuthorizedCall),
        InvokeCallback(InvokeCallbackCall),
        InvokeCallbacks(InvokeCallbacksCall),
        RequestCallback(RequestCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for IBonsaiRelayCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CallbackIsAuthorizedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallbackIsAuthorized(decoded));
            }
            if let Ok(decoded)
                = <InvokeCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvokeCallback(decoded));
            }
            if let Ok(decoded)
                = <InvokeCallbacksCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvokeCallbacks(decoded));
            }
            if let Ok(decoded)
                = <RequestCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IBonsaiRelayCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CallbackIsAuthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvokeCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvokeCallbacks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IBonsaiRelayCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallbackIsAuthorized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvokeCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvokeCallbacks(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestCallback(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CallbackIsAuthorizedCall> for IBonsaiRelayCalls {
        fn from(value: CallbackIsAuthorizedCall) -> Self {
            Self::CallbackIsAuthorized(value)
        }
    }
    impl ::core::convert::From<InvokeCallbackCall> for IBonsaiRelayCalls {
        fn from(value: InvokeCallbackCall) -> Self {
            Self::InvokeCallback(value)
        }
    }
    impl ::core::convert::From<InvokeCallbacksCall> for IBonsaiRelayCalls {
        fn from(value: InvokeCallbacksCall) -> Self {
            Self::InvokeCallbacks(value)
        }
    }
    impl ::core::convert::From<RequestCallbackCall> for IBonsaiRelayCalls {
        fn from(value: RequestCallbackCall) -> Self {
            Self::RequestCallback(value)
        }
    }
    ///Container type for all return fields from the `callbackIsAuthorized` function with signature `callbackIsAuthorized(bytes32,bytes,(bytes,bytes32))` and selector `0x59611e67`
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
    pub struct CallbackIsAuthorizedReturn(pub bool);
    ///Container type for all return fields from the `invokeCallbacks` function with signature `invokeCallbacks(((bytes,bytes32),address,bytes,uint64)[])` and selector `0x13ef1412`
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
    pub struct InvokeCallbacksReturn {
        pub invocation_results: ::std::vec::Vec<bool>,
    }
}
