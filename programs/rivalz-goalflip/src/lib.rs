use anchor_lang::solana_program;

mod access_control;
mod errors;
pub mod randomness_tools;
pub mod recent_blockhashes;
mod state;

use crate::access_control::is_admin;
use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

declare_id!("rivi27uFE2UGJCR2WmzddviqS6RWiTzciC8KnVp2rhi");
const ADMIN_PUBKEY: &[u8] = b"aut69244nPQ5A23MKwScxMiZxvsYeepBkNuaxK2TqSd";

#[program]
pub mod rivalz_goalflip {
    use super::*;

    #[access_control(is_admin(& ctx.accounts.admin))]
    pub fn init_game(ctx: Context<GameContext>) -> Result<()> {
        // %1.5 commission
        ctx.accounts.game.commission_rate = 15;
        ctx.accounts.game.created_at = Clock::get()?.unix_timestamp as u64;
        ctx.accounts.game.multiplier = 2000;
        ctx.accounts.game.name = b"myGame".to_vec();

        Ok(())
    }

    pub fn play(
        ctx: Context<PlayContext>,
        position: String,
        corner: String,
        bet_amount: u64,
    ) -> Result<()> {
        ctx.accounts.game_match.game = ctx.accounts.game.key();
        ctx.accounts.game.latest_match = ctx.accounts.game_match.key();
        ctx.accounts.game.last_play_date = Clock::get()?.unix_timestamp as u64;
        ctx.accounts.game.total_volume += bet_amount;

        ctx.accounts.game_match.status = GameMatchStatus::Pending;
        ctx.accounts.game_match.created_at = Clock::get()?.unix_timestamp as u64;
        ctx.accounts.game_match.player = ctx.accounts.player.key();

        let commission = (bet_amount * (ctx.accounts.game.commission_rate as u64)) / 1000 as u64;
        ctx.accounts.game_match.bet_amount = bet_amount - commission;
        ctx.accounts.game_match.commission_amount = commission;

        ctx.accounts.game_match.position = match parse_position(&position) {
            Some(position) => position,
            None => return Err(ErrorCode::InvalidPosition.into()),
        };

        ctx.accounts.game_match.player_corner = match parse_corner(&corner) {
            Some(corner) => corner,
            None => return Err(ErrorCode::InvalidCorner.into()),
        };

        if ctx.accounts.player.lamports() < bet_amount {
            return err!(ErrorCode::NoEnoughFund);
        }

        let transfer_to_game = solana_program::system_instruction::transfer(
            &ctx.accounts.player.key(),
            &ctx.accounts.game.key(),
            ctx.accounts.game_match.bet_amount + ctx.accounts.game_match.commission_amount,
        );
        solana_program::program::invoke(
            &transfer_to_game,
            &[
                ctx.accounts.player.to_account_info(),
                ctx.accounts.game.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    #[access_control(is_admin(& ctx.accounts.admin))]
    pub fn result_game_match(ctx: Context<ResultGameMatchContext>) -> Result<()> {
        if ctx.accounts.game_match.status != GameMatchStatus::Pending {
            return err!(ErrorCode::GameMatchAlreadyFinished);
        }

        if ctx.accounts.player.key().to_bytes() != ctx.accounts.game_match.player.to_bytes() {
            return err!(ErrorCode::WrongPlayerToResult);
        }

        let randomness =
            recent_blockhashes::last_blockhash_accessor(&ctx.accounts.recent_blockhashes)?;

        // Select a random corner
        let random_corner: Corner = match randomness_tools::expand(randomness) {
            0 => Corner::Left,
            _ => Corner::Right,
        };

        ctx.accounts.game_match.won = random_corner != ctx.accounts.game_match.player_corner;

        if ctx.accounts.game_match.won {
            msg!("you won!");
            ctx.accounts.game_match.status = GameMatchStatus::Won;

            ctx.accounts.game.win_count += 1;
            ctx.accounts.game_match.won_amount = (ctx.accounts.game_match.bet_amount
                * ctx.accounts.game.multiplier as u64)
                / 1000 as u64;

            msg!("won_amount: {}", ctx.accounts.game_match.won_amount);
            **ctx
                .accounts
                .game
                .to_account_info()
                .try_borrow_mut_lamports()? -= ctx.accounts.game_match.won_amount;

            **ctx
                .accounts
                .player
                .to_account_info()
                .try_borrow_mut_lamports()? += ctx.accounts.game_match.won_amount
        } else {
            msg!("you lost :(");
            ctx.accounts.game_match.status = GameMatchStatus::Lost;
            ctx.accounts.game.lose_count += 1;
        }

        emit!(ResultGameMatchEvent {
            game: ctx.accounts.game.key(),
            player: ctx.accounts.player.key(),
            won: ctx.accounts.game_match.won,
            won_amount: ctx.accounts.game_match.won_amount,
            bet_amount: ctx.accounts.game_match.bet_amount,
            commission_amount: ctx.accounts.game_match.commission_amount,
            position: ctx.accounts.game_match.position.clone(),
            player_corner: ctx.accounts.game_match.player_corner.clone(),
        });

        Ok(())
    }
}
