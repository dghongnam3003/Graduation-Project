use anchor_lang::prelude::Pubkey;


//seed for PDA account
pub const CAMPAIGN_SEED: &[u8] = b"campaign";
pub const CONFIG_SEED: &[u8] = b"config";
pub const CREATOR_SEED: &[u8] = b"creator";
pub const TREASURY_SEED: &[u8] = b"treasury";

// constants to define size of some types of variables
pub const DISCRIMINATOR: usize = std::mem::size_of::<u64>();
pub const BOOL_SIZE: usize = std::mem::size_of::<bool>();
pub const PUBKEY_SIZE: usize = std::mem::size_of::<Pubkey>();
pub const U8_SIZE: usize = std::mem::size_of::<u8>();
pub const U16_SIZE: usize = std::mem::size_of::<u16>();
pub const U32_SIZE: usize = std::mem::size_of::<u32>();
pub const U64_SIZE: usize = std::mem::size_of::<u64>();
pub const I64_SIZE: usize = std::mem::size_of::<i64>();
pub const STRING_MAX_10_CHAR_SIZE: usize = 4 + 10;
pub const STRING_MAX_32_CHAR_SIZE: usize = 4 + 32;
pub const STRING_MAX_100_CHAR_SIZE: usize = 4 + 100;

pub const PERCENTAGE_DENOMINATOR: u16 = 10000;
pub const DEFAULT_TOKEN_RESERVES: u64 = 1_073_000_000_000_000; // 1.073 billion
pub const DEFAULT_SOL_RESERVES: u64 = 30_000_000_000; // 30 SOL