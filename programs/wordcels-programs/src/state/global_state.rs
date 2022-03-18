use anchor_lang::prelude::*;

/// Account used as the authority for holding NFTs.
#[account]
#[derive(Default)] 
pub struct GlobalState {
    /// The bump seed for this account.
    pub nonce: u8
}
