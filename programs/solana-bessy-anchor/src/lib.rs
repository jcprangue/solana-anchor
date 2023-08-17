use anchor_lang::prelude::*;

pub mod constant;
pub mod states;
use crate::{constant::*, states::*};

//program ID
declare_id!("C8CAoXtCGzi9JodnZAMdf7Cex3b69TsLbcarWU3hX3ig");

#[program]
pub mod text_generator_sol {
    use super::*;

    pub fn init_user(ctx: Context<InitUser>, name: String) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.last_post_id = 0;
        user_account.authority = authority.key();

        Ok(())
    }

    pub fn generate_text(ctx: Context<GenerateText>, content: String) -> Result<()> {
        let text_account = &mut ctx.accounts.text_account;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        text_account.id = user_account.last_post_id;
        text_account.text = content;
        text_account.user = user_account.key();
        text_account.authority = authority.key();

        user_account.last_post_id = user_account.last_post_id.checked_add(1).unwrap();

        Ok(())
    }

}

#[derive(Accounts)]
#[instruction()]
pub struct InitUser<'info> {
    #[account(
        init,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 2212 + 8
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct GenerateText<'info> {
    #[account(
        init,
        seeds = [POST_SEED, authority.key().as_ref(),&[user_account.last_post_id as u8].as_ref()],
        bump,
        payer = authority,
        space = 2376 + 8
    )]
    pub text_account: Account<'info, TextAccount>,

    #[account(
        mut,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
