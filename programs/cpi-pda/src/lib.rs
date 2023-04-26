use anchor_lang::prelude::*;

declare_id!("8xEfwJb65GHD9fogdWWSxmt8gQj1eoCin1BCXC99FzC7");

#[program]
pub mod cpi_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
