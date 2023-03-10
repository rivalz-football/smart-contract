use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Permission denied")]
    PermissionDenied,

    #[msg("Game is already completed")]
    GameAlreadyCompleted,

    #[msg("Invalid Position")]
    InvalidPosition,

    #[msg("Invalid Corner")]
    InvalidCorner,
}
