use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::constants::{
  CAMPAIGN_SEED, DEFAULT_SOL_RESERVES, DEFAULT_TOKEN_RESERVES, PERCENTAGE_DENOMINATOR,
  TREASURY_SEED,
};

pub fn deposit_sol<'info>(
  from: &Signer<'info>,
  to: &AccountInfo<'info>,
  amount: u64,
  system_program: &Program<'info, System>,
) -> Result<()> {
  system_program::transfer(
    CpiContext::new(
      system_program.to_account_info(),
        system_program::Transfer {
          from: from.to_account_info(),
          to: to.to_account_info(),
        },
    ),
    amount,
  )?;

  Ok(())
}

pub fn deposit_token<'info>(
  from: &Account<'info, TokenAccount>,
  to: &Account<'info, TokenAccount>,
  amount: u64,
  authority: &Signer<'info>,
  token_program: &Program<'info, Token>,
) -> Result<()> {
  token::transfer(
    CpiContext::new(
      token_program.to_account_info(),
        token::Transfer {
          from: from.to_account_info(),
          to: to.to_account_info(),
          authority: authority.to_account_info(),
        },
    ),
    amount,
  )?;

  Ok(())
}

pub fn withdraw_sol<'info>(
  from: &AccountInfo<'info>,
  to: &AccountInfo<'info>,
  amount: u64,
) -> Result<()> {
  from.sub_lamports(amount)?;
  to.add_lamports(amount)?;

  Ok(())
}

pub fn withdraw_token_from_treasury<'info>(
  from: &Account<'info, TokenAccount>,
  to: &Account<'info, TokenAccount>,
  amount: u64,
  token_program: &Program<'info, Token>,
  authority: &AccountInfo<'info>,
  bump: u8,
) -> Result<()> {
  token::transfer(
    CpiContext::new_with_signer(
      token_program.to_account_info(),
        token::Transfer {
          from: from.to_account_info(),
          to: to.to_account_info(),
          authority: authority.to_account_info(),
        },
      &[&[TREASURY_SEED, &[bump]]],
    ),
    amount,
  )?;

  Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn withdraw_token_from_campaign<'info>(
  from: &Account<'info, TokenAccount>,
  to: &Account<'info, TokenAccount>,
  amount: u64,
  token_program: &Program<'info, Token>,
  authority: &AccountInfo<'info>,
  bump: u8,
  creator: Pubkey,
  index: u64,
) -> Result<()> {
  token::transfer(
    CpiContext::new(
      token_program.to_account_info(),
      token::Transfer {
          from: from.to_account_info(),
          to: to.to_account_info(),
          authority: authority.to_account_info(),
        },
    )
    .with_signer(&[&[
      CAMPAIGN_SEED,
      creator.as_ref(),
      index.to_le_bytes().as_ref(),
      &[bump],
    ]]),
    amount,
  )?;

  Ok(())
}

pub fn calc_out_token_amount(amount_in: u64, slippage: u16) -> u64 {
  // Convert values to u128 to prevent overflow during multiplication
  let token_reserves = DEFAULT_TOKEN_RESERVES as u128;
  let sol_reserves = DEFAULT_SOL_RESERVES as u128;
  let slippage_percentage = slippage as u128;
  let default_denom = PERCENTAGE_DENOMINATOR as u128;
  let amount = amount_in as u128;

  // Perform calculation with u128 to avoid overflow
  let numerator = token_reserves.checked_mul(amount).unwrap();
  let denominator = sol_reserves.checked_add(amount).unwrap();
  let base = numerator.checked_div(denominator).unwrap();
  let slippage = base
    .checked_mul(slippage_percentage)
    .unwrap()
    .checked_div(default_denom)
    .unwrap();

  // Convert back to u64
  base.checked_sub(slippage).unwrap() as u64
}
