use {
    crate::{
        errors::ErrorCode, manager::whirlpool_manager::next_whirlpool_reward_infos, math::checked_mul_shift_right,
        state::Whirlpool, util::to_timestamp_u64,
    },
    anchor_lang::prelude::*,
    anchor_spl::token::TokenAccount,
};

const DAY_IN_SECONDS: u128 = 60 * 60 * 24;

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct SetRewardEmissions<'info> {
    #[account(mut)]
    pub whirlpool: Account<'info, Whirlpool>,

    #[account(address = whirlpool.reward_infos[reward_index as usize].authority)]
    pub reward_authority: Signer<'info>,

    #[account(address = whirlpool.reward_infos[reward_index as usize].vault)]
    pub reward_vault: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<SetRewardEmissions>, reward_index: u8, emissions_per_second_x64: u128) -> Result<()> {
    let whirlpool = &ctx.accounts.whirlpool;
    let reward_vault = &ctx.accounts.reward_vault;

    let emissions_per_day = checked_mul_shift_right(DAY_IN_SECONDS, emissions_per_second_x64)?;
    if reward_vault.amount < emissions_per_day {
        return Err(ErrorCode::RewardVaultAmountInsufficient.into());
    }

    let clock = Clock::get()?;
    let timestamp = to_timestamp_u64(clock.unix_timestamp)?;
    let next_reward_infos = next_whirlpool_reward_infos(whirlpool, timestamp)?;

    ctx.accounts.whirlpool.update_emissions(
        reward_index as usize,
        next_reward_infos,
        timestamp,
        emissions_per_second_x64,
    )
}
