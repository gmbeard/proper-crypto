use std::env;
mod stdin_source;
extern crate proper_crypto;

use std::io::{self, Write};
use proper_crypto::{
    Transform, 
    transform_source, 
    Rot13, 
};

#[cfg(target_os = "windows")]
use proper_crypto::{
    ToBase64, 
    FromBase64,
    NativeEncrypt,
    NativeDecrypt,
};

use stdin_source::{StdinSource, StdinTransformError};

fn transform_stdin<T>(t: T) 
    where T: Transform,
          T::Item: AsRef<[u8]>,
          StdinTransformError: From<T::Error>
{
    if let Ok(bytes) = transform_source(StdinSource::new(), t) {
        io::stdout().write(bytes.as_ref()).ok();
        io::stdout().flush().ok();
    }
}

#[cfg(target_os = "windows")]
fn main() {
    let (use_native, encrypt) = (
        env::args().any(|a| a == "-n"),
        !env::args().any(|a| a == "-d")
    );

    match (use_native, encrypt) {
        (true, true ) => transform_stdin(ToBase64::new(NativeEncrypt::new())),
        (true, false) => transform_stdin(FromBase64::new(NativeDecrypt::new())),
        (false, _   ) => transform_stdin(Rot13::new())
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    transform_stdin(Rot13::new());
}
