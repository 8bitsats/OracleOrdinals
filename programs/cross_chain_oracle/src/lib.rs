use anchor_lang::prelude::*;
use solana_program::program_pack::{IsInitialized, Pack, Sealed};
use solana_program::pubkey::Pubkey;
use solana_program::program_error::ProgramError;
use solana_program::keccak::hash as keccak_hash;
use solana_program::secp256k1_recover::{Secp256k1RecoverError, Secp256k1Pubkey, Secp256k1Signature};

declare_id!("Fg6PaFpoGXkYsidMpWxqSWpmyoMaMdHk4Zr1Dz4V7gAa");

#[program]
pub mod cross_chain_oracle {
    use super::*;

    pub fn verify_and_update(
        ctx: Context<VerifyAndUpdate>,
        pubkey: [u8; 32],
        signature: Vec<u8>,
        tx_hash: Vec<u8>
    ) -> ProgramResult {
        // Validate pubkey
        let pubkey = Pubkey::new(&pubkey);

        // Compute the hash of the transaction
        let tx_hash = keccak_hash(&tx_hash);

        // Perform signature verification
        let secp256k1_pubkey = Secp256k1Pubkey::new(pubkey.to_bytes());
        let secp256k1_signature = Secp256k1Signature::new(&signature);
        
        let result = secp256k1_pubkey.verify(&tx_hash.to_bytes(), &secp256k1_signature);
        require!(result.is_ok(), CustomError::InvalidZKP);

        // Update the metadata if verification is successful
        let nft = &mut ctx.accounts.nft;
        nft.metadata = "New metadata based on verified ordinal".to_string();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyAndUpdate<'info> {
    #[account(mut)]
    pub nft: Account<'info, NFT>,
}

#[account]
pub struct NFT {
    pub metadata: String,
}

#[error]
pub enum CustomError {
    #[msg("Invalid Zero-Knowledge Proof")]
    InvalidZKP,
}

impl Sealed for NFT {}

impl IsInitialized for NFT {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Pack for NFT {
    const LEN: usize = 1024;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let metadata_bytes = self.metadata.as_bytes();
        let metadata_len = metadata_bytes.len();
        dst[..metadata_len].copy_from_slice(metadata_bytes);
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let metadata = String::from_utf8(src.to_vec()).map_err(|_| ProgramError::InvalidAccountData)?;
        Ok(NFT { metadata })
    }
}
