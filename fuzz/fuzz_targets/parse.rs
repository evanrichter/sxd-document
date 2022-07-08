#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    if let Ok(package) = sxd_document::parser::parse(data) {
        let doc = package.as_document();
        let _ = doc.root();
    }
});
