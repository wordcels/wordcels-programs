use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Incorrect hash")]
    IncorrectHash,
    #[msg("Must claim by signing with the signer account.")]
    MustClaimWithSigner,
    #[msg("Must claim by passing the word in.")]
    MustClaimWithHash,
}
