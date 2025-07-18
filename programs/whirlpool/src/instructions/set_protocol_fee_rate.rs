use {
    crate::state::{Whirlpool, WhirlpoolsConfig},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct SetProtocolFeeRate<'info> {
    pub whirlpools_config: Account<'info, WhirlpoolsConfig>,

    #[account(mut, has_one = whirlpools_config)]
    pub whirlpool: Account<'info, Whirlpool>,

    #[account(address = whirlpools_config.fee_authority)]
    pub fee_authority: Signer<'info>,
}

pub fn handler(ctx: Context<SetProtocolFeeRate>, protocol_fee_rate: u16) -> Result<()> {
    ctx.accounts.whirlpool.update_protocol_fee_rate(protocol_fee_rate)
}
