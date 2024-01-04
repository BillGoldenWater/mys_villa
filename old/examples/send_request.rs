/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use mys_villa::{
  bot::{
    bot_event_handler::BotEventHandler, bot_info::BotAuthInfo, bot_permission::BotPermission, Bot,
  },
  error::VResult,
  request::request_executor::request_executor_impl::RequestExecutorImpl,
};

#[derive(Debug)]
struct State;

#[derive(Debug)]
struct EventHandler {}

impl BotEventHandler<State, RequestExecutorImpl> for EventHandler {}

#[tokio::main]
pub async fn main() -> VResult<()> {
  const BOT_ID: &str = "bot_id";
  const BOT_SECRET: &str = "secret";

  const VILLA_ID: u64 = 123456789;

  // init logger
  let _ = env_logger::try_init();

  let req_executor = RequestExecutorImpl::new().unwrap();
  let bot = Bot::new(
    BotAuthInfo::new(BOT_ID, BOT_SECRET),
    vec![BotPermission::SendMessage, BotPermission::ViewRoomAndGroup],
    req_executor,
    State,
    EventHandler {},
  );

  let mys_villa = bot.villa(VILLA_ID);

  let groups = mys_villa.get_all_room_group_info().await?;

  let room_info = groups
    .iter()
    .flat_map(|it| &it.room_list)
    .next()
    .expect("expect has at least one room");

  let room = mys_villa.room(room_info.id);

  room
    .send_message(room.message_builder().mhy_text(|m| m.text("Hello world!")))
    .await?;

  Ok(())
}
