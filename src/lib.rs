// Copyright (C) 2017 Stephane Raux. Distributed under the MIT license.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate clue_sys as clue;

use clue::bindings::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
