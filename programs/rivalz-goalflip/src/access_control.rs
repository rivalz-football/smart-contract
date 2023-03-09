use anchor_lang::prelude::*;
use crate::ADMIN_PUBKEY;
use crate::errors::Err;

pub fn is_admin(signer: &Signer) -> Result<()> {
    if signer.key.to_string().as_bytes() != ADMIN_PUBKEY {
        return err!(Err::PermissionDenied);
    }

    Ok(())
}