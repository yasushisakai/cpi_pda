use anchor_lang::prelude::*;

use worker::cpi::accounts::Increment;
use worker::program::Worker;
use worker::{self, Data};

declare_id!("8xEfwJb65GHD9fogdWWSxmt8gQj1eoCin1BCXC99FzC7");

#[program]
pub mod cpi_pda {
    use super::*;

    pub fn inc_through_me(ctx: Context<IncrementThroughManager>) -> Result<()> {
        let cpi_program = ctx.accounts.worker_program.to_account_info();
        let cpi_accounts = Increment {
            data: ctx.accounts.worker_data.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        worker::cpi::increment(cpi_ctx)
    }
}

#[derive(Accounts)]
pub struct IncrementThroughManager<'info> {
    #[account(mut)]
    pub worker_data: Account<'info, Data>,

    pub worker_program: Program<'info, Worker>,

    pub authority: Signer<'info>,
}
