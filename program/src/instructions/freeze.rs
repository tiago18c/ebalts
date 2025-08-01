use solana_cpi::invoke_signed;
use solana_program::account_info::AccountInfo;
use solana_program_error::{ProgramError, ProgramResult};

use crate::{
    error::EbaltsError,
    state::{load_mint_config, MintConfig},
};

pub struct Freeze<'a> {
    pub authority: &'a AccountInfo<'a>,
    pub mint: &'a AccountInfo<'a>,
    pub token_account: &'a AccountInfo<'a>,
    pub mint_config: &'a AccountInfo<'a>,
    pub token_program: &'a AccountInfo<'a>,
}

impl Freeze<'_> {
    pub const DISCRIMINATOR: u8 = 5;

    pub fn process(&self) -> ProgramResult {
        let data = &self.mint_config.data.borrow();
        let config = load_mint_config(data)?;

        if config.freeze_authority != *self.authority.key {
            return Err(EbaltsError::InvalidAuthority.into());
        }

        if config.mint != *self.mint.key {
            return Err(EbaltsError::InvalidTokenMint.into());
        }

        let bump_seed = [config.bump];
        let seeds = [MintConfig::SEED_PREFIX, self.mint.key.as_ref(), &bump_seed];

        let ix = spl_token_2022::instruction::freeze_account(
            self.token_program.key,
            self.token_account.key,
            self.mint.key,
            self.mint_config.key,
            &[],
        )?;
        invoke_signed(
            &ix,
            &[
                self.token_account.clone(),
                self.mint.clone(),
                self.mint_config.clone(),
            ],
            &[&seeds],
        )?;

        Ok(())
    }
}

impl<'a> TryFrom<&'a [AccountInfo<'a>]> for Freeze<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo<'a>]) -> Result<Self, Self::Error> {
        let [authority, mint, token_account, mint_config, token_program] = &accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !authority.is_signer {
            return Err(EbaltsError::InvalidAuthority.into());
        }

        if !spl_token_2022::check_id(token_program.key) {
            return Err(EbaltsError::InvalidTokenProgram.into());
        }

        Ok(Self {
            authority,
            mint,
            token_account,
            mint_config,
            token_program,
        })
    }
}
