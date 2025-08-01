//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MintConfig {
    pub discriminator: u8,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub mint: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub freeze_authority: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub gating_program: Pubkey,
    pub bump: u8,
    pub enable_permissionless_thaw: bool,
    pub enable_permissionless_freeze: bool,
}

impl MintConfig {
    pub const LEN: usize = 0;

    /// Prefix values used to generate a PDA for this account.
    ///
    /// Values are positional and appear in the following order:
    ///
    ///   0. `MintConfig::PREFIX`
    ///   1. mint (`Pubkey`)
    pub const PREFIX: &'static [u8] = "MINT_CONFIG".as_bytes();

    pub fn create_pda(
        mint: Pubkey,
        bump: u8,
    ) -> Result<solana_pubkey::Pubkey, solana_pubkey::PubkeyError> {
        solana_pubkey::Pubkey::create_program_address(
            &["MINT_CONFIG".as_bytes(), mint.as_ref(), &[bump]],
            &crate::EBALTS_ID,
        )
    }

    pub fn find_pda(mint: &Pubkey) -> (solana_pubkey::Pubkey, u8) {
        solana_pubkey::Pubkey::find_program_address(
            &["MINT_CONFIG".as_bytes(), mint.as_ref()],
            &crate::EBALTS_ID,
        )
    }

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_account_info::AccountInfo<'a>> for MintConfig {
    type Error = std::io::Error;

    fn try_from(account_info: &solana_account_info::AccountInfo<'a>) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "fetch")]
pub fn fetch_mint_config(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_pubkey::Pubkey,
) -> Result<crate::shared::DecodedAccount<MintConfig>, std::io::Error> {
    let accounts = fetch_all_mint_config(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_mint_config(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_pubkey::Pubkey],
) -> Result<Vec<crate::shared::DecodedAccount<MintConfig>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::DecodedAccount<MintConfig>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i].as_ref().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Account not found: {}", address),
        ))?;
        let data = MintConfig::from_bytes(&account.data)?;
        decoded_accounts.push(crate::shared::DecodedAccount {
            address,
            account: account.clone(),
            data,
        });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_mint_config(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_pubkey::Pubkey,
) -> Result<crate::shared::MaybeAccount<MintConfig>, std::io::Error> {
    let accounts = fetch_all_maybe_mint_config(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_mint_config(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_pubkey::Pubkey],
) -> Result<Vec<crate::shared::MaybeAccount<MintConfig>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::MaybeAccount<MintConfig>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        if let Some(account) = accounts[i].as_ref() {
            let data = MintConfig::from_bytes(&account.data)?;
            decoded_accounts.push(crate::shared::MaybeAccount::Exists(
                crate::shared::DecodedAccount {
                    address,
                    account: account.clone(),
                    data,
                },
            ));
        } else {
            decoded_accounts.push(crate::shared::MaybeAccount::NotFound(address));
        }
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for MintConfig {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for MintConfig {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for MintConfig {
    fn owner() -> Pubkey {
        crate::EBALTS_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for MintConfig {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for MintConfig {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
