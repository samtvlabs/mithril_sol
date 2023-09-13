// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{error::Error, io::Write};

use blake2::{digest::consts::U32, Blake2b};
use ethabi::{ethereum_types::H256, Bytes};
use flate2::{write::ZlibEncoder, Compression};
use merkle_tree::MerkleTree;
use mithril_stm::{
    key_reg::KeyReg,
    stm::{StmAggrSig, StmClerk, StmInitializer, StmParameters, StmSig, StmSigner},
};
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

#[derive(Clone, Debug)]
struct Chunk([u8; 32]);

type H = Blake2b<U32>;

type D = Blake2b<U32>;
pub type Stake = u64;

#[allow(dead_code)]
#[derive(Debug)]
pub struct VerificationData {
    msg: H256,
    msig: H256,
}

impl VerificationData {
    #[allow(dead_code)]
    fn new(msg: H256, msig: H256) -> Self {
        VerificationData { msg, msig }
    }
}

use std::fmt;

fn setup_equal_parties(params: StmParameters, nparties: usize) -> Vec<StmSigner<D>> {
    let stake = vec![1; nparties];
    setup_parties(params, stake)
}

fn setup_parties(params: StmParameters, stake: Vec<Stake>) -> Vec<StmSigner<D>> {
    let mut kr = KeyReg::init();
    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);

    #[allow(clippy::needless_collect)]
    let ps = stake
        .into_iter()
        .map(|stake| {
            let p = StmInitializer::setup(params, stake, &mut rng);
            kr.register(stake, p.verification_key()).unwrap();
            p
        })
        .collect::<Vec<_>>();
    let closed_reg = kr.close();
    ps.into_iter()
        .map(|p| p.new_signer(closed_reg.clone()).unwrap())
        .collect()
}

fn find_signatures(msg: &[u8], ps: &[StmSigner<D>], is: &[usize]) -> Vec<StmSig> {
    let mut sigs = Vec::new();
    for i in is {
        if let Some(sig) = ps[*i].sign(msg) {
            sigs.push(sig);
        }
    }
    sigs
}

#[allow(dead_code)]
fn generate_aggregate_signatures() -> StmAggrSig<H> {
    // Initialize parameters and RNG
    let params = StmParameters {
        k: 357,
        m: 2642,
        phi_f: 0.2,
    };

    let nparties = 4;

    let ps = setup_equal_parties(params, 4);

    let clerk = StmClerk::from_signer(&ps[0]);

    let all_ps: Vec<usize> = (0..nparties).collect();
    let msg_vec: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    let msg: &[u8] = &msg_vec;
    let sigs = find_signatures(&msg, &ps, &all_ps);
    let msig = clerk.aggregate(&sigs, &msg).unwrap();

    // println!("Aggregate Signature {:?}, msg {:?}", msig, msg);

    msig
}

// TODO: We need to be able to encode the input from the smart contract i.e.
// receive the certifcate and serialise it somehow . Mostl likely just as a byte
// array. An issue might arise is its longer than 256. Then we need to get
// creative, like break it down , and send it  over in chunks
#[allow(dead_code)]

fn verify_aggregate_signature(msg: Bytes, msig: StmAggrSig<H>) -> bool {
    // Initialize parameters
    let params = StmParameters {
        k: 357,
        m: 2642,
        phi_f: 0.2,
    };

    let ps = setup_equal_parties(params, 4);

    // Create a clerk from the aggregate verification key
    let clerk = StmClerk::from_signer(&ps[0]);

    let verify_result = msig.verify(&msg, &clerk.compute_avk(), &params);

    match verify_result {
        Ok(_) => {
            println!("Verification successful");
            true
        }
        Err(_) => {
            println!("Verification failed");
            false
        }
    }
}

#[allow(dead_code)]

fn split_into_256_bit_chunks(msig: &StmAggrSig<H>) -> Vec<[u8; 32]> {
    let msig_bytes = msig.to_bytes();
    let mut chunks = Vec::new();

    for i in 0..(msig_bytes.len() / 32) {
        let start = i * 32;
        let end = start + 32;
        let chunk = &msig_bytes[start..end];
        let mut chunk_256_bits = [0u8; 32];
        chunk_256_bits.copy_from_slice(chunk);
        chunks.push(chunk_256_bits);
    }

    chunks
}

#[allow(dead_code)]

fn compress_msig(msig: &StmAggrSig<H>) -> Vec<u8> {
    let msig_bytes = msig.to_bytes();

    // Create a compressor with zlib compression algorithm
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

    // Write the msig bytes to the compressor
    encoder.write_all(&msig_bytes).unwrap();

    // Finish the compression and retrieve the compressed bytes
    let compressed_bytes = encoder.finish().unwrap();

    compressed_bytes
}

fn main() {
    let msg = Bytes::from(vec![0, 1, 2, 3, 4, 5]);
    let msig = generate_aggregate_signatures();
    println!("msig: {:?}", msig.to_bytes());
    println!("msg: {:?}", msg);
}
