///`Callback((bytes,bytes32),address,bytes,uint64)`
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
pub struct Callback {
    pub auth: CallbackAuthorization,
    pub callback_contract: ::ethers::core::types::Address,
    pub payload: ::ethers::core::types::Bytes,
    pub gas_limit: u64,
}
///`CallbackAuthorization(bytes,bytes32)`
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
pub struct CallbackAuthorization {
    pub seal: ::ethers::core::types::Bytes,
    pub post_state_digest: [u8; 32],
}
///`ExitCode(uint8,uint8)`
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
pub struct ExitCode {
    pub system: u8,
    pub user: u8,
}
///`Receipt(bytes,(bytes32,bytes32,(uint8,uint8),bytes32,bytes32))`
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
pub struct Receipt {
    pub seal: ::ethers::core::types::Bytes,
    pub meta: ReceiptMetadata,
}
///`ReceiptMetadata(bytes32,bytes32,(uint8,uint8),bytes32,bytes32)`
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
pub struct ReceiptMetadata {
    pub pre_state_digest: [u8; 32],
    pub post_state_digest: [u8; 32],
    pub exit_code: ExitCode,
    pub input: [u8; 32],
    pub output: [u8; 32],
}
///`FuzzSelector(address,bytes4[])`
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
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
