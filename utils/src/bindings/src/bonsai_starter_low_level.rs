pub use bonsai_starter_low_level::*;
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
pub mod bonsai_starter_low_level {
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
                        name: ::std::borrow::ToOwned::to_owned("_fibImageId"),
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
                (
                    ::std::borrow::ToOwned::to_owned("calculateFibonacci"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateFibonacci"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("n"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("fibImageId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fibImageId"),
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
                    ::std::borrow::ToOwned::to_owned("fibonacci"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fibonacci"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("n"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fibonacciCache"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fibonacciCache"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CalculateFibonacciCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CalculateFibonacciCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("n"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
    pub static BONSAISTARTERLOWLEVEL_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x06q8\x03\x80a\x06q\x839\x81\x01`@\x81\x90Ra\0/\x91a\0EV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x80R`\xA0Ra\0\x7FV[`\0\x80`@\x83\x85\x03\x12\x15a\0XW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0oW`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\x80Q`\xA0Qa\x05\xABa\0\xC6`\09`\0\x81\x81`l\x01R\x81\x81a\x01\xD7\x01Ra\x03\x83\x01R`\0\x81\x81a\x01\x03\x01R\x81\x81a\x01\xA8\x01R\x81\x81a\x02\xAE\x01Ra\x02\xEB\x01Ra\x05\xAB`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x13#N\xDC\x14a\0gW\x80ca\x04\x7F\xF4\x14a\0\xA1W\x80ck7\nL\x14a\0\xB4W\x80c\x99\x11\x98\x19\x14a\0\xC9W\x80c\xAEM\xDD\xE9\x14a\0\xE9W\x80c\xE7\x0F\xFDK\x14a\0\xFEW[`\0\x80\xFD[a\0\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x8Ea\0\xAF6`\x04a\x04\x1EV[a\x01=V[a\0\xC7a\0\xC26`\x04a\x04\x1EV[a\x01\xA6V[\0[a\0\x8Ea\0\xD76`\x04a\x04\x1EV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\0\xF1a\x02|V[`@Qa\0\x98\x91\x90a\x04}V[a\x01%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x98V[`\0\x81\x81R` \x81\x90R`@\x81 T\x80\x82\x03a\x01\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fvalue not available in cache\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE8\x08\x02\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`@Q` \x01a\x02\t\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x82Ra\x02G\x92\x910\x90c\xAEM\xDD\xE9`\xE0\x1B\x90au0\x90`\x04\x01a\x04\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02aW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02uW=`\0\x80>=`\0\xFD[PPPPPV[``a\x02\x86a\x02\xA3V[a\x02\x9Ea\x02\x91a\x03#V[a\x02\x99a\x03SV[a\x03\x7FV[\x90P\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x14a\x03 W`@QcC.\x037`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x01\x97V[PV[6`\0a\x03.a\x02\xA3V[`\0`\x046a\x03>` \x82a\x04\xEAV[\x92a\x03K\x93\x92\x91\x90a\x05\x0BV[\x91P\x91P\x90\x91V[`\0a\x03]a\x02\xA3V[`\x006a\x03k` \x82a\x04\xEAV[a\x03v\x92\x82\x90a\x05\x0BV[a\x02\x9E\x91a\x055V[``\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x14a\x03\xADW`\0\x80\xFD[`\0\x80a\x03\xBC\x85\x87\x01\x87a\x05SV[\x91P\x91P\x81\x7F\xA4W\xFB_i\x17i[\x86\xF8\x981\xBB6\x06<\xB2\x1E\x8D/M\x100#\xFB\xBD\xB5\x87+\x91\x1A\x91\x82`@Qa\x03\xF2\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\0\x91\x82R` \x82\x81R`@\x80\x84 \x92\x90\x92U\x81Q\x92\x83R\x82\x01\x90R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x040W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x04]W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x04AV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x04\x90` \x83\x01\x84a\x047V[\x93\x92PPPV[\x85\x81R`\xA0` \x82\x01R`\0a\x04\xB0`\xA0\x83\x01\x87a\x047V[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16`@\x83\x01RP`\x01`\x01`\xE0\x1B\x03\x19\x92\x90\x92\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90\x91\x01R\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x01\xA0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x80\x85\x85\x11\x15a\x05\x1BW`\0\x80\xFD[\x83\x86\x11\x15a\x05(W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\x01\xA0W`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x05fW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV\xFE\xA2dipfsX\"\x12 \n\xD1\t]>Y\xD6~\x0C/\xE4\xD4gbDP\xEF\x8C\x1B\xB8\xFF\xCEt\xE67\xCA\xE5\\\x8F>RQdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static BONSAISTARTERLOWLEVEL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x13#N\xDC\x14a\0gW\x80ca\x04\x7F\xF4\x14a\0\xA1W\x80ck7\nL\x14a\0\xB4W\x80c\x99\x11\x98\x19\x14a\0\xC9W\x80c\xAEM\xDD\xE9\x14a\0\xE9W\x80c\xE7\x0F\xFDK\x14a\0\xFEW[`\0\x80\xFD[a\0\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x8Ea\0\xAF6`\x04a\x04\x1EV[a\x01=V[a\0\xC7a\0\xC26`\x04a\x04\x1EV[a\x01\xA6V[\0[a\0\x8Ea\0\xD76`\x04a\x04\x1EV[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[a\0\xF1a\x02|V[`@Qa\0\x98\x91\x90a\x04}V[a\x01%\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x98V[`\0\x81\x81R` \x81\x90R`@\x81 T\x80\x82\x03a\x01\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fvalue not available in cache\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xE8\x08\x02\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83`@Q` \x01a\x02\t\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x82Ra\x02G\x92\x910\x90c\xAEM\xDD\xE9`\xE0\x1B\x90au0\x90`\x04\x01a\x04\x97V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02aW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02uW=`\0\x80>=`\0\xFD[PPPPPV[``a\x02\x86a\x02\xA3V[a\x02\x9Ea\x02\x91a\x03#V[a\x02\x99a\x03SV[a\x03\x7FV[\x90P\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81\x14a\x03 W`@QcC.\x037`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x01\x97V[PV[6`\0a\x03.a\x02\xA3V[`\0`\x046a\x03>` \x82a\x04\xEAV[\x92a\x03K\x93\x92\x91\x90a\x05\x0BV[\x91P\x91P\x90\x91V[`\0a\x03]a\x02\xA3V[`\x006a\x03k` \x82a\x04\xEAV[a\x03v\x92\x82\x90a\x05\x0BV[a\x02\x9E\x91a\x055V[``\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x14a\x03\xADW`\0\x80\xFD[`\0\x80a\x03\xBC\x85\x87\x01\x87a\x05SV[\x91P\x91P\x81\x7F\xA4W\xFB_i\x17i[\x86\xF8\x981\xBB6\x06<\xB2\x1E\x8D/M\x100#\xFB\xBD\xB5\x87+\x91\x1A\x91\x82`@Qa\x03\xF2\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\0\x91\x82R` \x82\x81R`@\x80\x84 \x92\x90\x92U\x81Q\x92\x83R\x82\x01\x90R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x040W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x04]W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x04AV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x04\x90` \x83\x01\x84a\x047V[\x93\x92PPPV[\x85\x81R`\xA0` \x82\x01R`\0a\x04\xB0`\xA0\x83\x01\x87a\x047V[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16`@\x83\x01RP`\x01`\x01`\xE0\x1B\x03\x19\x92\x90\x92\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90\x91\x01R\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x01\xA0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x80\x85\x85\x11\x15a\x05\x1BW`\0\x80\xFD[\x83\x86\x11\x15a\x05(W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a\x01\xA0W`\0\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x05fW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV\xFE\xA2dipfsX\"\x12 \n\xD1\t]>Y\xD6~\x0C/\xE4\xD4gbDP\xEF\x8C\x1B\xB8\xFF\xCEt\xE67\xCA\xE5\\\x8F>RQdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static BONSAISTARTERLOWLEVEL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BonsaiStarterLowLevel<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BonsaiStarterLowLevel<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BonsaiStarterLowLevel<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BonsaiStarterLowLevel<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BonsaiStarterLowLevel<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BonsaiStarterLowLevel))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BonsaiStarterLowLevel<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BONSAISTARTERLOWLEVEL_ABI.clone(),
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
                BONSAISTARTERLOWLEVEL_ABI.clone(),
                BONSAISTARTERLOWLEVEL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `calculateFibonacci` (0x6b370a4c) function
        pub fn calculate_fibonacci(
            &self,
            n: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 55, 10, 76], n)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fibImageId` (0x13234edc) function
        pub fn fib_image_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([19, 35, 78, 220], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fibonacci` (0x61047ff4) function
        pub fn fibonacci(
            &self,
            n: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([97, 4, 127, 244], n)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fibonacciCache` (0x99119819) function
        pub fn fibonacci_cache(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([153, 17, 152, 25], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CalculateFibonacciCallback` event
        pub fn calculate_fibonacci_callback_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CalculateFibonacciCallbackFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CalculateFibonacciCallbackFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BonsaiStarterLowLevel<M> {
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
    pub enum BonsaiStarterLowLevelErrors {
        UnauthorizedCallbackSource(UnauthorizedCallbackSource),
        UnexpectedImageId(UnexpectedImageId),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BonsaiStarterLowLevelErrors {
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
    impl ::ethers::core::abi::AbiEncode for BonsaiStarterLowLevelErrors {
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
    impl ::ethers::contract::ContractRevert for BonsaiStarterLowLevelErrors {
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
    impl ::core::fmt::Display for BonsaiStarterLowLevelErrors {
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
    impl ::core::convert::From<::std::string::String> for BonsaiStarterLowLevelErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<UnauthorizedCallbackSource>
    for BonsaiStarterLowLevelErrors {
        fn from(value: UnauthorizedCallbackSource) -> Self {
            Self::UnauthorizedCallbackSource(value)
        }
    }
    impl ::core::convert::From<UnexpectedImageId> for BonsaiStarterLowLevelErrors {
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
        name = "CalculateFibonacciCallback",
        abi = "CalculateFibonacciCallback(uint256,uint256)"
    )]
    pub struct CalculateFibonacciCallbackFilter {
        #[ethevent(indexed)]
        pub n: ::ethers::core::types::U256,
        pub result: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `calculateFibonacci` function with signature `calculateFibonacci(uint256)` and selector `0x6b370a4c`
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
    #[ethcall(name = "calculateFibonacci", abi = "calculateFibonacci(uint256)")]
    pub struct CalculateFibonacciCall {
        pub n: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fibImageId` function with signature `fibImageId()` and selector `0x13234edc`
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
    #[ethcall(name = "fibImageId", abi = "fibImageId()")]
    pub struct FibImageIdCall;
    ///Container type for all input parameters for the `fibonacci` function with signature `fibonacci(uint256)` and selector `0x61047ff4`
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
    #[ethcall(name = "fibonacci", abi = "fibonacci(uint256)")]
    pub struct FibonacciCall {
        pub n: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fibonacciCache` function with signature `fibonacciCache(uint256)` and selector `0x99119819`
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
    #[ethcall(name = "fibonacciCache", abi = "fibonacciCache(uint256)")]
    pub struct FibonacciCacheCall(pub ::ethers::core::types::U256);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BonsaiStarterLowLevelCalls {
        BonsaiLowLevelCallbackReceiver(BonsaiLowLevelCallbackReceiverCall),
        BonsaiRelay(BonsaiRelayCall),
        CalculateFibonacci(CalculateFibonacciCall),
        FibImageId(FibImageIdCall),
        Fibonacci(FibonacciCall),
        FibonacciCache(FibonacciCacheCall),
    }
    impl ::ethers::core::abi::AbiDecode for BonsaiStarterLowLevelCalls {
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
            if let Ok(decoded)
                = <CalculateFibonacciCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateFibonacci(decoded));
            }
            if let Ok(decoded)
                = <FibImageIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FibImageId(decoded));
            }
            if let Ok(decoded)
                = <FibonacciCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fibonacci(decoded));
            }
            if let Ok(decoded)
                = <FibonacciCacheCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FibonacciCache(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BonsaiStarterLowLevelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BonsaiLowLevelCallbackReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BonsaiRelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateFibonacci(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FibImageId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Fibonacci(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FibonacciCache(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BonsaiStarterLowLevelCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BonsaiLowLevelCallbackReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BonsaiRelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateFibonacci(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FibImageId(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fibonacci(element) => ::core::fmt::Display::fmt(element, f),
                Self::FibonacciCache(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BonsaiLowLevelCallbackReceiverCall>
    for BonsaiStarterLowLevelCalls {
        fn from(value: BonsaiLowLevelCallbackReceiverCall) -> Self {
            Self::BonsaiLowLevelCallbackReceiver(value)
        }
    }
    impl ::core::convert::From<BonsaiRelayCall> for BonsaiStarterLowLevelCalls {
        fn from(value: BonsaiRelayCall) -> Self {
            Self::BonsaiRelay(value)
        }
    }
    impl ::core::convert::From<CalculateFibonacciCall> for BonsaiStarterLowLevelCalls {
        fn from(value: CalculateFibonacciCall) -> Self {
            Self::CalculateFibonacci(value)
        }
    }
    impl ::core::convert::From<FibImageIdCall> for BonsaiStarterLowLevelCalls {
        fn from(value: FibImageIdCall) -> Self {
            Self::FibImageId(value)
        }
    }
    impl ::core::convert::From<FibonacciCall> for BonsaiStarterLowLevelCalls {
        fn from(value: FibonacciCall) -> Self {
            Self::Fibonacci(value)
        }
    }
    impl ::core::convert::From<FibonacciCacheCall> for BonsaiStarterLowLevelCalls {
        fn from(value: FibonacciCacheCall) -> Self {
            Self::FibonacciCache(value)
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
    ///Container type for all return fields from the `fibImageId` function with signature `fibImageId()` and selector `0x13234edc`
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
    pub struct FibImageIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `fibonacci` function with signature `fibonacci(uint256)` and selector `0x61047ff4`
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
    pub struct FibonacciReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `fibonacciCache` function with signature `fibonacciCache(uint256)` and selector `0x99119819`
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
    pub struct FibonacciCacheReturn(pub ::ethers::core::types::U256);
}
