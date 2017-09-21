// This file is part of sfnt2woff-zopfli-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sfnt2woff-zopfli-sys/master/COPYRIGHT. No part of sfnt2woff-zopfli-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sfnt2woff-zopfli-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sfnt2woff-zopfli-sys/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


extern crate core;
extern crate libc;


use ::libc::FILE;


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
