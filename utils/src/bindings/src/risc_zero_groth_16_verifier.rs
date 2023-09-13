pub use risc_zero_groth_16_verifier::*;
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
pub mod risc_zero_groth_16_verifier {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receipt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Receipt"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("seal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("postStateDigest"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("seal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("postStateDigest"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("journalHash"),
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
                    ::std::borrow::ToOwned::to_owned("verifyProof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifyProof"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pA"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        2usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[2]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pB"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                        ),
                                        2usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[2][2]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pC"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        2usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[2]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pubSignals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[4]"),
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
    pub static RISCZEROGROTH16VERIFIER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x10\xE6\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x04\xB1\x840\x14a\0QW\x80c\x1A6\xEDu\x14a\0xW\x80c_\xE8\xC1;\x14a\0\x8BW\x80cn\xFE\xF0\t\x14a\0\x9EW[`\0\x80\xFD[a\0da\0_6`\x04a\x0B\xE1V[a\0\xB1V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0da\0\x866`\x04a\x0C\xE7V[a\x01\xB3V[a\0da\0\x996`\x04a\r\xB1V[a\x02\x17V[a\0da\0\xAC6`\x04a\x0E\x0BV[a\x07\x8FV[`\0\x80`\0a\0\xCBa\0\xC6\x85` \x01Qa\x07\xFDV[a\t`V[\x91P\x91P`\0\x84`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\0\xE9\x91\x90a\x0E\xAFV[\x90P0`\x01`\x01`\xA0\x1B\x03\x16c_\xE8\xC1;\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q`@Q\x80`\x80\x01`@R\x80oh\xE4-\x8B=\xDCI\x9FN\x17\x99\xA7g\x05*\xB3\x81R` \x01o8\x02hO\x16E\xE0\xA0(X[\x04E\xD3\x921\x81R` \x01\x89\x81R` \x01\x88\x81RP`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01i\x94\x93\x92\x91\x90a\x0FrV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xAA\x91\x90a\x10\x19V[\x95\x94PPPPPV[`\0a\x02\r\x86\x86\x86`\x02\x87\x87`@Qa\x01\xCD\x92\x91\x90a\x10BV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\xEAW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xAC\x91\x90a\x10RV[\x96\x95PPPPPPV[`\0a\x07*V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x81\x10a\x02OW`\0\x80R` `\0\xF3[PV[`\0`@Q\x83\x81R\x84` \x82\x01R\x85`@\x82\x01R`@\x81``\x83`\x07a\x07\xD0Z\x03\xFA\x91P\x81a\x02\x85W`\0\x80R` `\0\xF3[\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83`\x80\x83`\x06a\x07\xD0Z\x03\xFA\x91PP\x80a\x02\xB6W`\0\x80R` `\0\xF3[PPPPPV[\x7F\x0B\xAEN\x82I\xE1jz`\xCC\xD0!_\xBE\t\xB3\x85\x0E\x8C\x18!:\xCD\xB7\xF3\xE7x\xE89\t\xCBU\x85R\x7F)Z \xC0`\x88.\xBB\x89\xC03\x9Fg\xC0e\x9B\x89\xAB#\x9B\xFA\x129\x94=ns\x84\x18\x04\xE1\x1C` \x86\x01R`\0`\x80\x86\x01\x86a\x03Z\x875\x7F\x1B\x88\xE5\xDDT\"\xD4\xC0\x0B\xDA+\xC1\x8B\xC9\xACZ\xBA\"\xB3\x0F\x81\xD6\xF6\xF6\x86 \"k\x98J\xC2f\x7F\tK\xD9\xC4\xB3\x86\x94\x96\xBC\xF8\xAB\x0Fs<\xC6\xB6BC\xB5\xD4\x08\xD0\x9Bp\x1B\x16\xE7\xDD\xD1$\xB1~\x84a\x02RV[a\x03\xAA` \x88\x015\x7F\x0Fh\x0C\x08c\x0F\x99\x8C_\xAC\x17nY<\x8C~\xE2y\x10[\xAF\xF2\xD9:\xEA\xEDZZ\xDD\x0C\xCF\xD6\x7F!\x10\xDC\xCD\xAF\x8A\xE0*\x1D\xA4[\x13\x9C\x06\xDF4So\x19\xE6\x82mf\x87\x94 u\x1AA\x9C\x1A\x10\x84a\x02RV[a\x03\xFA`@\x88\x015\x7F%\xEE\x06\xC4\xF9B\x94q\x04\x99H\x19\x9B)s\xA58\xD9\xBA\xFFT%\xB5,\xE9x\xBC\x88T}\xDC#\x7F\x0Ee\xC1\xC9\xABi\x13<.s\x99\xAA\x17.\xB5\xA8\x9C\xCB~{\xA8D\xA1\x0F\x87\xC8\xCCr\x9A\x0B;y\x84a\x02RV[a\x04J``\x88\x015\x7F#\xFD\xAD\x98\xB6M\xBC\xBC\x18f\xFA\xF6\xF7\xDA=v\x9C\x91i\x9B\xB7)\xC9Bq\xDCB\x88\x11\xD9^Q\x7F\x0B|v9\xA6L\xB5PX1\xEB\t\xE7]A\x86,c\xB1\xF3\xECU\xFCC*`\xB9\x89\xEE\xAA\xD5\xA5\x84a\x02RV[P\x825\x81R\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG` \x84\x015\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x03\x06` \x82\x01R\x835`@\x82\x01R` \x84\x015``\x82\x01R`@\x84\x015`\x80\x82\x01R``\x84\x015`\xA0\x82\x01R\x7F-M\x9A\xA7\xE3\x02\xD9\xDFAt\x9DU\x07\x94\x9D\x05\xDB\xEA3\xFB\xB1ld;\"\xF5\x99\xA2\xBEm\xF2\xE2`\xC0\x82\x01R\x7F\x14\xBE\xDDP<7\xCE\xB0a\xD8\xEC` \x9F\xE3E\xCE\x89\x83\n\x19#\x03\x01\xF0v\xCA\xFF\0M\x19&`\xE0\x82\x01R\x7F\tg\x03/\xCB\xF7v\xD1\xAF\xC9\x85\xF8\x88w\xF1\x82\xD3\x84\x80\xA6S\xF2\xDE\xCA\xA9yL\xBC;\xF3\x06\x0Ca\x01\0\x82\x01R\x7F\x0E\x18xG\xADLy\x83t\xD0\xD6s+\xF5\x01\x84}\xD6\x8B\xC0\xE0q$\x1E\x02\x13\xBC\x7F\xC1=\xB7\xABa\x01 \x82\x01R\x7F0L\xFB\xD1\xE0\x8ApJ\x99\xF5\xE8G\xD9?\x8C<\xAA\xFD\xDE\xC4kz\r7\x9D\xA6\x9AM\x11#F\xA7a\x01@\x82\x01R\x7F\x179\xC1\xB1\xA4W\xA8\xC711#\xD2M/\x91\x92\xF8\x96\xB7\xC6>\xEA\x05\xA9\xD5\x7F\x06Tz\xD0\xCE\xC8a\x01`\x82\x01R`\0\x87\x01Qa\x01\x80\x82\x01R` `\0\x01\x87\x01Qa\x01\xA0\x82\x01R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2a\x01\xC0\x82\x01R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xEDa\x01\xE0\x82\x01R\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[a\x02\0\x82\x01R\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAAa\x02 \x82\x01R\x845a\x02@\x82\x01R` \x85\x015a\x02`\x82\x01R\x7F(\xF1YE\xC0\xD3P\xDCy\xC3\xFA\x9A\xEA\xE5j|op\xA3\x8B\xAA\xA4\x8B\xBD\xA1\x951\xAC\x05d3ya\x02\x80\x82\x01R\x7F\x14\xFCwj|\xE2\x8A\xFA\x94d\x9F8\xF3s\xA6\x1Fd\xAC\x98L\x02\xEE\xE7\xEB\x8A6\xE6\x8A\xC6\x9E\xDF\xEDa\x02\xA0\x82\x01R\x7F+\x13Z%\xCE\x83L!\x81\x81\x85\xF0w\x9D\xDA\xF6_\x83\x1Dl\xB9A\xD7Jn\x8A\x10\xCE\x1B\x0E\x05aa\x02\xC0\x82\x01R\x7F/A\xF6\xB9\xA1qC?*.\xAA\x04B\xB6\xCC\x89x^\xDD\xCB\xD2i\xE2\xA48m\xBD\x91\xF6%;\x84a\x02\xE0\x82\x01R` \x81a\x03\0\x83`\x08a\x07\xD0Z\x03\xFA\x90Q\x16\x96\x95PPPPPPV[`@Qa\x03\x80\x81\x01`@Ra\x07B`\0\x84\x015a\x02\x1EV[a\x07O` \x84\x015a\x02\x1EV[a\x07\\`@\x84\x015a\x02\x1EV[a\x07i``\x84\x015a\x02\x1EV[a\x07v`\x80\x84\x015a\x02\x1EV[a\x07\x83\x81\x84\x86\x88\x8Aa\x02\xBDV[\x90P\x80`\0R` `\0\xF3[`\0\x80`@Q\x80`@\x01`@R\x80\x87\x81R` \x01`@Q\x80`\xA0\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01`@Q\x80`@\x01`@R\x80`\0`\x02\x81\x11\x15a\x07\xD8Wa\x07\xD8a\x10kV[\x81R`\0` \x91\x82\x01\x81\x90R\x91\x83R\x82\x01R`@\x01\x86\x90R\x90R\x90Pa\x02\r\x81a\0\xB1V[`\0`\x02\x80`@Qa\x08&\x90prisc0.ReceiptMeta`x\x1B\x81R`\x11\x01\x90V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x08CW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08f\x91\x90a\x10RV[``\x84\x01Q\x84Q` \x86\x01Q`\x80\x87\x01Q`@\x88\x01QQ`\x18\x90`\x02\x81\x11\x15a\x08\x91Wa\x08\x91a\x10kV[`@\x8A\x81\x01Q` \x90\x81\x01Q\x82Q\x91\x82\x01\x99\x90\x99R\x90\x81\x01\x96\x90\x96R``\x86\x01\x94\x90\x94R`\x80\x85\x01\x92\x90\x92R`\xA0\x84\x01Rc\xFF\xFF\xFF\xFF\x90\x91\x16\x90\x1B`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16`\xC0\x82\x01R`\xF8\x91\x90\x91\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\xC4\x82\x01R`\x01`\xFA\x1B`\xC8\x82\x01R`\xCA\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\t\x1A\x91a\x10\x81V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\t7W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tZ\x91\x90a\x10RV[\x92\x91PPV[`\0\x80\x80a\n\xAD\x84`\0\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x7F\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\x16\x90\x1C\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x7F\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\x16\x90\x1C\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x7F\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\x16\x90\x1C\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x1C\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x95`\x80\x91\x90\x91\x1C\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x08Wa\x0B\x08a\n\xCFV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x08Wa\x0B\x08a\n\xCFV[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x08Wa\x0B\x08a\n\xCFV[`\0\x82`\x1F\x83\x01\x12a\x0BeW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x80Wa\x0B\x80a\n\xCFV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0B\xA8Wa\x0B\xA8a\n\xCFV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0B\xC1W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0B\xF3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\x0BW`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xE0\x81\x12\x15a\x0C W`\0\x80\xFD[a\x0C(a\n\xE5V[\x835\x83\x81\x11\x15a\x0C7W`\0\x80\xFD[a\x0CC\x88\x82\x87\x01a\x0BTV[\x82RP`\xC0`\x1F\x19\x83\x01\x12\x15a\x0CXW`\0\x80\xFD[a\x0C`a\x0B\x0EV[\x92P` \x84\x015\x83R`@\x84\x015` \x84\x01R`@`_\x19\x83\x01\x12\x15a\x0C\x85W`\0\x80\xFD[a\x0C\x8Da\n\xE5V[\x91P``\x84\x015`\x03\x81\x10a\x0C\xA1W`\0\x80\xFD[\x82R`\x80\x84\x015`\xFF\x81\x16\x81\x14a\x0C\xB7W`\0\x80\xFD[` \x83\x81\x01\x91\x90\x91R`@\x84\x01\x92\x90\x92R`\xA0\x84\x015``\x84\x01R`\xC0\x90\x93\x015`\x80\x83\x01R\x82\x01R\x93\x92PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x0C\xFFW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\x17W`\0\x80\xFD[a\r#\x89\x83\x8A\x01a\x0BTV[\x96P` \x88\x015\x95P`@\x88\x015\x94P``\x88\x015\x91P\x80\x82\x11\x15a\rGW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\r[W`\0\x80\xFD[\x815\x81\x81\x11\x15a\rjW`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\r|W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[\x80`@\x81\x01\x83\x10\x15a\tZW`\0\x80\xFD[\x80`\x80\x81\x01\x83\x10\x15a\tZW`\0\x80\xFD[`\0\x80`\0\x80a\x01\x80\x85\x87\x03\x12\x15a\r\xC8W`\0\x80\xFD[a\r\xD2\x86\x86a\r\x8FV[\x93Pa\r\xE1\x86`@\x87\x01a\r\xA0V[\x92Pa\r\xF0\x86`\xC0\x87\x01a\r\x8FV[\x91Pa\x0E\0\x86a\x01\0\x87\x01a\r\xA0V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0E!W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E8W`\0\x80\xFD[a\x0ED\x87\x82\x88\x01a\x0BTV[\x97` \x87\x015\x97P`@\x87\x015\x96``\x015\x95P\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a\x0EpW`\0\x80\xFD[a\x0Exa\n\xE5V[\x80`@\x84\x01\x85\x81\x11\x15a\x0E\x8AW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x0E\xA4W\x80Q\x84R` \x93\x84\x01\x93\x01a\x0E\x8CV[P\x90\x95\x94PPPPPV[`\0a\x01\0\x82\x84\x03\x12\x15a\x0E\xC2W`\0\x80\xFD[a\x0E\xCAa\x0B1V[a\x0E\xD4\x84\x84a\x0E_V[\x81R`@\x84`_\x85\x01\x12a\x0E\xE7W`\0\x80\xFD[a\x0E\xEFa\n\xE5V[\x80`\xC0\x86\x01\x87\x81\x11\x15a\x0F\x01W`\0\x80\xFD[\x83\x87\x01[\x81\x81\x10\x15a\x0F&Wa\x0F\x17\x89\x82a\x0E_V[\x84R` \x90\x93\x01\x92\x84\x01a\x0F\x05V[P\x81` \x86\x01Ra\x0F7\x88\x82a\x0E_V[\x84\x86\x01RPPPP\x80\x91PP\x92\x91PPV[\x80`\0[`\x02\x81\x10\x15a\x0FlW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x0FMV[PPPPV[a\x01\x80\x81\x01a\x0F\x81\x82\x87a\x0FIV[`@\x80\x83\x01\x86`\0[`\x02\x80\x82\x10a\x0F\x99WPa\x0F\xD4V[\x82Q\x84`\0[\x83\x81\x10\x15a\x0F\xBDW\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x0F\x9FV[PPP\x92\x84\x01\x92P` \x91\x90\x91\x01\x90`\x01\x01a\x0F\x8AV[PPPPa\x0F\xE5`\xC0\x83\x01\x85a\x0FIV[a\x01\0\x82\x01\x83`\0[`\x04\x81\x10\x15a\x10\rW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x0F\xEEV[PPP\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x10+W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x10;W`\0\x80\xFD[\x93\x92PPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10dW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82Q`\0[\x81\x81\x10\x15a\x10\xA2W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x10\x88V[P`\0\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 \x87\xD1\xE2\x1A\xF0GZ%\xA60\x1B\xAAX\x84\x9B]\x1BR\x9D\xA5\x1A8A\xC5\x8F\xDBp\x1D\x8E\xDB\xC8\xAAdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static RISCZEROGROTH16VERIFIER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x04\xB1\x840\x14a\0QW\x80c\x1A6\xEDu\x14a\0xW\x80c_\xE8\xC1;\x14a\0\x8BW\x80cn\xFE\xF0\t\x14a\0\x9EW[`\0\x80\xFD[a\0da\0_6`\x04a\x0B\xE1V[a\0\xB1V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0da\0\x866`\x04a\x0C\xE7V[a\x01\xB3V[a\0da\0\x996`\x04a\r\xB1V[a\x02\x17V[a\0da\0\xAC6`\x04a\x0E\x0BV[a\x07\x8FV[`\0\x80`\0a\0\xCBa\0\xC6\x85` \x01Qa\x07\xFDV[a\t`V[\x91P\x91P`\0\x84`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\0\xE9\x91\x90a\x0E\xAFV[\x90P0`\x01`\x01`\xA0\x1B\x03\x16c_\xE8\xC1;\x82`\0\x01Q\x83` \x01Q\x84`@\x01Q`@Q\x80`\x80\x01`@R\x80oh\xE4-\x8B=\xDCI\x9FN\x17\x99\xA7g\x05*\xB3\x81R` \x01o8\x02hO\x16E\xE0\xA0(X[\x04E\xD3\x921\x81R` \x01\x89\x81R` \x01\x88\x81RP`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01i\x94\x93\x92\x91\x90a\x0FrV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xAA\x91\x90a\x10\x19V[\x95\x94PPPPPV[`\0a\x02\r\x86\x86\x86`\x02\x87\x87`@Qa\x01\xCD\x92\x91\x90a\x10BV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x01\xEAW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xAC\x91\x90a\x10RV[\x96\x95PPPPPPV[`\0a\x07*V[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x81\x10a\x02OW`\0\x80R` `\0\xF3[PV[`\0`@Q\x83\x81R\x84` \x82\x01R\x85`@\x82\x01R`@\x81``\x83`\x07a\x07\xD0Z\x03\xFA\x91P\x81a\x02\x85W`\0\x80R` `\0\xF3[\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83`\x80\x83`\x06a\x07\xD0Z\x03\xFA\x91PP\x80a\x02\xB6W`\0\x80R` `\0\xF3[PPPPPV[\x7F\x0B\xAEN\x82I\xE1jz`\xCC\xD0!_\xBE\t\xB3\x85\x0E\x8C\x18!:\xCD\xB7\xF3\xE7x\xE89\t\xCBU\x85R\x7F)Z \xC0`\x88.\xBB\x89\xC03\x9Fg\xC0e\x9B\x89\xAB#\x9B\xFA\x129\x94=ns\x84\x18\x04\xE1\x1C` \x86\x01R`\0`\x80\x86\x01\x86a\x03Z\x875\x7F\x1B\x88\xE5\xDDT\"\xD4\xC0\x0B\xDA+\xC1\x8B\xC9\xACZ\xBA\"\xB3\x0F\x81\xD6\xF6\xF6\x86 \"k\x98J\xC2f\x7F\tK\xD9\xC4\xB3\x86\x94\x96\xBC\xF8\xAB\x0Fs<\xC6\xB6BC\xB5\xD4\x08\xD0\x9Bp\x1B\x16\xE7\xDD\xD1$\xB1~\x84a\x02RV[a\x03\xAA` \x88\x015\x7F\x0Fh\x0C\x08c\x0F\x99\x8C_\xAC\x17nY<\x8C~\xE2y\x10[\xAF\xF2\xD9:\xEA\xEDZZ\xDD\x0C\xCF\xD6\x7F!\x10\xDC\xCD\xAF\x8A\xE0*\x1D\xA4[\x13\x9C\x06\xDF4So\x19\xE6\x82mf\x87\x94 u\x1AA\x9C\x1A\x10\x84a\x02RV[a\x03\xFA`@\x88\x015\x7F%\xEE\x06\xC4\xF9B\x94q\x04\x99H\x19\x9B)s\xA58\xD9\xBA\xFFT%\xB5,\xE9x\xBC\x88T}\xDC#\x7F\x0Ee\xC1\xC9\xABi\x13<.s\x99\xAA\x17.\xB5\xA8\x9C\xCB~{\xA8D\xA1\x0F\x87\xC8\xCCr\x9A\x0B;y\x84a\x02RV[a\x04J``\x88\x015\x7F#\xFD\xAD\x98\xB6M\xBC\xBC\x18f\xFA\xF6\xF7\xDA=v\x9C\x91i\x9B\xB7)\xC9Bq\xDCB\x88\x11\xD9^Q\x7F\x0B|v9\xA6L\xB5PX1\xEB\t\xE7]A\x86,c\xB1\xF3\xECU\xFCC*`\xB9\x89\xEE\xAA\xD5\xA5\x84a\x02RV[P\x825\x81R\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG` \x84\x015\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x03\x06` \x82\x01R\x835`@\x82\x01R` \x84\x015``\x82\x01R`@\x84\x015`\x80\x82\x01R``\x84\x015`\xA0\x82\x01R\x7F-M\x9A\xA7\xE3\x02\xD9\xDFAt\x9DU\x07\x94\x9D\x05\xDB\xEA3\xFB\xB1ld;\"\xF5\x99\xA2\xBEm\xF2\xE2`\xC0\x82\x01R\x7F\x14\xBE\xDDP<7\xCE\xB0a\xD8\xEC` \x9F\xE3E\xCE\x89\x83\n\x19#\x03\x01\xF0v\xCA\xFF\0M\x19&`\xE0\x82\x01R\x7F\tg\x03/\xCB\xF7v\xD1\xAF\xC9\x85\xF8\x88w\xF1\x82\xD3\x84\x80\xA6S\xF2\xDE\xCA\xA9yL\xBC;\xF3\x06\x0Ca\x01\0\x82\x01R\x7F\x0E\x18xG\xADLy\x83t\xD0\xD6s+\xF5\x01\x84}\xD6\x8B\xC0\xE0q$\x1E\x02\x13\xBC\x7F\xC1=\xB7\xABa\x01 \x82\x01R\x7F0L\xFB\xD1\xE0\x8ApJ\x99\xF5\xE8G\xD9?\x8C<\xAA\xFD\xDE\xC4kz\r7\x9D\xA6\x9AM\x11#F\xA7a\x01@\x82\x01R\x7F\x179\xC1\xB1\xA4W\xA8\xC711#\xD2M/\x91\x92\xF8\x96\xB7\xC6>\xEA\x05\xA9\xD5\x7F\x06Tz\xD0\xCE\xC8a\x01`\x82\x01R`\0\x87\x01Qa\x01\x80\x82\x01R` `\0\x01\x87\x01Qa\x01\xA0\x82\x01R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2a\x01\xC0\x82\x01R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xEDa\x01\xE0\x82\x01R\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[a\x02\0\x82\x01R\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAAa\x02 \x82\x01R\x845a\x02@\x82\x01R` \x85\x015a\x02`\x82\x01R\x7F(\xF1YE\xC0\xD3P\xDCy\xC3\xFA\x9A\xEA\xE5j|op\xA3\x8B\xAA\xA4\x8B\xBD\xA1\x951\xAC\x05d3ya\x02\x80\x82\x01R\x7F\x14\xFCwj|\xE2\x8A\xFA\x94d\x9F8\xF3s\xA6\x1Fd\xAC\x98L\x02\xEE\xE7\xEB\x8A6\xE6\x8A\xC6\x9E\xDF\xEDa\x02\xA0\x82\x01R\x7F+\x13Z%\xCE\x83L!\x81\x81\x85\xF0w\x9D\xDA\xF6_\x83\x1Dl\xB9A\xD7Jn\x8A\x10\xCE\x1B\x0E\x05aa\x02\xC0\x82\x01R\x7F/A\xF6\xB9\xA1qC?*.\xAA\x04B\xB6\xCC\x89x^\xDD\xCB\xD2i\xE2\xA48m\xBD\x91\xF6%;\x84a\x02\xE0\x82\x01R` \x81a\x03\0\x83`\x08a\x07\xD0Z\x03\xFA\x90Q\x16\x96\x95PPPPPPV[`@Qa\x03\x80\x81\x01`@Ra\x07B`\0\x84\x015a\x02\x1EV[a\x07O` \x84\x015a\x02\x1EV[a\x07\\`@\x84\x015a\x02\x1EV[a\x07i``\x84\x015a\x02\x1EV[a\x07v`\x80\x84\x015a\x02\x1EV[a\x07\x83\x81\x84\x86\x88\x8Aa\x02\xBDV[\x90P\x80`\0R` `\0\xF3[`\0\x80`@Q\x80`@\x01`@R\x80\x87\x81R` \x01`@Q\x80`\xA0\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01`@Q\x80`@\x01`@R\x80`\0`\x02\x81\x11\x15a\x07\xD8Wa\x07\xD8a\x10kV[\x81R`\0` \x91\x82\x01\x81\x90R\x91\x83R\x82\x01R`@\x01\x86\x90R\x90R\x90Pa\x02\r\x81a\0\xB1V[`\0`\x02\x80`@Qa\x08&\x90prisc0.ReceiptMeta`x\x1B\x81R`\x11\x01\x90V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x08CW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08f\x91\x90a\x10RV[``\x84\x01Q\x84Q` \x86\x01Q`\x80\x87\x01Q`@\x88\x01QQ`\x18\x90`\x02\x81\x11\x15a\x08\x91Wa\x08\x91a\x10kV[`@\x8A\x81\x01Q` \x90\x81\x01Q\x82Q\x91\x82\x01\x99\x90\x99R\x90\x81\x01\x96\x90\x96R``\x86\x01\x94\x90\x94R`\x80\x85\x01\x92\x90\x92R`\xA0\x84\x01Rc\xFF\xFF\xFF\xFF\x90\x91\x16\x90\x1B`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16`\xC0\x82\x01R`\xF8\x91\x90\x91\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16`\xC4\x82\x01R`\x01`\xFA\x1B`\xC8\x82\x01R`\xCA\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\t\x1A\x91a\x10\x81V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\t7W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tZ\x91\x90a\x10RV[\x92\x91PPV[`\0\x80\x80a\n\xAD\x84`\0\x81\x90P`\x08\x81~\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\x16\x90\x1B`\x08\x82\x7F\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\xFF\0\x16\x90\x1C\x17\x90P`\x10\x81}\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\x16\x90\x1B`\x10\x82\x7F\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\xFF\xFF\0\0\x16\x90\x1C\x17\x90P` \x81{\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\x16\x90\x1B` \x82\x7F\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\x16\x90\x1C\x17\x90P`@\x81w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x1B`@\x82w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x1C\x17\x90P`\x80\x81\x90\x1B`\x80\x82\x90\x1C\x17\x90P\x91\x90PV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x95`\x80\x91\x90\x91\x1C\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x08Wa\x0B\x08a\n\xCFV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x08Wa\x0B\x08a\n\xCFV[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x08Wa\x0B\x08a\n\xCFV[`\0\x82`\x1F\x83\x01\x12a\x0BeW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x80Wa\x0B\x80a\n\xCFV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0B\xA8Wa\x0B\xA8a\n\xCFV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0B\xC1W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0B\xF3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\x0BW`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xE0\x81\x12\x15a\x0C W`\0\x80\xFD[a\x0C(a\n\xE5V[\x835\x83\x81\x11\x15a\x0C7W`\0\x80\xFD[a\x0CC\x88\x82\x87\x01a\x0BTV[\x82RP`\xC0`\x1F\x19\x83\x01\x12\x15a\x0CXW`\0\x80\xFD[a\x0C`a\x0B\x0EV[\x92P` \x84\x015\x83R`@\x84\x015` \x84\x01R`@`_\x19\x83\x01\x12\x15a\x0C\x85W`\0\x80\xFD[a\x0C\x8Da\n\xE5V[\x91P``\x84\x015`\x03\x81\x10a\x0C\xA1W`\0\x80\xFD[\x82R`\x80\x84\x015`\xFF\x81\x16\x81\x14a\x0C\xB7W`\0\x80\xFD[` \x83\x81\x01\x91\x90\x91R`@\x84\x01\x92\x90\x92R`\xA0\x84\x015``\x84\x01R`\xC0\x90\x93\x015`\x80\x83\x01R\x82\x01R\x93\x92PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x0C\xFFW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\x17W`\0\x80\xFD[a\r#\x89\x83\x8A\x01a\x0BTV[\x96P` \x88\x015\x95P`@\x88\x015\x94P``\x88\x015\x91P\x80\x82\x11\x15a\rGW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\r[W`\0\x80\xFD[\x815\x81\x81\x11\x15a\rjW`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\r|W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[\x80`@\x81\x01\x83\x10\x15a\tZW`\0\x80\xFD[\x80`\x80\x81\x01\x83\x10\x15a\tZW`\0\x80\xFD[`\0\x80`\0\x80a\x01\x80\x85\x87\x03\x12\x15a\r\xC8W`\0\x80\xFD[a\r\xD2\x86\x86a\r\x8FV[\x93Pa\r\xE1\x86`@\x87\x01a\r\xA0V[\x92Pa\r\xF0\x86`\xC0\x87\x01a\r\x8FV[\x91Pa\x0E\0\x86a\x01\0\x87\x01a\r\xA0V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0E!W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E8W`\0\x80\xFD[a\x0ED\x87\x82\x88\x01a\x0BTV[\x97` \x87\x015\x97P`@\x87\x015\x96``\x015\x95P\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a\x0EpW`\0\x80\xFD[a\x0Exa\n\xE5V[\x80`@\x84\x01\x85\x81\x11\x15a\x0E\x8AW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x0E\xA4W\x80Q\x84R` \x93\x84\x01\x93\x01a\x0E\x8CV[P\x90\x95\x94PPPPPV[`\0a\x01\0\x82\x84\x03\x12\x15a\x0E\xC2W`\0\x80\xFD[a\x0E\xCAa\x0B1V[a\x0E\xD4\x84\x84a\x0E_V[\x81R`@\x84`_\x85\x01\x12a\x0E\xE7W`\0\x80\xFD[a\x0E\xEFa\n\xE5V[\x80`\xC0\x86\x01\x87\x81\x11\x15a\x0F\x01W`\0\x80\xFD[\x83\x87\x01[\x81\x81\x10\x15a\x0F&Wa\x0F\x17\x89\x82a\x0E_V[\x84R` \x90\x93\x01\x92\x84\x01a\x0F\x05V[P\x81` \x86\x01Ra\x0F7\x88\x82a\x0E_V[\x84\x86\x01RPPPP\x80\x91PP\x92\x91PPV[\x80`\0[`\x02\x81\x10\x15a\x0FlW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\x0FMV[PPPPV[a\x01\x80\x81\x01a\x0F\x81\x82\x87a\x0FIV[`@\x80\x83\x01\x86`\0[`\x02\x80\x82\x10a\x0F\x99WPa\x0F\xD4V[\x82Q\x84`\0[\x83\x81\x10\x15a\x0F\xBDW\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x0F\x9FV[PPP\x92\x84\x01\x92P` \x91\x90\x91\x01\x90`\x01\x01a\x0F\x8AV[PPPPa\x0F\xE5`\xC0\x83\x01\x85a\x0FIV[a\x01\0\x82\x01\x83`\0[`\x04\x81\x10\x15a\x10\rW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a\x0F\xEEV[PPP\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x10+W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x10;W`\0\x80\xFD[\x93\x92PPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10dW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82Q`\0[\x81\x81\x10\x15a\x10\xA2W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x10\x88V[P`\0\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 \x87\xD1\xE2\x1A\xF0GZ%\xA60\x1B\xAAX\x84\x9B]\x1BR\x9D\xA5\x1A8A\xC5\x8F\xDBp\x1D\x8E\xDB\xC8\xAAdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static RISCZEROGROTH16VERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RiscZeroGroth16Verifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RiscZeroGroth16Verifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RiscZeroGroth16Verifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RiscZeroGroth16Verifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RiscZeroGroth16Verifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RiscZeroGroth16Verifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RiscZeroGroth16Verifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RISCZEROGROTH16VERIFIER_ABI.clone(),
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
                RISCZEROGROTH16VERIFIER_ABI.clone(),
                RISCZEROGROTH16VERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `verify` (0x04b18430) function
        pub fn verify_0(
            &self,
            receipt: Receipt,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([4, 177, 132, 48], (receipt,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0x1a36ed75) function
        pub fn verify_1(
            &self,
            seal: ::ethers::core::types::Bytes,
            image_id: [u8; 32],
            post_state_digest: [u8; 32],
            journal: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [26, 54, 237, 117],
                    (seal, image_id, post_state_digest, journal),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify` (0x6efef009) function
        pub fn verify_2(
            &self,
            seal: ::ethers::core::types::Bytes,
            image_id: [u8; 32],
            post_state_digest: [u8; 32],
            journal_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [110, 254, 240, 9],
                    (seal, image_id, post_state_digest, journal_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyProof` (0x5fe8c13b) function
        pub fn verify_proof(
            &self,
            p_a: [::ethers::core::types::U256; 2],
            p_b: [[::ethers::core::types::U256; 2]; 2],
            p_c: [::ethers::core::types::U256; 2],
            pub_signals: [::ethers::core::types::U256; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([95, 232, 193, 59], (p_a, p_b, p_c, pub_signals))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RiscZeroGroth16Verifier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `verify` function with signature `verify((bytes,(bytes32,bytes32,(uint8,uint8),bytes32,bytes32)))` and selector `0x04b18430`
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
        name = "verify",
        abi = "verify((bytes,(bytes32,bytes32,(uint8,uint8),bytes32,bytes32)))"
    )]
    pub struct Verify0Call {
        pub receipt: Receipt,
    }
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes,bytes32,bytes32,bytes)` and selector `0x1a36ed75`
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
    #[ethcall(name = "verify", abi = "verify(bytes,bytes32,bytes32,bytes)")]
    pub struct Verify1Call {
        pub seal: ::ethers::core::types::Bytes,
        pub image_id: [u8; 32],
        pub post_state_digest: [u8; 32],
        pub journal: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes,bytes32,bytes32,bytes32)` and selector `0x6efef009`
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
    #[ethcall(name = "verify", abi = "verify(bytes,bytes32,bytes32,bytes32)")]
    pub struct Verify2Call {
        pub seal: ::ethers::core::types::Bytes,
        pub image_id: [u8; 32],
        pub post_state_digest: [u8; 32],
        pub journal_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `verifyProof` function with signature `verifyProof(uint256[2],uint256[2][2],uint256[2],uint256[4])` and selector `0x5fe8c13b`
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
        name = "verifyProof",
        abi = "verifyProof(uint256[2],uint256[2][2],uint256[2],uint256[4])"
    )]
    pub struct VerifyProofCall {
        pub p_a: [::ethers::core::types::U256; 2],
        pub p_b: [[::ethers::core::types::U256; 2]; 2],
        pub p_c: [::ethers::core::types::U256; 2],
        pub pub_signals: [::ethers::core::types::U256; 4],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum RiscZeroGroth16VerifierCalls {
        Verify0(Verify0Call),
        Verify1(Verify1Call),
        Verify2(Verify2Call),
        VerifyProof(VerifyProofCall),
    }
    impl ::ethers::core::abi::AbiDecode for RiscZeroGroth16VerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <Verify0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verify0(decoded));
            }
            if let Ok(decoded)
                = <Verify1Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verify1(decoded));
            }
            if let Ok(decoded)
                = <Verify2Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Verify2(decoded));
            }
            if let Ok(decoded)
                = <VerifyProofCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VerifyProof(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RiscZeroGroth16VerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Verify0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Verify1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Verify2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RiscZeroGroth16VerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Verify0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify2(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyProof(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Verify0Call> for RiscZeroGroth16VerifierCalls {
        fn from(value: Verify0Call) -> Self {
            Self::Verify0(value)
        }
    }
    impl ::core::convert::From<Verify1Call> for RiscZeroGroth16VerifierCalls {
        fn from(value: Verify1Call) -> Self {
            Self::Verify1(value)
        }
    }
    impl ::core::convert::From<Verify2Call> for RiscZeroGroth16VerifierCalls {
        fn from(value: Verify2Call) -> Self {
            Self::Verify2(value)
        }
    }
    impl ::core::convert::From<VerifyProofCall> for RiscZeroGroth16VerifierCalls {
        fn from(value: VerifyProofCall) -> Self {
            Self::VerifyProof(value)
        }
    }
    ///Container type for all return fields from the `verify` function with signature `verify((bytes,(bytes32,bytes32,(uint8,uint8),bytes32,bytes32)))` and selector `0x04b18430`
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
    pub struct Verify0Return(pub bool);
    ///Container type for all return fields from the `verify` function with signature `verify(bytes,bytes32,bytes32,bytes)` and selector `0x1a36ed75`
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
    pub struct Verify1Return(pub bool);
    ///Container type for all return fields from the `verify` function with signature `verify(bytes,bytes32,bytes32,bytes32)` and selector `0x6efef009`
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
    pub struct Verify2Return(pub bool);
    ///Container type for all return fields from the `verifyProof` function with signature `verifyProof(uint256[2],uint256[2][2],uint256[2],uint256[4])` and selector `0x5fe8c13b`
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
    pub struct VerifyProofReturn(pub bool);
}
