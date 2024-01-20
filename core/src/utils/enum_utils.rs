/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[macro_export]
macro_rules! impl_enum_str_convert {
  (
    $target:ident;
    $(@preprocess_fn($preprocess_value:ident) => $preprocess:block;)?
    $(
      $name:literal => $variant:ident
    ),+;
    _ => $default:ident $(,)?
  ) => {
    impl From<::std::string::String> for $target {
      fn from(value: String) -> Self {
        $(
          let value = {
            let $preprocess_value = value;
            $preprocess
          };
        )?

        match value.as_str() {
          $($name => Self::$variant,)+
          _ => Self::$default(value)
        }
      }
    }

    impl From<$target> for ::std::string::String {
      fn from(value: $target) -> Self {
        match value {
          $($target::$variant => $name.to_string(),)+
          $target::$default(value) => value,
        }
      }
    }
  };
}

#[macro_export]
macro_rules! impl_enum_convert_exact {
  ($enum_0:ident <=> $enum_1:ident; $($variant:ident),+ $(,)?) => {
    impl From<$enum_0> for $enum_1 {
      fn from(value: $enum_0) -> Self {
        match value {
          $($enum_0::$variant => Self::$variant,)+
        }
      }
    }

    impl From<$enum_1> for $enum_0 {
      fn from(value: $enum_1) -> Self {
        match value {
          $($enum_1::$variant => Self::$variant,)+
        }
      }
    }
  };
}
