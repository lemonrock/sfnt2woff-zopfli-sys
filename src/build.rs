// This file is part of sfnt2woff-zopfli-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sfnt2woff-zopfli-sys/master/COPYRIGHT. No part of sfnt2woff-zopfli-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sfnt2woff-zopfli-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sfnt2woff-zopfli-sys/master/COPYRIGHT.


extern crate bindgen;
extern crate gcc;


use ::bindgen::Builder;
use ::gcc::Build;
use ::std::env;
use ::std::path::PathBuf;


fn main()
{
	Build::new()
		.shared_flag(false)
		.static_flag(true)
		.warnings(false)
        .file("lib/sfnt2woff-zopfli/blocksplitter.c")
        .file("lib/sfnt2woff-zopfli/cache.c")
        .file("lib/sfnt2woff-zopfli/deflate.c")
        .file("lib/sfnt2woff-zopfli/gzip_container.c")
        .file("lib/sfnt2woff-zopfli/hash.c")
        .file("lib/sfnt2woff-zopfli/katajainen.c")
        .file("lib/sfnt2woff-zopfli/lz77.c")
        .file("lib/sfnt2woff-zopfli/squeeze.c")
        .file("lib/sfnt2woff-zopfli/tree.c")
        .file("lib/sfnt2woff-zopfli/util.c")
        .file("lib/sfnt2woff-zopfli/zlib_container.c")
        .file("lib/sfnt2woff-zopfli/zopfli_lib.c")
        .file("lib/sfnt2woff-zopfli/woff.c")
        .compile("sfnt2woff-zopfli");
	
	let bindings = Builder::default()
		.header("lib/sfnt2woff-zopfli/woff.h")
		.generate_comments(false)
		.whitelist_recursively(false)
		.whitelisted_type("WOFF.*")
		.whitelisted_function("woff.*")
		.whitelisted_var("WOFF.*")
		.whitelisted_var("eWOFF.*")
		.bitfield_enum("eWOFF.*")
		.layout_tests(false)
		.derive_debug(true)
		.impl_debug(true)
		.derive_default(true)
		.derive_hash(true)
		.derive_partialeq(true)
		.derive_eq(true)
		.use_core()
		.ctypes_prefix("::libc")
		.prepend_enum_name(false)
		.rustfmt_bindings(true)
		.generate()
		.expect("Unable to generate bindings for woff.h");
	
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings for woff.h");
}
