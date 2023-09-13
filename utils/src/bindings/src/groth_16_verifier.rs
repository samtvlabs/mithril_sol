pub use groth_16_verifier::*;
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
pub mod groth_16_verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
    pub static GROTH16VERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x06\x87\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c_\xE8\xC1;\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x05\xF7V[a\0WV[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0a\x05jV[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x81\x10a\0\x8FW`\0\x80R` `\0\xF3[PV[`\0`@Q\x83\x81R\x84` \x82\x01R\x85`@\x82\x01R`@\x81``\x83`\x07a\x07\xD0Z\x03\xFA\x91P\x81a\0\xC5W`\0\x80R` `\0\xF3[\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83`\x80\x83`\x06a\x07\xD0Z\x03\xFA\x91PP\x80a\0\xF6W`\0\x80R` `\0\xF3[PPPPPV[\x7F\x0B\xAEN\x82I\xE1jz`\xCC\xD0!_\xBE\t\xB3\x85\x0E\x8C\x18!:\xCD\xB7\xF3\xE7x\xE89\t\xCBU\x85R\x7F)Z \xC0`\x88.\xBB\x89\xC03\x9Fg\xC0e\x9B\x89\xAB#\x9B\xFA\x129\x94=ns\x84\x18\x04\xE1\x1C` \x86\x01R`\0`\x80\x86\x01\x86a\x01\x9A\x875\x7F\x1B\x88\xE5\xDDT\"\xD4\xC0\x0B\xDA+\xC1\x8B\xC9\xACZ\xBA\"\xB3\x0F\x81\xD6\xF6\xF6\x86 \"k\x98J\xC2f\x7F\tK\xD9\xC4\xB3\x86\x94\x96\xBC\xF8\xAB\x0Fs<\xC6\xB6BC\xB5\xD4\x08\xD0\x9Bp\x1B\x16\xE7\xDD\xD1$\xB1~\x84a\0\x92V[a\x01\xEA` \x88\x015\x7F\x0Fh\x0C\x08c\x0F\x99\x8C_\xAC\x17nY<\x8C~\xE2y\x10[\xAF\xF2\xD9:\xEA\xEDZZ\xDD\x0C\xCF\xD6\x7F!\x10\xDC\xCD\xAF\x8A\xE0*\x1D\xA4[\x13\x9C\x06\xDF4So\x19\xE6\x82mf\x87\x94 u\x1AA\x9C\x1A\x10\x84a\0\x92V[a\x02:`@\x88\x015\x7F%\xEE\x06\xC4\xF9B\x94q\x04\x99H\x19\x9B)s\xA58\xD9\xBA\xFFT%\xB5,\xE9x\xBC\x88T}\xDC#\x7F\x0Ee\xC1\xC9\xABi\x13<.s\x99\xAA\x17.\xB5\xA8\x9C\xCB~{\xA8D\xA1\x0F\x87\xC8\xCCr\x9A\x0B;y\x84a\0\x92V[a\x02\x8A``\x88\x015\x7F#\xFD\xAD\x98\xB6M\xBC\xBC\x18f\xFA\xF6\xF7\xDA=v\x9C\x91i\x9B\xB7)\xC9Bq\xDCB\x88\x11\xD9^Q\x7F\x0B|v9\xA6L\xB5PX1\xEB\t\xE7]A\x86,c\xB1\xF3\xECU\xFCC*`\xB9\x89\xEE\xAA\xD5\xA5\x84a\0\x92V[P\x825\x81R\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG` \x84\x015\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x03\x06` \x82\x01R\x835`@\x82\x01R` \x84\x015``\x82\x01R`@\x84\x015`\x80\x82\x01R``\x84\x015`\xA0\x82\x01R\x7F-M\x9A\xA7\xE3\x02\xD9\xDFAt\x9DU\x07\x94\x9D\x05\xDB\xEA3\xFB\xB1ld;\"\xF5\x99\xA2\xBEm\xF2\xE2`\xC0\x82\x01R\x7F\x14\xBE\xDDP<7\xCE\xB0a\xD8\xEC` \x9F\xE3E\xCE\x89\x83\n\x19#\x03\x01\xF0v\xCA\xFF\0M\x19&`\xE0\x82\x01R\x7F\tg\x03/\xCB\xF7v\xD1\xAF\xC9\x85\xF8\x88w\xF1\x82\xD3\x84\x80\xA6S\xF2\xDE\xCA\xA9yL\xBC;\xF3\x06\x0Ca\x01\0\x82\x01R\x7F\x0E\x18xG\xADLy\x83t\xD0\xD6s+\xF5\x01\x84}\xD6\x8B\xC0\xE0q$\x1E\x02\x13\xBC\x7F\xC1=\xB7\xABa\x01 \x82\x01R\x7F0L\xFB\xD1\xE0\x8ApJ\x99\xF5\xE8G\xD9?\x8C<\xAA\xFD\xDE\xC4kz\r7\x9D\xA6\x9AM\x11#F\xA7a\x01@\x82\x01R\x7F\x179\xC1\xB1\xA4W\xA8\xC711#\xD2M/\x91\x92\xF8\x96\xB7\xC6>\xEA\x05\xA9\xD5\x7F\x06Tz\xD0\xCE\xC8a\x01`\x82\x01R`\0\x87\x01Qa\x01\x80\x82\x01R` `\0\x01\x87\x01Qa\x01\xA0\x82\x01R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2a\x01\xC0\x82\x01R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xEDa\x01\xE0\x82\x01R\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[a\x02\0\x82\x01R\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAAa\x02 \x82\x01R\x845a\x02@\x82\x01R` \x85\x015a\x02`\x82\x01R\x7F(\xF1YE\xC0\xD3P\xDCy\xC3\xFA\x9A\xEA\xE5j|op\xA3\x8B\xAA\xA4\x8B\xBD\xA1\x951\xAC\x05d3ya\x02\x80\x82\x01R\x7F\x14\xFCwj|\xE2\x8A\xFA\x94d\x9F8\xF3s\xA6\x1Fd\xAC\x98L\x02\xEE\xE7\xEB\x8A6\xE6\x8A\xC6\x9E\xDF\xEDa\x02\xA0\x82\x01R\x7F+\x13Z%\xCE\x83L!\x81\x81\x85\xF0w\x9D\xDA\xF6_\x83\x1Dl\xB9A\xD7Jn\x8A\x10\xCE\x1B\x0E\x05aa\x02\xC0\x82\x01R\x7F/A\xF6\xB9\xA1qC?*.\xAA\x04B\xB6\xCC\x89x^\xDD\xCB\xD2i\xE2\xA48m\xBD\x91\xF6%;\x84a\x02\xE0\x82\x01R` \x81a\x03\0\x83`\x08a\x07\xD0Z\x03\xFA\x90Q\x16\x96\x95PPPPPPV[`@Qa\x03\x80\x81\x01`@Ra\x05\x82`\0\x84\x015a\0^V[a\x05\x8F` \x84\x015a\0^V[a\x05\x9C`@\x84\x015a\0^V[a\x05\xA9``\x84\x015a\0^V[a\x05\xB6`\x80\x84\x015a\0^V[a\x05\xC3\x81\x84\x86\x88\x8Aa\0\xFDV[\x90P\x80`\0R` `\0\xF3[\x80`@\x81\x01\x83\x10\x15a\x05\xE0W`\0\x80\xFD[\x92\x91PPV[\x80`\x80\x81\x01\x83\x10\x15a\x05\xE0W`\0\x80\xFD[`\0\x80`\0\x80a\x01\x80\x85\x87\x03\x12\x15a\x06\x0EW`\0\x80\xFD[a\x06\x18\x86\x86a\x05\xCFV[\x93Pa\x06'\x86`@\x87\x01a\x05\xE6V[\x92Pa\x066\x86`\xC0\x87\x01a\x05\xCFV[\x91Pa\x06F\x86a\x01\0\x87\x01a\x05\xE6V[\x90P\x92\x95\x91\x94P\x92PV\xFE\xA2dipfsX\"\x12 Y\xF1\xB9N\xFB\x84\x1F\x9C\x9F\xBEcy\xA9\x96n\x1BF\xE0\xCF\x0F\xEE\x1F\x84\x11\xADt\xC1\xE5\xFC_\xFD\x1DdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static GROTH16VERIFIER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c_\xE8\xC1;\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x05\xF7V[a\0WV[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0a\x05jV[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x81\x10a\0\x8FW`\0\x80R` `\0\xF3[PV[`\0`@Q\x83\x81R\x84` \x82\x01R\x85`@\x82\x01R`@\x81``\x83`\x07a\x07\xD0Z\x03\xFA\x91P\x81a\0\xC5W`\0\x80R` `\0\xF3[\x82Q`@\x82\x01R` \x83\x01Q``\x82\x01R`@\x83`\x80\x83`\x06a\x07\xD0Z\x03\xFA\x91PP\x80a\0\xF6W`\0\x80R` `\0\xF3[PPPPPV[\x7F\x0B\xAEN\x82I\xE1jz`\xCC\xD0!_\xBE\t\xB3\x85\x0E\x8C\x18!:\xCD\xB7\xF3\xE7x\xE89\t\xCBU\x85R\x7F)Z \xC0`\x88.\xBB\x89\xC03\x9Fg\xC0e\x9B\x89\xAB#\x9B\xFA\x129\x94=ns\x84\x18\x04\xE1\x1C` \x86\x01R`\0`\x80\x86\x01\x86a\x01\x9A\x875\x7F\x1B\x88\xE5\xDDT\"\xD4\xC0\x0B\xDA+\xC1\x8B\xC9\xACZ\xBA\"\xB3\x0F\x81\xD6\xF6\xF6\x86 \"k\x98J\xC2f\x7F\tK\xD9\xC4\xB3\x86\x94\x96\xBC\xF8\xAB\x0Fs<\xC6\xB6BC\xB5\xD4\x08\xD0\x9Bp\x1B\x16\xE7\xDD\xD1$\xB1~\x84a\0\x92V[a\x01\xEA` \x88\x015\x7F\x0Fh\x0C\x08c\x0F\x99\x8C_\xAC\x17nY<\x8C~\xE2y\x10[\xAF\xF2\xD9:\xEA\xEDZZ\xDD\x0C\xCF\xD6\x7F!\x10\xDC\xCD\xAF\x8A\xE0*\x1D\xA4[\x13\x9C\x06\xDF4So\x19\xE6\x82mf\x87\x94 u\x1AA\x9C\x1A\x10\x84a\0\x92V[a\x02:`@\x88\x015\x7F%\xEE\x06\xC4\xF9B\x94q\x04\x99H\x19\x9B)s\xA58\xD9\xBA\xFFT%\xB5,\xE9x\xBC\x88T}\xDC#\x7F\x0Ee\xC1\xC9\xABi\x13<.s\x99\xAA\x17.\xB5\xA8\x9C\xCB~{\xA8D\xA1\x0F\x87\xC8\xCCr\x9A\x0B;y\x84a\0\x92V[a\x02\x8A``\x88\x015\x7F#\xFD\xAD\x98\xB6M\xBC\xBC\x18f\xFA\xF6\xF7\xDA=v\x9C\x91i\x9B\xB7)\xC9Bq\xDCB\x88\x11\xD9^Q\x7F\x0B|v9\xA6L\xB5PX1\xEB\t\xE7]A\x86,c\xB1\xF3\xECU\xFCC*`\xB9\x89\xEE\xAA\xD5\xA5\x84a\0\x92V[P\x825\x81R\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG` \x84\x015\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x03\x06` \x82\x01R\x835`@\x82\x01R` \x84\x015``\x82\x01R`@\x84\x015`\x80\x82\x01R``\x84\x015`\xA0\x82\x01R\x7F-M\x9A\xA7\xE3\x02\xD9\xDFAt\x9DU\x07\x94\x9D\x05\xDB\xEA3\xFB\xB1ld;\"\xF5\x99\xA2\xBEm\xF2\xE2`\xC0\x82\x01R\x7F\x14\xBE\xDDP<7\xCE\xB0a\xD8\xEC` \x9F\xE3E\xCE\x89\x83\n\x19#\x03\x01\xF0v\xCA\xFF\0M\x19&`\xE0\x82\x01R\x7F\tg\x03/\xCB\xF7v\xD1\xAF\xC9\x85\xF8\x88w\xF1\x82\xD3\x84\x80\xA6S\xF2\xDE\xCA\xA9yL\xBC;\xF3\x06\x0Ca\x01\0\x82\x01R\x7F\x0E\x18xG\xADLy\x83t\xD0\xD6s+\xF5\x01\x84}\xD6\x8B\xC0\xE0q$\x1E\x02\x13\xBC\x7F\xC1=\xB7\xABa\x01 \x82\x01R\x7F0L\xFB\xD1\xE0\x8ApJ\x99\xF5\xE8G\xD9?\x8C<\xAA\xFD\xDE\xC4kz\r7\x9D\xA6\x9AM\x11#F\xA7a\x01@\x82\x01R\x7F\x179\xC1\xB1\xA4W\xA8\xC711#\xD2M/\x91\x92\xF8\x96\xB7\xC6>\xEA\x05\xA9\xD5\x7F\x06Tz\xD0\xCE\xC8a\x01`\x82\x01R`\0\x87\x01Qa\x01\x80\x82\x01R` `\0\x01\x87\x01Qa\x01\xA0\x82\x01R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2a\x01\xC0\x82\x01R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xEDa\x01\xE0\x82\x01R\x7F\t\x06\x89\xD0X_\xF0u\xEC\x9E\x99\xADi\x0C3\x95\xBCK13p\xB3\x8E\xF3U\xAC\xDA\xDC\xD1\"\x97[a\x02\0\x82\x01R\x7F\x12\xC8^\xA5\xDB\x8Cm\xEBJ\xABq\x80\x8D\xCB@\x8F\xE3\xD1\xE7i\x0CC\xD3{L\xE6\xCC\x01f\xFA}\xAAa\x02 \x82\x01R\x845a\x02@\x82\x01R` \x85\x015a\x02`\x82\x01R\x7F(\xF1YE\xC0\xD3P\xDCy\xC3\xFA\x9A\xEA\xE5j|op\xA3\x8B\xAA\xA4\x8B\xBD\xA1\x951\xAC\x05d3ya\x02\x80\x82\x01R\x7F\x14\xFCwj|\xE2\x8A\xFA\x94d\x9F8\xF3s\xA6\x1Fd\xAC\x98L\x02\xEE\xE7\xEB\x8A6\xE6\x8A\xC6\x9E\xDF\xEDa\x02\xA0\x82\x01R\x7F+\x13Z%\xCE\x83L!\x81\x81\x85\xF0w\x9D\xDA\xF6_\x83\x1Dl\xB9A\xD7Jn\x8A\x10\xCE\x1B\x0E\x05aa\x02\xC0\x82\x01R\x7F/A\xF6\xB9\xA1qC?*.\xAA\x04B\xB6\xCC\x89x^\xDD\xCB\xD2i\xE2\xA48m\xBD\x91\xF6%;\x84a\x02\xE0\x82\x01R` \x81a\x03\0\x83`\x08a\x07\xD0Z\x03\xFA\x90Q\x16\x96\x95PPPPPPV[`@Qa\x03\x80\x81\x01`@Ra\x05\x82`\0\x84\x015a\0^V[a\x05\x8F` \x84\x015a\0^V[a\x05\x9C`@\x84\x015a\0^V[a\x05\xA9``\x84\x015a\0^V[a\x05\xB6`\x80\x84\x015a\0^V[a\x05\xC3\x81\x84\x86\x88\x8Aa\0\xFDV[\x90P\x80`\0R` `\0\xF3[\x80`@\x81\x01\x83\x10\x15a\x05\xE0W`\0\x80\xFD[\x92\x91PPV[\x80`\x80\x81\x01\x83\x10\x15a\x05\xE0W`\0\x80\xFD[`\0\x80`\0\x80a\x01\x80\x85\x87\x03\x12\x15a\x06\x0EW`\0\x80\xFD[a\x06\x18\x86\x86a\x05\xCFV[\x93Pa\x06'\x86`@\x87\x01a\x05\xE6V[\x92Pa\x066\x86`\xC0\x87\x01a\x05\xCFV[\x91Pa\x06F\x86a\x01\0\x87\x01a\x05\xE6V[\x90P\x92\x95\x91\x94P\x92PV\xFE\xA2dipfsX\"\x12 Y\xF1\xB9N\xFB\x84\x1F\x9C\x9F\xBEcy\xA9\x96n\x1BF\xE0\xCF\x0F\xEE\x1F\x84\x11\xADt\xC1\xE5\xFC_\xFD\x1DdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static GROTH16VERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Groth16Verifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Groth16Verifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Groth16Verifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Groth16Verifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Groth16Verifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Groth16Verifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Groth16Verifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GROTH16VERIFIER_ABI.clone(),
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
                GROTH16VERIFIER_ABI.clone(),
                GROTH16VERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
    for Groth16Verifier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
