use anchor_lang::prelude::*;

declare_id!("9aLnL3iN99rJNz9VqQrEzijLDN5dcHfnCoitR6zbWYFM");

#[program]
pub mod worker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        let data = &mut ctx.accounts.data;
        data.authority = authority;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let data = &mut ctx.accounts.data;
        data.value += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    
    #[account(init, payer=user, space=8+8+32)]
    data: Account<'info, Data>,

    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    data: Account<'info, Data>,

    authority: Signer<'info>
}

#[account]
pub struct Data {
    value: u64,

    // New! now we need this in order to be incremented
    // note that this doesn't have to be the 'user' who paid the rent.
    authority: Pubkey, 
}
