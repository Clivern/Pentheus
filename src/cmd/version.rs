// Copyright 2025 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

pub fn get_version() -> &'static str {
    "v0.1.0"
}

#[test]
fn test_get_version() {
    assert_eq!(get_version(), "v0.1.0");
}
