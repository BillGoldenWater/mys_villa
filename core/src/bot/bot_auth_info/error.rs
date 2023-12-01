/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("failed to parse public key: {0}")]
  FailedToParsePubKey(#[from] rsa::pkcs8::spki::Error),

  #[error("failed to load auth info from env: {0}")]
  FailedToLoadFromEnv(#[from] std::env::VarError),
  #[error("failed to read public key from file: {0}")]
  FailedToReadPubKey(#[from] std::io::Error),

  #[error("failed to verify sign: {0}")]
  FailedToVerifySign(#[from] rsa::Error),
}
