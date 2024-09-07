#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::account_info::AccountInfo;
    use solana_program::pubkey::Pubkey;
    use solana_program::rent::Rent;
    use solana_program::sysvar::Sysvar;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn create_account_info(key: Pubkey, is_signer: bool, is_writable: bool, lamports: u64, data_len: usize) -> AccountInfo {
        let lamports = Rc::new(RefCell::new(lamports));
        let data = Rc::new(RefCell::new(vec![0; data_len]));
        AccountInfo {
            key: &key,
            is_signer,
            is_writable,
            lamports,
            data,
            owner: &Pubkey::default(),
            executable: false,
            rent_epoch: 0,
        }
    }

    #[test]
    fn test_process_mint() {
        let program_id = Pubkey::new_unique();
        let fee_payer_key = Pubkey::new_unique();
        let mint_key = Pubkey::new_unique();
        let gem_key = Pubkey::new_unique();
        let system_program_key = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        let funding_key = Pubkey::new_unique();
        let assoc_token_account_key = Pubkey::new_unique();
        let wallet_key = Pubkey::new_unique();
        let token_program_key = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        let owner_key = Pubkey::new_unique();
        let csl_spl_token_v0_0_0_key = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
        let csl_spl_assoc_token_v0_0_0_key = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();

        let fee_payer_info = create_account_info(fee_payer_key, true, true, 1000000, 0);
        let mint_info = create_account_info(mint_key, true, true, 1000000, 82);
        let gem_info = create_account_info(gem_key, false, true, 1000000, 364);
        let system_program_info = create_account_info(system_program_key, false, false, 0, 0);
        let funding_info = create_account_info(funding_key, true, true, 1000000, 0);
        let assoc_token_account_info = create_account_info(assoc_token_account_key, false, true, 1000000, 0);
        let wallet_info = create_account_info(wallet_key, false, false, 0, 0);
        let token_program_info = create_account_info(token_program_key, false, false, 0, 0);
        let owner_info = create_account_info(owner_key, true, false, 0, 0);
        let csl_spl_token_v0_0_0_info = create_account_info(csl_spl_token_v0_0_0_key, false, false, 0, 0);
        let csl_spl_assoc_token_v0_0_0_info = create_account_info(csl_spl_assoc_token_v0_0_0_key, false, false, 0, 0);

        let accounts = vec![
            fee_payer_info,
            mint_info,
            gem_info,
            system_program_info,
            funding_info,
            assoc_token_account_info,
            wallet_info,
            token_program_info,
            owner_info,
            csl_spl_token_v0_0_0_info,
            csl_spl_assoc_token_v0_0_0_info,
        ];

        let color = "red".to_string();
        let rarity = "rare".to_string();
        let short_description = "A rare red gem".to_string();

        let result = Processor::process_mint(&program_id, &accounts, color, rarity, short_description);

        assert!(result.is_ok());
    }
}
