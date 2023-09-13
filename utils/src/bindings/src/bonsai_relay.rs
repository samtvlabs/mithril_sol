pub use bonsai_relay::*;
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
pub mod bonsai_relay {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("verifier_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IRiscZeroVerifier",
                            ),
                        ),
                    },
                ],
            }),
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
                    ::std::borrow::ToOwned::to_owned("parsePayload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parsePayload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static BONSAIRELAY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0B~8\x03\x80a\x0B~\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\n\xF3a\0\x8B`\09`\0a\x02\x9A\x01Ra\n\xF3`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x13\xEF\x14\x12\x14a\0\\W\x80cYa\x1Eg\x14a\0\x85W\x80c\x83\xA1\x9D9\x14a\0\xA8W\x80c\xDF\x05\\\xF1\x14a\0\xBDW\x80c\xE8\x08\x02\xA2\x14a\0\xDFW[`\0\x80\xFD[a\0oa\0j6`\x04a\x05#V[a\0\xF2V[`@Qa\0|\x91\x90a\x05\x98V[`@Q\x80\x91\x03\x90\xF3[a\0\x98a\0\x936`\x04a\x06'V[a\x02\x8EV[`@Q\x90\x15\x15\x81R` \x01a\0|V[a\0\xBBa\0\xB66`\x04a\x06\x9FV[a\x03\x8BV[\0[a\0\xD0a\0\xCB6`\x04a\x06\xE1V[a\x04\x80V[`@Qa\0|\x93\x92\x91\x90a\x07LV[a\0\xBBa\0\xED6`\x04a\x07\x9AV[a\x04\xD8V[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\rWa\x01\ra\x08'V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x016W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x02\x87W6\x84\x84\x83\x81\x81\x10a\x01WWa\x01Wa\x08=V[\x90P` \x02\x81\x01\x90a\x01i\x91\x90a\x08SV[\x90P`\x006\x81a\x01\x7Fa\0\xCB`@\x86\x01\x86a\x08sV[\x91\x94P\x92P\x90Pa\x01\x96\x83\x83\x83a\0\x93\x88\x80a\x08\xBAV[a\x01\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xB2\x90a\x08\xD0V[`@Q\x80\x91\x03\x90\xFD[a\x01\xCB`@\x85\x01` \x86\x01a\t V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\xE4`\x80\x86\x01``\x87\x01a\t;V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xFB`@\x87\x01\x87a\x08sV[`@Qa\x02\t\x92\x91\x90a\tVV[`\0`@Q\x80\x83\x03\x81`\0\x87\x87\xF1\x92PPP=\x80`\0\x81\x14a\x02GW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02LV[``\x91P[PP\x86\x86\x81Q\x81\x10a\x02`Wa\x02`a\x08=V[` \x02` \x01\x01\x81\x15\x15\x15\x15\x81RPPPPPP\x80\x80a\x02\x7F\x90a\t|V[\x91PPa\x01<V[P\x92\x91PPV[`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16cn\xFE\xF0\ta\x02\xC9\x84\x80a\x08sV[\x88\x86` \x015`\x02\x8A\x8A`@Qa\x02\xE1\x92\x91\x90a\tVV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x02\xFEW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03!\x91\x90a\t\x95V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03A\x95\x94\x93\x92\x91\x90a\t\xAEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x82\x91\x90a\t\xDFV[\x95\x94PPPPPV[`\x006\x81a\x03\x9Fa\0\xCB`@\x86\x01\x86a\x08sV[\x91\x94P\x92P\x90Pa\x03\xB6\x83\x83\x83a\0\x93\x88\x80a\x08\xBAV[a\x03\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xB2\x90a\x08\xD0V[`\0\x80a\x03\xE5`@\x87\x01` \x88\x01a\t V[`\x01`\x01`\xA0\x1B\x03\x16a\x03\xFE`\x80\x88\x01``\x89\x01a\t;V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\x15`@\x89\x01\x89a\x08sV[`@Qa\x04#\x92\x91\x90a\tVV[`\0`@Q\x80\x83\x03\x81`\0\x87\x87\xF1\x92PPP=\x80`\0\x81\x14a\x04aW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04fV[``\x91P[P\x91P\x91P\x81a\x04xW\x80Q` \x82\x01\xFD[PPPPPPV[`\x006\x81\x80\x85\x85a\x04\x92` \x82a\n\x01V[a\x04\x9D\x92\x82\x90a\n\x1AV[a\x04\xA6\x91a\nDV[\x90P6`\0\x87`\x04\x88a\x04\xBA` \x82a\n\x01V[\x92a\x04\xC7\x93\x92\x91\x90a\n\x1AV[\x93\x96P\x94P\x91\x92PPP\x92P\x92P\x92V[\x7F\x10[|\x9D\xBB\xC8ec\x8C\xD7\x873Ap\x0B\xCFLb\xDF\xC9\xA4\xF2!B[V\xFD\xE3@\x8Fj\x853\x87\x87\x87\x87\x87\x87`@Qa\x05\x13\x97\x96\x95\x94\x93\x92\x91\x90a\nbV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\0\x80` \x83\x85\x03\x12\x15a\x056W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05NW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x05bW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05qW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x05\x86W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x05\xD2W\x83Q\x15\x15\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x05\xB4V[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x05\xF0W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x08W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x06 W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x06=W`\0\x80\xFD[\x845\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\\W`\0\x80\xFD[a\x06h\x88\x83\x89\x01a\x05\xDEV[\x90\x95P\x93P`@\x87\x015\x91P\x80\x82\x11\x15a\x06\x81W`\0\x80\xFD[P\x85\x01`@\x81\x88\x03\x12\x15a\x06\x94W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a\x06\xB1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xC8W`\0\x80\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a\x06\xDAW`\0\x80\xFD[\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x06\xF4W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x0BW`\0\x80\xFD[a\x07\x17\x85\x82\x86\x01a\x05\xDEV[\x90\x96\x90\x95P\x93PPPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x83\x81R`@` \x82\x01R`\0a\x03\x82`@\x83\x01\x84\x86a\x07#V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07}W`\0\x80\xFD[\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07}W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x07\xB3W`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xD1W`\0\x80\xFD[a\x07\xDD\x89\x82\x8A\x01a\x05\xDEV[\x90\x96P\x94Pa\x07\xF0\x90P`@\x88\x01a\x07fV[\x92P``\x87\x015`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08\rW`\0\x80\xFD[\x91Pa\x08\x1B`\x80\x88\x01a\x07\x82V[\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12a\x08iW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x08\x8AW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xA5W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x06 W`\0\x80\xFD[`\0\x825`>\x19\x836\x03\x01\x81\x12a\x08iW`\0\x80\xFD[` \x80\x82R`0\x90\x82\x01R\x7FBonsaiRelay: callback authorizat`@\x82\x01Ro\x1A[\xDB\x88\x18\xDA\x19X\xDA\xC8\x19\x98Z[\x19Y`\x82\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\t2W`\0\x80\xFD[a\x06\xDA\x82a\x07fV[`\0` \x82\x84\x03\x12\x15a\tMW`\0\x80\xFD[a\x06\xDA\x82a\x07\x82V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\t\x8EWa\t\x8Ea\tfV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\t\xA7W`\0\x80\xFD[PQ\x91\x90PV[`\x80\x81R`\0a\t\xC2`\x80\x83\x01\x87\x89a\x07#V[` \x83\x01\x95\x90\x95RP`@\x81\x01\x92\x90\x92R``\x90\x91\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\t\xF1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\xDAW`\0\x80\xFD[\x81\x81\x03\x81\x81\x11\x15a\n\x14Wa\n\x14a\tfV[\x92\x91PPV[`\0\x80\x85\x85\x11\x15a\n*W`\0\x80\xFD[\x83\x86\x11\x15a\n7W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\n\x14W`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[`\0`\x01\x80`\xA0\x1B\x03\x80\x8A\x16\x83R\x88` \x84\x01R`\xC0`@\x84\x01Ra\n\x8B`\xC0\x84\x01\x88\x8Aa\x07#V[\x95\x16``\x83\x01RP`\x01`\x01`\xE0\x1B\x03\x19\x92\x90\x92\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \x92\xDB\xCBXx9\xCEs\xB8\x85~|2\x92VDA|\x83\xF5\xF4\xB4\xED\x1BY\xDDy4:\t\xD4adsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static BONSAIRELAY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x13\xEF\x14\x12\x14a\0\\W\x80cYa\x1Eg\x14a\0\x85W\x80c\x83\xA1\x9D9\x14a\0\xA8W\x80c\xDF\x05\\\xF1\x14a\0\xBDW\x80c\xE8\x08\x02\xA2\x14a\0\xDFW[`\0\x80\xFD[a\0oa\0j6`\x04a\x05#V[a\0\xF2V[`@Qa\0|\x91\x90a\x05\x98V[`@Q\x80\x91\x03\x90\xF3[a\0\x98a\0\x936`\x04a\x06'V[a\x02\x8EV[`@Q\x90\x15\x15\x81R` \x01a\0|V[a\0\xBBa\0\xB66`\x04a\x06\x9FV[a\x03\x8BV[\0[a\0\xD0a\0\xCB6`\x04a\x06\xE1V[a\x04\x80V[`@Qa\0|\x93\x92\x91\x90a\x07LV[a\0\xBBa\0\xED6`\x04a\x07\x9AV[a\x04\xD8V[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\rWa\x01\ra\x08'V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x016W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x02\x87W6\x84\x84\x83\x81\x81\x10a\x01WWa\x01Wa\x08=V[\x90P` \x02\x81\x01\x90a\x01i\x91\x90a\x08SV[\x90P`\x006\x81a\x01\x7Fa\0\xCB`@\x86\x01\x86a\x08sV[\x91\x94P\x92P\x90Pa\x01\x96\x83\x83\x83a\0\x93\x88\x80a\x08\xBAV[a\x01\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xB2\x90a\x08\xD0V[`@Q\x80\x91\x03\x90\xFD[a\x01\xCB`@\x85\x01` \x86\x01a\t V[`\x01`\x01`\xA0\x1B\x03\x16a\x01\xE4`\x80\x86\x01``\x87\x01a\t;V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\xFB`@\x87\x01\x87a\x08sV[`@Qa\x02\t\x92\x91\x90a\tVV[`\0`@Q\x80\x83\x03\x81`\0\x87\x87\xF1\x92PPP=\x80`\0\x81\x14a\x02GW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02LV[``\x91P[PP\x86\x86\x81Q\x81\x10a\x02`Wa\x02`a\x08=V[` \x02` \x01\x01\x81\x15\x15\x15\x15\x81RPPPPPP\x80\x80a\x02\x7F\x90a\t|V[\x91PPa\x01<V[P\x92\x91PPV[`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16cn\xFE\xF0\ta\x02\xC9\x84\x80a\x08sV[\x88\x86` \x015`\x02\x8A\x8A`@Qa\x02\xE1\x92\x91\x90a\tVV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x02\xFEW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03!\x91\x90a\t\x95V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03A\x95\x94\x93\x92\x91\x90a\t\xAEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x82\x91\x90a\t\xDFV[\x95\x94PPPPPV[`\x006\x81a\x03\x9Fa\0\xCB`@\x86\x01\x86a\x08sV[\x91\x94P\x92P\x90Pa\x03\xB6\x83\x83\x83a\0\x93\x88\x80a\x08\xBAV[a\x03\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xB2\x90a\x08\xD0V[`\0\x80a\x03\xE5`@\x87\x01` \x88\x01a\t V[`\x01`\x01`\xA0\x1B\x03\x16a\x03\xFE`\x80\x88\x01``\x89\x01a\t;V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x04\x15`@\x89\x01\x89a\x08sV[`@Qa\x04#\x92\x91\x90a\tVV[`\0`@Q\x80\x83\x03\x81`\0\x87\x87\xF1\x92PPP=\x80`\0\x81\x14a\x04aW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04fV[``\x91P[P\x91P\x91P\x81a\x04xW\x80Q` \x82\x01\xFD[PPPPPPV[`\x006\x81\x80\x85\x85a\x04\x92` \x82a\n\x01V[a\x04\x9D\x92\x82\x90a\n\x1AV[a\x04\xA6\x91a\nDV[\x90P6`\0\x87`\x04\x88a\x04\xBA` \x82a\n\x01V[\x92a\x04\xC7\x93\x92\x91\x90a\n\x1AV[\x93\x96P\x94P\x91\x92PPP\x92P\x92P\x92V[\x7F\x10[|\x9D\xBB\xC8ec\x8C\xD7\x873Ap\x0B\xCFLb\xDF\xC9\xA4\xF2!B[V\xFD\xE3@\x8Fj\x853\x87\x87\x87\x87\x87\x87`@Qa\x05\x13\x97\x96\x95\x94\x93\x92\x91\x90a\nbV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\0\x80` \x83\x85\x03\x12\x15a\x056W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05NW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x05bW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05qW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x05\x86W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x05\xD2W\x83Q\x15\x15\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x05\xB4V[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x05\xF0W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x08W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x06 W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x06=W`\0\x80\xFD[\x845\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\\W`\0\x80\xFD[a\x06h\x88\x83\x89\x01a\x05\xDEV[\x90\x95P\x93P`@\x87\x015\x91P\x80\x82\x11\x15a\x06\x81W`\0\x80\xFD[P\x85\x01`@\x81\x88\x03\x12\x15a\x06\x94W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a\x06\xB1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xC8W`\0\x80\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a\x06\xDAW`\0\x80\xFD[\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x06\xF4W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x0BW`\0\x80\xFD[a\x07\x17\x85\x82\x86\x01a\x05\xDEV[\x90\x96\x90\x95P\x93PPPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x83\x81R`@` \x82\x01R`\0a\x03\x82`@\x83\x01\x84\x86a\x07#V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07}W`\0\x80\xFD[\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07}W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x07\xB3W`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xD1W`\0\x80\xFD[a\x07\xDD\x89\x82\x8A\x01a\x05\xDEV[\x90\x96P\x94Pa\x07\xF0\x90P`@\x88\x01a\x07fV[\x92P``\x87\x015`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08\rW`\0\x80\xFD[\x91Pa\x08\x1B`\x80\x88\x01a\x07\x82V[\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12a\x08iW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x08\x8AW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xA5W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x06 W`\0\x80\xFD[`\0\x825`>\x19\x836\x03\x01\x81\x12a\x08iW`\0\x80\xFD[` \x80\x82R`0\x90\x82\x01R\x7FBonsaiRelay: callback authorizat`@\x82\x01Ro\x1A[\xDB\x88\x18\xDA\x19X\xDA\xC8\x19\x98Z[\x19Y`\x82\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\t2W`\0\x80\xFD[a\x06\xDA\x82a\x07fV[`\0` \x82\x84\x03\x12\x15a\tMW`\0\x80\xFD[a\x06\xDA\x82a\x07\x82V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\t\x8EWa\t\x8Ea\tfV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\t\xA7W`\0\x80\xFD[PQ\x91\x90PV[`\x80\x81R`\0a\t\xC2`\x80\x83\x01\x87\x89a\x07#V[` \x83\x01\x95\x90\x95RP`@\x81\x01\x92\x90\x92R``\x90\x91\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\t\xF1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06\xDAW`\0\x80\xFD[\x81\x81\x03\x81\x81\x11\x15a\n\x14Wa\n\x14a\tfV[\x92\x91PPV[`\0\x80\x85\x85\x11\x15a\n*W`\0\x80\xFD[\x83\x86\x11\x15a\n7W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\n\x14W`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[`\0`\x01\x80`\xA0\x1B\x03\x80\x8A\x16\x83R\x88` \x84\x01R`\xC0`@\x84\x01Ra\n\x8B`\xC0\x84\x01\x88\x8Aa\x07#V[\x95\x16``\x83\x01RP`\x01`\x01`\xE0\x1B\x03\x19\x92\x90\x92\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \x92\xDB\xCBXx9\xCEs\xB8\x85~|2\x92VDA|\x83\xF5\xF4\xB4\xED\x1BY\xDDy4:\t\xD4adsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static BONSAIRELAY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BonsaiRelay<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BonsaiRelay<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BonsaiRelay<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BonsaiRelay<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BonsaiRelay<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BonsaiRelay))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BonsaiRelay<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BONSAIRELAY_ABI.clone(),
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
                BONSAIRELAY_ABI.clone(),
                BONSAIRELAY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `parsePayload` (0xdf055cf1) function
        pub fn parse_payload(
            &self,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ([u8; 32], ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([223, 5, 92, 241], payload)
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
    for BonsaiRelay<M> {
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
    ///Container type for all input parameters for the `parsePayload` function with signature `parsePayload(bytes)` and selector `0xdf055cf1`
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
    #[ethcall(name = "parsePayload", abi = "parsePayload(bytes)")]
    pub struct ParsePayloadCall {
        pub payload: ::ethers::core::types::Bytes,
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
    pub enum BonsaiRelayCalls {
        CallbackIsAuthorized(CallbackIsAuthorizedCall),
        InvokeCallback(InvokeCallbackCall),
        InvokeCallbacks(InvokeCallbacksCall),
        ParsePayload(ParsePayloadCall),
        RequestCallback(RequestCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for BonsaiRelayCalls {
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
                = <ParsePayloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ParsePayload(decoded));
            }
            if let Ok(decoded)
                = <RequestCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BonsaiRelayCalls {
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
                Self::ParsePayload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BonsaiRelayCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallbackIsAuthorized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvokeCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvokeCallbacks(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParsePayload(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestCallback(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CallbackIsAuthorizedCall> for BonsaiRelayCalls {
        fn from(value: CallbackIsAuthorizedCall) -> Self {
            Self::CallbackIsAuthorized(value)
        }
    }
    impl ::core::convert::From<InvokeCallbackCall> for BonsaiRelayCalls {
        fn from(value: InvokeCallbackCall) -> Self {
            Self::InvokeCallback(value)
        }
    }
    impl ::core::convert::From<InvokeCallbacksCall> for BonsaiRelayCalls {
        fn from(value: InvokeCallbacksCall) -> Self {
            Self::InvokeCallbacks(value)
        }
    }
    impl ::core::convert::From<ParsePayloadCall> for BonsaiRelayCalls {
        fn from(value: ParsePayloadCall) -> Self {
            Self::ParsePayload(value)
        }
    }
    impl ::core::convert::From<RequestCallbackCall> for BonsaiRelayCalls {
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
    ///Container type for all return fields from the `parsePayload` function with signature `parsePayload(bytes)` and selector `0xdf055cf1`
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
    pub struct ParsePayloadReturn(pub [u8; 32], pub ::ethers::core::types::Bytes);
}
