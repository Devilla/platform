use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	NonPdaaccountWithOneField,
};

/// Test `close` with Non-PDA account and 4 signers
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` signer_1: [AccountInfo] 
/// 2. `[signer]` signer_2: [AccountInfo] 
/// 3. `[signer]` signer_3: [AccountInfo] 
/// 4. `[writable]` account: [NonPdaaccountWithOneField] 
pub fn instruction_17(
	program_id: &Pubkey,
	account: &mut Account<NonPdaaccountWithOneField>,
) -> ProgramResult {
	msg!("Account.data.field_1 = {}", account.data.field_1);

    Ok(())
}