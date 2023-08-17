use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserAccount {
    pub name: String,
    pub authority: Pubkey,
    pub last_post_id: u8,
}


#[account]
#[derive(Default)]
pub struct TextAccount {
    pub id: u8,
    pub text: String,
    pub user: Pubkey,
    pub authority: Pubkey,
}
