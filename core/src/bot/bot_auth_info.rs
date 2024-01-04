/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::fmt::{Debug, Formatter};

use rsa::{
  pkcs8::{DecodePublicKey, EncodePublicKey, LineEnding},
  Pkcs1v15Sign, RsaPublicKey,
};
use sha2::Sha256;
use tracing::instrument;

use crate::bot::bot_auth_info::error::Error;

pub mod error;

/// authentication information
pub struct BotAuthInfo {
  /// bot_id
  id: Box<str>,
  /// bot_secret
  secret: Box<str>,
  /// public key
  pub_key: RsaPublicKey,

  secret_hmac: Box<str>,
}

impl BotAuthInfo {
  /// bot id
  pub fn id(&self) -> &str {
    &self.id
  }

  /// hmac of bot secret with raw pub_key as key
  pub fn secret_hmac(&self) -> &str {
    &self.secret_hmac
  }

  /// verify signature of callback
  #[instrument(level = "debug")]
  pub fn verify_callback_sign(&self, body: &str, signature: &[u8]) -> Result<(), Error> {
    use sha2::{digest::FixedOutput, Digest};

    let content = form_urlencoded::Serializer::new(String::new())
      .append_pair("body", body.trim())
      .append_pair("secret", &self.secret)
      .finish()
      .replace('*', "%2A")
      .replace("%7E", "~");

    let mut sha256 = Sha256::default();
    sha256.update(content.as_bytes());
    let hash = sha256.finalize_fixed();

    self
      .pub_key
      .verify(Pkcs1v15Sign::new::<Sha256>(), &hash, signature)
      .map_err(Into::into)
  }
}

/// creation of [BotAuthInfo]
impl BotAuthInfo {
  #[instrument(level = "debug", skip(secret, pub_key))]
  pub fn new(id: String, secret: String, pub_key: &str) -> Result<Self, Error> {
    // region public key
    let pub_key = RsaPublicKey::from_public_key_pem(pub_key)?;
    let pub_key_pem_formatted = pub_key
      .to_public_key_pem(LineEnding::LF)
      .expect("expect success without error (since it's load from pem format)");
    // endregion

    // region secret
    use hmac::{Hmac, Mac};

    let mut mac = Hmac::<Sha256>::new_from_slice(pub_key_pem_formatted.as_bytes())
      .expect("expect success without error (hmac shouldn't generate a length error)");

    mac.update(secret.as_bytes());
    let secret_hmac = mac.finalize().into_bytes();
    // endregion

    Ok(Self {
      id: id.into(),
      secret: secret.into(),
      pub_key,
      secret_hmac: hex::encode(secret_hmac).into(),
    })
  }

  /// create a instance from environment variable
  /// VILLA_BOT_ID, VILLA_BOT_SECRET, VILLA_BOT_PUB_KEY
  ///
  /// VILLA_BOT_PUB_KEY: public key file path (pem format)
  #[instrument(level = "debug")]
  pub fn try_from_env() -> Result<Self, Error> {
    use std::env::var;
    let id = var("VILLA_BOT_ID")?;
    let secret = var("VILLA_BOT_SECRET")?;
    let pub_key_path = var("VILLA_BOT_PUB_KEY")?;

    let pub_key = std::fs::read_to_string(pub_key_path)?;

    Self::new(id, secret, &pub_key)
  }
}

impl Debug for BotAuthInfo {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("BotAuthInfo").field("id", &self.id).finish()
  }
}
