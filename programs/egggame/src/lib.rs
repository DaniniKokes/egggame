use anchor_lang::prelude::*;

// ID programu - toto je unikátní identifikátor tvého smart kontraktu
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod egggame {
    use super::*;

    // Funkce pro inicializaci - nastaví počáteční skóre na 0
    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.score = 0;
        game.authority = ctx.accounts.user.key();
        msg!("Hra inicializována!");
        Ok(())
    }

    // Funkce pro chycení vajíčka - přičte bod
    pub fn catch_egg(ctx: Context<CatchEgg>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.score += 1;
        msg!("Vajíčko chyceno! Skóre: {}", game.score);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CatchEgg<'info> {
    #[account(mut, has_one = authority)]
    pub game: Account<'info, Game>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Game {
    pub score: u64,
    pub authority: Pubkey,
}