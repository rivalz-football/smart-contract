use anchor_lang::solana_program::keccak;
use std::convert::TryInto;

//https://docs.chain.link/docs/chainlink-vrf-best-practices/#getting-multiple-random-number
pub fn expand(randomness: [u8; 32]) -> u32 {
    let mut hasher = keccak::Hasher::default();
    hasher.hash(&randomness);

    let mut result = [0u8; 32];
    hasher
        .result()
        .to_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, byte)| {
            result[i % 4] ^= byte;
        });

    let slice: [u8; 4] = result[0..4]
        .try_into()
        .expect("slice with incorrect length");
    u32::from_le_bytes(slice) % 2 // Returns 0 or 1
}
