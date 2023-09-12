/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::fmt::{Debug, Formatter};

use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Sign, RsaPublicKey};
use sha2::Sha256;

use crate::error::VResult;

/// authentication information
pub struct BotAuthInfo {
  /// bot_id
  id: String,
  /// bot_secret
  secret: String,
  /// public key
  pub_key: RsaPublicKey,

  secret_hmac: String,
}

impl BotAuthInfo {
  /// create a instance
  pub fn new(id: impl Into<String>, secret: impl Into<String>, pub_key: impl AsRef<str>) -> Self {
    // region secret
    use hmac::{Hmac, Mac};
    let secret = secret.into();

    let mut mac = Hmac::<Sha256>::new_from_slice(pub_key.as_ref().as_bytes())
      .expect("expect success without error (hmac shouldn't generate a length error)");

    mac.update(secret.as_bytes());
    let secret_hmac = mac.finalize().into_bytes();
    // endregion

    let pub_key = RsaPublicKey::from_public_key_pem(pub_key.as_ref())
      .expect("expect correct public key with pem format");

    Self {
      id: id.into(),
      secret,
      pub_key,
      secret_hmac: hex::encode(secret_hmac),
    }
  }

  /// create a instance from environment variable
  /// VILLA_BOT_ID, VILLA_BOT_SECRET
  /// and public key in pem format
  pub fn from_env(pub_key: impl AsRef<str>) -> VResult<Self> {
    let id = std::env::var("VILLA_BOT_ID")?;
    let secret = std::env::var("VILLA_BOT_SECRET")?;

    Ok(Self::new(id, secret, pub_key))
  }

  /// get bot id
  pub fn id(&self) -> &str {
    &self.id
  }

  /// get hmac of bot secret
  pub fn secret_hmac(&self) -> &str {
    &self.secret_hmac
  }

  /// verify signature of callback
  pub fn verify_callback_sign(
    &self,
    body: impl AsRef<str>,
    signature: &[u8],
  ) -> Result<(), rsa::Error> {
    use sha2::{digest::FixedOutput, Digest};

    let content = form_urlencoded::Serializer::new(String::new())
      .append_pair("body", body.as_ref().trim())
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
  }
}

impl Debug for BotAuthInfo {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("BotAuthInfo")
      .field("id", &self.id)
      .field("secret", &"***")
      .field("pub_key", &"***")
      .field("secret_hmac", &"***")
      .finish()
  }
}
