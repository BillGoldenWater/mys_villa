/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::fmt::{Debug, Display};

pub trait ResultExt
where
  Self: Sized,
{
  type Ok;
  type Err;
  fn err_only(self) -> Result<(), Self::Err>;
}

impl<T, E> ResultExt for Result<T, E> {
  type Ok = T;
  type Err = E;

  fn err_only(self) -> Result<(), <Self as ResultExt>::Err> {
    self.map(|_| ())
  }
}

pub trait FpUtils
where
  Self: Sized,
{
  fn then<R>(self, f: impl FnOnce(Self) -> R) -> R {
    f(self)
  }

  fn also(self, f: impl FnOnce(&Self)) -> Self {
    f(&self);
    self
  }

  fn also_mut(mut self, f: impl FnOnce(&mut Self)) -> Self {
    f(&mut self);
    self
  }

  fn some(self) -> Option<Self> {
    Some(self)
  }

  fn ok<E>(self) -> Result<Self, E>
  where
    Self: Sized,
  {
    Ok(self)
  }

  fn err<O>(self) -> Result<O, Self>
  where
    Self: Sized,
  {
    Err(self)
  }

  fn unit(self) {}

  fn unit_result<T, E>(self) -> Result<(), E>
  where
    Self: ResultExt<Ok = T, Err = E>,
  {
    self.err_only()
  }

  fn println(&self)
  where
    Self: Display,
  {
    println!("{self}");
  }

  fn println_dbg(&self)
  where
    Self: Debug,
  {
    println!("{self:#?}");
  }

  fn println_ret(self) -> Self
  where
    Self: Display,
  {
    println!("{self}");
    self
  }

  fn println_dbg_ret(self) -> Self
  where
    Self: Debug,
  {
    println!("{self:#?}");
    self
  }
}

impl<T> FpUtils for T {}
