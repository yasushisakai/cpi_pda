use anchor_lang::prelude::*;

use worker::cpi::accounts::Increment;
use worker::program::Worker;
use worker::{self, Data};

declare_id!("8xEfwJb65GHD9fogdWWSxmt8gQj1eoCin1BCXC99FzC7");

#[program]
pub mod cpi_pda {
    use super::*;

    pub fn inc_through_pda(ctx: Context<IncrementThroughPDA>, bump: u8) -> Result<()> {
        let bump = &[bump][..];
        worker::cpi::increment(ctx.accounts.set_ctx().with_signer(&[&[bump][..]]))
    }
}

#[derive(Accounts)]
pub struct IncrementThroughPDA<'info> {
    #[account(mut)]
    pub worker_data: Account<'info, Data>,

    pub worker_program: Program<'info, Worker>,

    /// CHECK: only used for signing the pda
    pub authority: UncheckedAccount<'info>,
}

impl<'info> IncrementThroughPDA<'info> {
    pub fn set_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Increment<'info>> {
        let cpi_program = self.worker_program.to_account_info();
        let cpi_accounts = Increment {
            data: self.worker_data.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
