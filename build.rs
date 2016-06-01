/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate cmake;
extern crate pkg_config;

use std::env;

fn main() {
    if pkg_config::Config::new().atleast_version("2.1.0").find("expat").is_ok() {
        return
    }

    //let dst = cmake::Config::new("expat").build_target("ALL_BUILD").build();
    let mut dst = cmake::build("expat");
    dst.push("lib");
    let target = env::var("TARGET").unwrap();
    if target.contains("eabi") || target.contains("windows") {
        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-link-lib=static=expat");
    } else {
        println!("cargo:rustc-link-lib=expat");
    }
}
