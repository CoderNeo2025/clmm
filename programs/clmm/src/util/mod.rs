pub mod account_load;
use std::{cell::{Ref, RefMut}, mem};

pub use account_load::*;
use anchor_lang::{error::{Error, ErrorCode}, prelude::AccountInfo, Owner, ZeroCopy};

/// Avoid &'a AccountInfo<'a> in AccountLoader
pub fn account_map_mut<'info, T: ZeroCopy + Owner, U>(
    acc_info: &AccountInfo<'info>,
    f: impl FnOnce(&mut T) -> U
) -> anchor_lang::prelude::Result<U> {
    if acc_info.owner != &T::owner() {
        return Err(Error::from(ErrorCode::AccountOwnedByWrongProgram)
            .with_pubkeys((*acc_info.owner, T::owner())));
    }

    let data = acc_info.try_borrow_mut_data()?;
    let disc = T::DISCRIMINATOR;
    if data.len() < disc.len() {
        return Err(ErrorCode::AccountDiscriminatorNotFound.into());
    }

    let given_disc = &data[..disc.len()];
    if given_disc != disc {
        return Err(ErrorCode::AccountDiscriminatorMismatch.into());
    }
    let mut value = RefMut::map(data, |data: &mut &mut [u8]| {
        bytemuck::from_bytes_mut::<T>(
            &mut data[disc.len()..mem::size_of::<T>() + disc.len()],
        )
    });
    Ok(f(&mut value))
}

/// Avoid &'a AccountInfo<'a> in AccountLoader
pub fn account_map<'info, T: ZeroCopy + Owner, U>(
    acc_info: &AccountInfo<'info>,
    f: impl FnOnce(&T) -> U
) -> anchor_lang::prelude::Result<U> {
    if acc_info.owner != &T::owner() {
        return Err(Error::from(ErrorCode::AccountOwnedByWrongProgram)
            .with_pubkeys((*acc_info.owner, T::owner())));
    }

    let data = acc_info.try_borrow_data()?;
    let disc = T::DISCRIMINATOR;
    if data.len() < disc.len() {
        return Err(ErrorCode::AccountDiscriminatorNotFound.into());
    }

    let given_disc = &data[..disc.len()];
    if given_disc != disc {
        return Err(ErrorCode::AccountDiscriminatorMismatch.into());
    }
    let value = Ref::map(data, |data: &&mut[u8]| {
        bytemuck::from_bytes::<T>(
            &data[disc.len()..mem::size_of::<T>() + disc.len()],
        )
    });
    Ok(f(&value))
}