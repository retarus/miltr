#![no_main]

use libfuzzer_sys::fuzz_target;

use bytes::BytesMut;
use miltr_client::fuzzing::fuzz_parse;

fuzz_target!(|data: &[u8]| {
    let mut buffer = BytesMut::from(data);
    let _decoded = fuzz_parse(&mut buffer);
});
