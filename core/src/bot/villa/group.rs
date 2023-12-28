/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::sync::Arc;

use serde_json::Value;
use tracing::instrument;

use crate::api::villa_bot_api::villa_api::group_api::delete_group::DeleteGroup;
use crate::api::villa_bot_api::villa_api::group_api::edit_group::EditGroupRequest;
use crate::bot::bot_permission::BotPermission;
use crate::bot::villa::Villa;
use crate::bot::Bot;
use crate::error::VResult;
use crate::http::request::Request;
use crate::utils::fp_utils::FpUtils;

#[derive(Debug, Clone)]
pub struct Group {
  villa: Arc<Villa>,

  id: u64,
}

impl Group {
  #[instrument(level = "debug", skip(villa))]
  pub fn new(villa: Arc<Villa>, id: u64) -> Self {
    Self { villa, id }
  }

  pub fn id(&self) -> u64 {
    self.id
  }
}

/// api
impl Group {
  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn set_name(&self, name: String) -> VResult<()> {
    BotPermission::ManageRoomAndGroup.check(self)?;

    self
      .villa
      .execute_bot_req_with_villa::<Value>(
        Request::post("/vila/api/bot/platform/editGroup")
          .with_json(EditGroupRequest::new(self.id, name))?,
      )
      .await
      .unit_result()
  }

  #[instrument(level = "debug", skip(self), fields(id = self.id))]
  pub async fn delete(self) -> VResult<()> {
    BotPermission::ManageRoomAndGroup.check(&self)?;

    self
      .villa
      .execute_bot_req_with_villa::<Value>(
        Request::post("/vila/api/bot/platform/deleteGroup").with_json(DeleteGroup::new(self.id))?,
      )
      .await
      .unit_result()
  }
}

impl AsRef<Bot> for Group {
  fn as_ref(&self) -> &Bot {
    &self.villa.bot
  }
}
