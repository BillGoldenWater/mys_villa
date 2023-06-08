/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

//! ### Initializing bot instance
//! you can simply initialize a Bot instance by calling the new method
//! ```no_run
//! use mys_villa::bot::bot_event_handler::BotEventHandler;
//! use mys_villa::bot::bot_info::BotAuthInfo;
//! use mys_villa::bot::bot_permission::BotPermission;
//! use mys_villa::bot::Bot;
//! // the default implementation of request executor
//! use mys_villa::request::request_executor::request_executor_impl::RequestExecutorImpl;
//!
//! # use mys_villa::error::VResult;
//! #
//! # #[derive(Debug)]
//! # struct State;
//! #
//! # #[derive(Debug)]
//! # struct EventHandler;
//! #
//! # impl BotEventHandler<State, RequestExecutorImpl> for EventHandler {}
//! #
//! # fn main() -> VResult<()> {
//! let bot = Bot::new(
//!   BotAuthInfo::from_env()?,
//!   BotPermission::all(),
//!   RequestExecutorImpl::new()?,
//!   State,
//!   EventHandler,
//! );
//! #   Ok(())
//! # }
//! ```
//!
//! ### Make a api request
//! for example if we want to get the villa info of villa by id 123456789
//! ```no_run
//! use mys_villa::api_type::villa::villa_info::VillaInfo;
//!
//! # use mys_villa::error::VResult;
//! # use mys_villa::bot::default::default;
//! #
//! # #[tokio::main]
//! # async fn main() -> VResult<()> {
//! #   let bot = default();
//! // first create a instance of target villa,
//! // store it in variable for future reuse
//! let villa = bot.villa(12345789);
//! // then we can call the get_info method on the instance
//! let villa_info: VillaInfo = villa.get_info().await?;
//! println!("{}",villa_info.name); // name of this villa
//! #   Ok(())
//! # }
//! ```
//!
//! ### Other
//! for more information, you can view the related module/struct documentation and examples

#![warn(missing_docs, missing_debug_implementations)]
#![doc(html_no_source)]

/// api related type
pub mod api_type;
/// bot related logic, includes villa, role, group, room and message related operation
pub mod bot;
/// define errors
pub mod error;
/// request related logic, include [request::request_executor::RequestExecutor]
pub mod request;
/// define response structure
pub mod response;
/// some utility
pub mod utils;
