use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("F4nmdd2VdE1fnc2wSphDm4rAepUxULovXkWtN3m1J863");

#[program]
pub mod sol_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault_state =&mut ctx.accounts.vault_state;
        vault_state.owner = ctx.accounts.owner.key();
        vault_state.bump = ctx.bumps.vault_state;
        msg!("Vault initialized! Owner: {}", vault_state.owner);
        Ok(())
    }
}



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 1,
        seeds = [b "vault_state", owner.key().as_ref()],
        bump
    )]

    pub vault: SystemAccount <'info>,
    #[account(
        seeds = [b"vault_state", vault_state.owner.as_ref()],
         bump = vault_state.bump
    )]

    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>, 


}

#[derive(Accounts)]

pub struct Deposit<'info> {
     #[account(mut)]
     pub signer: Signer<'info>,

     #[account(
        mut,
        seeds = [b"vault", vault_state.owner.as_ref()],
        bump
     )]

     pub vault: SystemAccount<'info>,

     #[account(
        seeds = [b"vault_state", vault_state.owner.as_ref()],
         bump = vault_state.bump
     )]

    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]

pub struct Withdraw<'info> {
     #[account(mut)]
     pub owner: Signer<'info>,

     #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

     #[account(
        seeds = [b"vault_state", owner.key().as_ref()],
        bump = vault_state.bump,
        has_one = owner
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub owner: Pubkey,  // 32 bytes
    pub bump: u8,       // 1 byte
}

pub struct Initialize {}
