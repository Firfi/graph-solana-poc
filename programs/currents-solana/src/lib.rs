use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod currents_solana {
    use super::*;

    pub fn add_user(ctx: Context<AddUser>, stripe_id: String, name: String) -> Result<()> {
        // require_keys_eq!(
        //     game.current_player(),
        //     ctx.accounts.player.key(),
        //     TicTacToeError::NotPlayersTurn
        // );
        let meta = &mut ctx.accounts.user_meta;
        if stripe_id.as_bytes().len() > 200 {
            // proper error handling omitted for brevity
            panic!();
        }
        if name.as_bytes().len() > 200 {
            // proper error handling omitted for brevity
            panic!();
        }
        meta.stripe_id = stripe_id;
        meta.name = name;
        meta.bump = *ctx.bumps.get("user_meta").unwrap();
        Ok(())
    }

    pub fn add_blog(ctx: Context<AddBlog>, name: String) -> Result<()> {
        // require_keys_eq!(
        //     game.current_player(),
        //     ctx.accounts.player.key(),
        //     TicTacToeError::NotPlayersTurn
        // );
        let meta = &mut ctx.accounts.blog_meta;
        if name.as_bytes().len() > 200 {
            // proper error handling omitted for brevity
            panic!();
        }
        meta.name = name;
        meta.bump = *ctx.bumps.get("blog_meta").unwrap();
        Ok(())
    }

    pub fn add_subscription(ctx: Context<AddSubscription>) -> Result<()> {
        let edge = &mut ctx.accounts.edge;
        edge.from = ctx.accounts.user_meta.key();
        edge.to = ctx.accounts.blog_meta.key();
        edge.bump = *ctx.bumps.get("edge").unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct UserMeta {
    name: String,
    stripe_id: String,
    bump: u8
}

impl UserMeta {
    pub const MAXIMUM_SIZE: usize = 200; // TODO
}

#[account]
pub struct BlogMeta {
    name: String,
    bump: u8
}

impl BlogMeta {
    pub const MAXIMUM_SIZE: usize = 200; // TODO
}

#[account]
pub struct SubscriptionEdgeMeta {
    from: Pubkey, // TODO can we type it?
    to: Pubkey, // TODO can we type it?
    bump: u8
}

impl SubscriptionEdgeMeta {
    pub const MAXIMUM_SIZE: usize = 200; // TODO
}

#[derive(Accounts)]
#[instruction(stripe_id: String, name: String)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init,
    payer = user,
    space = 8 + UserMeta::MAXIMUM_SIZE, seeds = [b"nodes/users", name.as_bytes()], bump
    )]
    pub user_meta: Account<'info, UserMeta>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct AddBlog<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init,
    payer = user,
    space = 8 + BlogMeta::MAXIMUM_SIZE, seeds = [b"nodes/blogs", name.as_bytes()], bump
    )]
    pub blog_meta: Account<'info, BlogMeta>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddSubscription<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account()]
    pub user_meta: Account<'info, UserMeta>,
    #[account()]
    pub blog_meta: Account<'info, BlogMeta>,
    #[account(
    init,
    payer = user,
    space = 8 + SubscriptionEdgeMeta::MAXIMUM_SIZE, seeds = [b"edges/subscriptions", user_meta.key().as_ref(), blog_meta.key().as_ref()], bump
    )]
    pub edge: Account<'info, SubscriptionEdgeMeta>, // , user_meta.key().as_ref(), blog_meta.key().as_ref()
    pub system_program: Program<'info, System>,
}