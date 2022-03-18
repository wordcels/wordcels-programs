use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::Hash;

/// Account that holds NFT vault info.
#[account]
#[derive(Default)]
pub struct NftVaultInfo {
    /// Token account that holds the NFT.
    pub account: Pubkey,
    /// The keccak256 hash of the word for this NFT.
    pub hash: [u8; 32],
    /// Whether the NFT has been unlocked or not.
    pub unlocked: bool,
    /// Optional. If set, this signer must sign to unlock the NFT.
    pub signer: Pubkey,
    /// The amount of lamports to deposit for unlock.
    pub amount: u64,
}
