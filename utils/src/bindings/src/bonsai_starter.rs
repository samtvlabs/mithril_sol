pub use bonsai_starter::*;
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
pub mod bonsai_starter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("bonsaiRelay"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IBonsaiRelay"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_verificationImageId"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                            32usize,
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes32"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                (
                    ::std::borrow::ToOwned::to_owned("getVerificationDataId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getVerificationDataId",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BonsaiStarter.VerificationData",
                                        ),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("storeResult"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("storeResult"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BonsaiStarter.VerificationData",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("verification"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verification"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BonsaiStarter.VerificationData",
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
                    ::std::borrow::ToOwned::to_owned("verificationCache"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verificationCache"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("verificationImageId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verificationImageId",
                            ),
                            inputs: ::std::vec![],
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifySignatures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifySignatures"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BonsaiStarter.VerificationData",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("CalculateVerificationCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CalculateVerificationCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("msg"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
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
    pub static BONSAISTARTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\t\x068\x03\x80a\t\x06\x839\x81\x01`@\x81\x90Ra\0/\x91a\0EV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x80R`\xA0Ra\0\x7FV[`\0\x80`@\x83\x85\x03\x12\x15a\0XW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0oW`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\x80Q`\xA0Qa\x08@a\0\xC6`\09`\0\x81\x81`\x87\x01R\x81\x81a\x01\xB8\x01Ra\x02\xAA\x01R`\0\x81\x81a\x01/\x01R\x81\x81a\x02{\x01R\x81\x81a\x03\xD3\x01Ra\x04\x10\x01Ra\x08@`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xCE\xB9\xDEh\x11a\0[W\x80c\xCE\xB9\xDEh\x14a\0\xE4W\x80c\xD3\x83I!\x14a\x01\x17W\x80c\xE7\x0F\xFDK\x14a\x01*W\x80c\xF1k\xA0\r\x14a\x01iW`\0\x80\xFD[\x80c\xA6\xDC\xC3\x88\x14a\0\x82W\x80c\xB2\x84\x08F\x14a\0\xBCW\x80c\xBA\x08\x0F\xAA\x14a\0\xCFW[`\0\x80\xFD[a\0\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA9a\0\xCA6`\x04a\x05\xC8V[a\x01|V[a\0\xE2a\0\xDD6`\x04a\x06\x05V[a\x01\xB6V[\0[a\x01\x07a\0\xF26`\x04a\x06\\V[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xB3V[a\0\xE2a\x01%6`\x04a\x05\xC8V[a\x02yV[a\x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB3V[a\x01\x07a\x01w6`\x04a\x05\xC8V[a\x03NV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x01\x99\x92\x91\x90a\x06\xC5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xDFa\x03\xC8V[a\x01\xE8\x81a\x04HV[`\0a\x01\xF3\x84a\x01|V[`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x87\x15\x15\x17\x90U\x86\x01Q\x90Q\x91\x92Pa\x02 \x91a\x06\xF3V[`@Q\x90\x81\x90\x03\x81 \x85Q\x90\x91a\x027\x91\x90a\x06\xF3V[`@Q\x90\x81\x90\x03\x81 \x85\x15\x15\x82R\x90\x7F\xD4\x9F\x8B\x0C<\xCD\x01^\x1F\x84\xEB\xBFu?\xFD\x99*\xD5I\xB0\xF3\x8A\x89-\xA0\xA3l7\xEF\x87 z\x90` \x01`@Q\x80\x91\x03\x90\xA3PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE8\x08\x02\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`@Q` \x01a\x02\xDA\x91\x90a\x07\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x82Ra\x03\x19\x92\x910\x90c]\x04\x07\xD5`\xE1\x1B\x90b\x01\x86\xA0\x90`\x04\x01a\x07HV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x033W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03GW=`\0\x80>=`\0\xFD[PPPPPV[`\0\x80a\x03Z\x83a\x01|V[`\0\x81\x81R` \x81\x90R`@\x90 T\x90\x91P`\xFF\x16\x80a\x03\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fvalue not available in cache\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x14a\x04EW`@QcC.\x037`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x03\xB8V[PV[`\0\x806a\x04W` \x82a\x07\x9BV[a\x04b\x92\x82\x90a\x07\xC2V[a\x04k\x91a\x07\xECV[\x90P\x80\x82\x14a\x04\x97W`@Qc\x0B\xE5G+`\xE4\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x03\xB8V[PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x04\xC2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xDDWa\x04\xDDa\x04\x9BV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x05\x05Wa\x05\x05a\x04\x9BV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x05\x1EW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x05PW`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x05tWa\x05ta\x04\x9BV[\x81`@R\x82\x93P\x845\x91P\x80\x82\x11\x15a\x05\x8CW`\0\x80\xFD[a\x05\x98\x86\x83\x87\x01a\x04\xB1V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x05\xAEW`\0\x80\xFD[Pa\x05\xBB\x85\x82\x86\x01a\x04\xB1V[` \x83\x01RPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x05\xDAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xF1W`\0\x80\xFD[a\x05\xFD\x84\x82\x85\x01a\x05>V[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\x18W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06/W`\0\x80\xFD[a\x06;\x85\x82\x86\x01a\x05>V[\x92PP` \x83\x015\x80\x15\x15\x81\x14a\x06QW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x06nW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x06\x90W\x81\x81\x01Q\x83\x82\x01R` \x01a\x06xV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x06\xB1\x81` \x86\x01` \x86\x01a\x06uV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\x06\xD8`@\x83\x01\x85a\x06\x99V[\x82\x81\x03` \x84\x01Ra\x06\xEA\x81\x85a\x06\x99V[\x95\x94PPPPPV[`\0\x82Qa\x07\x05\x81\x84` \x87\x01a\x06uV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q`@` \x84\x01Ra\x07+``\x84\x01\x82a\x06\x99V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra\x06\xEA\x82\x82a\x06\x99V[\x85\x81R`\xA0` \x82\x01R`\0a\x07a`\xA0\x83\x01\x87a\x06\x99V[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16`@\x83\x01RP`\x01`\x01`\xE0\x1B\x03\x19\x92\x90\x92\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90\x91\x01R\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x07\xBCWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0\x80\x85\x85\x11\x15a\x07\xD2W`\0\x80\xFD[\x83\x86\x11\x15a\x07\xDFW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\x07\xBCW`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV\xFE\xA2dipfsX\"\x12 I\xA8\xD43\xB15\xCF\xC4S\x08\xEE.\xFErB$q\xAA\x17\xC9\xE1nJ\xFE\xE0\xA0\xD6\xCB\t\xA6\x86\xE4dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static BONSAISTARTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xCE\xB9\xDEh\x11a\0[W\x80c\xCE\xB9\xDEh\x14a\0\xE4W\x80c\xD3\x83I!\x14a\x01\x17W\x80c\xE7\x0F\xFDK\x14a\x01*W\x80c\xF1k\xA0\r\x14a\x01iW`\0\x80\xFD[\x80c\xA6\xDC\xC3\x88\x14a\0\x82W\x80c\xB2\x84\x08F\x14a\0\xBCW\x80c\xBA\x08\x0F\xAA\x14a\0\xCFW[`\0\x80\xFD[a\0\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA9a\0\xCA6`\x04a\x05\xC8V[a\x01|V[a\0\xE2a\0\xDD6`\x04a\x06\x05V[a\x01\xB6V[\0[a\x01\x07a\0\xF26`\x04a\x06\\V[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xB3V[a\0\xE2a\x01%6`\x04a\x05\xC8V[a\x02yV[a\x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB3V[a\x01\x07a\x01w6`\x04a\x05\xC8V[a\x03NV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x01\x99\x92\x91\x90a\x06\xC5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\xDFa\x03\xC8V[a\x01\xE8\x81a\x04HV[`\0a\x01\xF3\x84a\x01|V[`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x87\x15\x15\x17\x90U\x86\x01Q\x90Q\x91\x92Pa\x02 \x91a\x06\xF3V[`@Q\x90\x81\x90\x03\x81 \x85Q\x90\x91a\x027\x91\x90a\x06\xF3V[`@Q\x90\x81\x90\x03\x81 \x85\x15\x15\x82R\x90\x7F\xD4\x9F\x8B\x0C<\xCD\x01^\x1F\x84\xEB\xBFu?\xFD\x99*\xD5I\xB0\xF3\x8A\x89-\xA0\xA3l7\xEF\x87 z\x90` \x01`@Q\x80\x91\x03\x90\xA3PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE8\x08\x02\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`@Q` \x01a\x02\xDA\x91\x90a\x07\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x82Ra\x03\x19\x92\x910\x90c]\x04\x07\xD5`\xE1\x1B\x90b\x01\x86\xA0\x90`\x04\x01a\x07HV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x033W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03GW=`\0\x80>=`\0\xFD[PPPPPV[`\0\x80a\x03Z\x83a\x01|V[`\0\x81\x81R` \x81\x90R`@\x90 T\x90\x91P`\xFF\x16\x80a\x03\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fvalue not available in cache\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x14a\x04EW`@QcC.\x037`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x03\xB8V[PV[`\0\x806a\x04W` \x82a\x07\x9BV[a\x04b\x92\x82\x90a\x07\xC2V[a\x04k\x91a\x07\xECV[\x90P\x80\x82\x14a\x04\x97W`@Qc\x0B\xE5G+`\xE4\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x03\xB8V[PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x04\xC2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xDDWa\x04\xDDa\x04\x9BV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x05\x05Wa\x05\x05a\x04\x9BV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x05\x1EW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`@\x82\x84\x03\x12\x15a\x05PW`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x05tWa\x05ta\x04\x9BV[\x81`@R\x82\x93P\x845\x91P\x80\x82\x11\x15a\x05\x8CW`\0\x80\xFD[a\x05\x98\x86\x83\x87\x01a\x04\xB1V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x05\xAEW`\0\x80\xFD[Pa\x05\xBB\x85\x82\x86\x01a\x04\xB1V[` \x83\x01RPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x05\xDAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xF1W`\0\x80\xFD[a\x05\xFD\x84\x82\x85\x01a\x05>V[\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\x18W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06/W`\0\x80\xFD[a\x06;\x85\x82\x86\x01a\x05>V[\x92PP` \x83\x015\x80\x15\x15\x81\x14a\x06QW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x06nW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x06\x90W\x81\x81\x01Q\x83\x82\x01R` \x01a\x06xV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x06\xB1\x81` \x86\x01` \x86\x01a\x06uV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\x06\xD8`@\x83\x01\x85a\x06\x99V[\x82\x81\x03` \x84\x01Ra\x06\xEA\x81\x85a\x06\x99V[\x95\x94PPPPPV[`\0\x82Qa\x07\x05\x81\x84` \x87\x01a\x06uV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q`@` \x84\x01Ra\x07+``\x84\x01\x82a\x06\x99V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra\x06\xEA\x82\x82a\x06\x99V[\x85\x81R`\xA0` \x82\x01R`\0a\x07a`\xA0\x83\x01\x87a\x06\x99V[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16`@\x83\x01RP`\x01`\x01`\xE0\x1B\x03\x19\x92\x90\x92\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90\x91\x01R\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x07\xBCWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0\x80\x85\x85\x11\x15a\x07\xD2W`\0\x80\xFD[\x83\x86\x11\x15a\x07\xDFW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\x07\xBCW`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV\xFE\xA2dipfsX\"\x12 I\xA8\xD43\xB15\xCF\xC4S\x08\xEE.\xFErB$q\xAA\x17\xC9\xE1nJ\xFE\xE0\xA0\xD6\xCB\t\xA6\x86\xE4dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static BONSAISTARTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BonsaiStarter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BonsaiStarter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BonsaiStarter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BonsaiStarter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BonsaiStarter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BonsaiStarter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BonsaiStarter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BONSAISTARTER_ABI.clone(),
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
                BONSAISTARTER_ABI.clone(),
                BONSAISTARTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `getVerificationDataId` (0xb2840846) function
        pub fn get_verification_data_id(
            &self,
            data: VerificationData,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([178, 132, 8, 70], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `storeResult` (0xba080faa) function
        pub fn store_result(
            &self,
            data: VerificationData,
            result: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 8, 15, 170], (data, result))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verification` (0xf16ba00d) function
        pub fn verification(
            &self,
            data: VerificationData,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([241, 107, 160, 13], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verificationCache` (0xceb9de68) function
        pub fn verification_cache(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([206, 185, 222, 104], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verificationImageId` (0xa6dcc388) function
        pub fn verification_image_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([166, 220, 195, 136], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySignatures` (0xd3834921) function
        pub fn verify_signatures(
            &self,
            data: VerificationData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 131, 73, 33], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CalculateVerificationCallback` event
        pub fn calculate_verification_callback_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CalculateVerificationCallbackFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CalculateVerificationCallbackFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BonsaiStarter<M> {
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
    pub enum BonsaiStarterErrors {
        UnauthorizedCallbackSource(UnauthorizedCallbackSource),
        UnexpectedImageId(UnexpectedImageId),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BonsaiStarterErrors {
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
    impl ::ethers::core::abi::AbiEncode for BonsaiStarterErrors {
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
    impl ::ethers::contract::ContractRevert for BonsaiStarterErrors {
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
    impl ::core::fmt::Display for BonsaiStarterErrors {
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
    impl ::core::convert::From<::std::string::String> for BonsaiStarterErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<UnauthorizedCallbackSource> for BonsaiStarterErrors {
        fn from(value: UnauthorizedCallbackSource) -> Self {
            Self::UnauthorizedCallbackSource(value)
        }
    }
    impl ::core::convert::From<UnexpectedImageId> for BonsaiStarterErrors {
        fn from(value: UnexpectedImageId) -> Self {
            Self::UnexpectedImageId(value)
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
        name = "CalculateVerificationCallback",
        abi = "CalculateVerificationCallback(bytes,bytes,bool)"
    )]
    pub struct CalculateVerificationCallbackFilter {
        #[ethevent(indexed)]
        pub msg: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub sig: ::ethers::core::types::H256,
        pub result: bool,
    }
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
    ///Container type for all input parameters for the `getVerificationDataId` function with signature `getVerificationDataId((bytes,bytes))` and selector `0xb2840846`
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
        name = "getVerificationDataId",
        abi = "getVerificationDataId((bytes,bytes))"
    )]
    pub struct GetVerificationDataIdCall {
        pub data: VerificationData,
    }
    ///Container type for all input parameters for the `storeResult` function with signature `storeResult((bytes,bytes),bool)` and selector `0xba080faa`
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
    #[ethcall(name = "storeResult", abi = "storeResult((bytes,bytes),bool)")]
    pub struct StoreResultCall {
        pub data: VerificationData,
        pub result: bool,
    }
    ///Container type for all input parameters for the `verification` function with signature `verification((bytes,bytes))` and selector `0xf16ba00d`
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
    #[ethcall(name = "verification", abi = "verification((bytes,bytes))")]
    pub struct VerificationCall {
        pub data: VerificationData,
    }
    ///Container type for all input parameters for the `verificationCache` function with signature `verificationCache(bytes32)` and selector `0xceb9de68`
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
    #[ethcall(name = "verificationCache", abi = "verificationCache(bytes32)")]
    pub struct VerificationCacheCall(pub [u8; 32]);
    ///Container type for all input parameters for the `verificationImageId` function with signature `verificationImageId()` and selector `0xa6dcc388`
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
    #[ethcall(name = "verificationImageId", abi = "verificationImageId()")]
    pub struct VerificationImageIdCall;
    ///Container type for all input parameters for the `verifySignatures` function with signature `verifySignatures((bytes,bytes))` and selector `0xd3834921`
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
    #[ethcall(name = "verifySignatures", abi = "verifySignatures((bytes,bytes))")]
    pub struct VerifySignaturesCall {
        pub data: VerificationData,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BonsaiStarterCalls {
        BonsaiRelay(BonsaiRelayCall),
        GetVerificationDataId(GetVerificationDataIdCall),
        StoreResult(StoreResultCall),
        Verification(VerificationCall),
        VerificationCache(VerificationCacheCall),
        VerificationImageId(VerificationImageIdCall),
        VerifySignatures(VerifySignaturesCall),
    }
    impl ::ethers::core::abi::AbiDecode for BonsaiStarterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BonsaiRelayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BonsaiRelay(decoded));
            }
            if let Ok(decoded)
                = <GetVerificationDataIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetVerificationDataId(decoded));
            }
            if let Ok(decoded)
                = <StoreResultCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StoreResult(decoded));
            }
            if let Ok(decoded)
                = <VerificationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verification(decoded));
            }
            if let Ok(decoded)
                = <VerificationCacheCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerificationCache(decoded));
            }
            if let Ok(decoded)
                = <VerificationImageIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerificationImageId(decoded));
            }
            if let Ok(decoded)
                = <VerifySignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::VerifySignatures(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BonsaiStarterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BonsaiRelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVerificationDataId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StoreResult(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verification(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerificationCache(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerificationImageId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifySignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BonsaiStarterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BonsaiRelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVerificationDataId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StoreResult(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verification(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerificationCache(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerificationImageId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifySignatures(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BonsaiRelayCall> for BonsaiStarterCalls {
        fn from(value: BonsaiRelayCall) -> Self {
            Self::BonsaiRelay(value)
        }
    }
    impl ::core::convert::From<GetVerificationDataIdCall> for BonsaiStarterCalls {
        fn from(value: GetVerificationDataIdCall) -> Self {
            Self::GetVerificationDataId(value)
        }
    }
    impl ::core::convert::From<StoreResultCall> for BonsaiStarterCalls {
        fn from(value: StoreResultCall) -> Self {
            Self::StoreResult(value)
        }
    }
    impl ::core::convert::From<VerificationCall> for BonsaiStarterCalls {
        fn from(value: VerificationCall) -> Self {
            Self::Verification(value)
        }
    }
    impl ::core::convert::From<VerificationCacheCall> for BonsaiStarterCalls {
        fn from(value: VerificationCacheCall) -> Self {
            Self::VerificationCache(value)
        }
    }
    impl ::core::convert::From<VerificationImageIdCall> for BonsaiStarterCalls {
        fn from(value: VerificationImageIdCall) -> Self {
            Self::VerificationImageId(value)
        }
    }
    impl ::core::convert::From<VerifySignaturesCall> for BonsaiStarterCalls {
        fn from(value: VerifySignaturesCall) -> Self {
            Self::VerifySignatures(value)
        }
    }
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
    ///Container type for all return fields from the `getVerificationDataId` function with signature `getVerificationDataId((bytes,bytes))` and selector `0xb2840846`
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
    pub struct GetVerificationDataIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `verification` function with signature `verification((bytes,bytes))` and selector `0xf16ba00d`
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
    pub struct VerificationReturn(pub bool);
    ///Container type for all return fields from the `verificationCache` function with signature `verificationCache(bytes32)` and selector `0xceb9de68`
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
    pub struct VerificationCacheReturn(pub bool);
    ///Container type for all return fields from the `verificationImageId` function with signature `verificationImageId()` and selector `0xa6dcc388`
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
    pub struct VerificationImageIdReturn(pub [u8; 32]);
    ///`VerificationData(bytes,bytes)`
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
    pub struct VerificationData {
        pub msg: ::ethers::core::types::Bytes,
        pub sig: ::ethers::core::types::Bytes,
    }
}
