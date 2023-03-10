use crate::errors::ErrorCode;
use crate::ADMIN_PUBKEY;
use anchor_lang::prelude::*;

pub fn is_admin(signer: &Signer) -> Result<()> {
    if signer.key.to_string().as_bytes() != ADMIN_PUBKEY {
        return err!(ErrorCode::PermissionDenied);
    }

    Ok(())
}
