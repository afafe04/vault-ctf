use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgZ4v5G5LGBf");

#[program]
pub mod vault_ctf {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, _bump: u8, flag: [u8; 32]) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.user.key();
        vault.balance = 100_000_000;
        vault.flag = flag;
        Ok(())
    }

    pub fn withdraw(_ctx: Context<Withdraw>) -> Result<()> {
        // vulnérabilité : aucune vérification d'identité ici !
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeVault<'info> {
    #[account(init, payer = user, seeds = [b"vault", user.key().as_ref()], bump, space = 8 + 32 + 8 + 32)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub user: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
    pub flag: [u8; 32],
}
