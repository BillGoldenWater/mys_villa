/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

/// for measure string length in utf-16
pub fn len_utf16(string: impl AsRef<str>) -> usize {
  string.as_ref().encode_utf16().count()
}
