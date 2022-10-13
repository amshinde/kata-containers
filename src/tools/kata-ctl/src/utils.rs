// Copyright (c) 2022 Intel Corporation
// //
// // SPDX-License-Identifier: Apache-2.0
// //
//
// // Contains checks that are not architecture-specific

use privdrop::PrivDrop;
use anyhow::{anyhow, Result};

pub const NON_PRIV_USER: &str = "nobody";

fn drop_privs()->Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        privdrop::PrivDrop::default()
            .user(NON_PRIV_USER).unwrap()
            .apply()
            .map_err(|e| anyhow!("Failed to drop privileges to user {}: {}", NON_PRIV_USER, e))?;
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_privs() {
        let res = drop_privs();
        assert!(res.is_ok(), res);
    }
}
