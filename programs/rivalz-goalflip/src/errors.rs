use anchor_lang::prelude::*;

#[error_code]
pub enum Err {
    #[msg("Permission denied")]
    PermissionDenied
}