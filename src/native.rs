use std::marker;
use transform::Transform;

/// An error type indicating a platform native encryption /
/// decryption transformation failure.
#[derive(Debug)]
pub struct EncryptionError;

#[repr(C)]
struct DataBlob {
    data_size: u32,
    data_ptr: *const u8,
}

#[repr(C)]
struct PromptStruct {
    struct_size: u32,
    flags: u32,
    window: usize,
    prompt: *const u8,
}

#[link(name = "crypt32")]
extern "system" {
    fn CryptProtectData(data: Option<&DataBlob>,
                        descr: Option<&u8>,
                        entrophy: Option<&DataBlob>,
                        resv: usize,
                        prompt: Option<&PromptStruct>,
                        flags: u32,
                        output: Option<&mut DataBlob>) -> bool;

    fn CryptUnprotectData(data: Option<&DataBlob>,
                          descr: Option<&u8>,
                          entrophy: Option<&DataBlob>,
                          resv: usize,
                          prompt: Option<&PromptStruct>,
                          flags: u32,
                          output: Option<&mut DataBlob>) -> bool;
}

#[link(name = "kernel32")]
extern "system" {
    #[link(name = "kernel32")]
    fn LocalFree(mem: usize) -> usize;
}

struct Decrypt;
struct Encrypt;

struct CryptOp<T> { _marker: marker::PhantomData<T> }

trait Crypt {
    fn crypt(input: &DataBlob, output: &mut DataBlob) -> bool;
}

impl Crypt for CryptOp<Encrypt> {
    fn crypt(input: &DataBlob, output: &mut DataBlob) -> bool {
        unsafe {
            CryptProtectData(Some(input),
                             None,
                             None,
                             0,
                             None,
                             0,
                             Some(output))
        }
    }
}

impl Crypt for CryptOp<Decrypt> {
    fn crypt(input: &DataBlob, output: &mut DataBlob) -> bool {
        unsafe {
            CryptUnprotectData(Some(input),
                           None,
                           None,
                           0,
                           None,
                           0,
                           Some(output))
        }
    }
}

fn crypt_with<O>(data: &[u8]) -> Result<Vec<u8>, EncryptionError>
    where O: Crypt
{
    use std::mem;
    use std::slice;

    let blob_in = DataBlob {
        data_size: data.len() as u32,
        data_ptr: data.as_ptr(),
    };

    struct BlobScope(*const u8);

    impl Drop for BlobScope {
        fn drop(&mut self) {
            assert_eq!(0, unsafe{ LocalFree(self.0 as usize) });
        }
    }

    let mut blob_out = unsafe { mem::uninitialized::<DataBlob>() };

    if !O::crypt(&blob_in, &mut blob_out) {
        return Err(EncryptionError)
    }

    let _s = BlobScope(blob_out.data_ptr);

    let v = unsafe {
        let s = slice::from_raw_parts(blob_out.data_ptr, blob_out.data_size as usize);
        let v = s.to_vec();
        v
    };

    Ok(v)

}

fn decrypt(data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    crypt_with::<CryptOp<Decrypt>>(data)
}

fn encrypt(data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    crypt_with::<CryptOp<Encrypt>>(data)
}

/// Platform native encryption
pub struct NativeEncrypt;

impl NativeEncrypt {
    pub fn new() -> NativeEncrypt {
        NativeEncrypt
    }
}

impl Transform for NativeEncrypt {
    type Item = Vec<u8>;
    type Error = EncryptionError;

    fn transform(&mut self, data: &[u8]) -> Result<Self::Item, Self::Error> {
        encrypt(data)
    }
}

/// Platform native decryption
pub struct NativeDecrypt;

impl NativeDecrypt {
    pub fn new() -> NativeDecrypt {
        NativeDecrypt
    }
}

impl Transform for NativeDecrypt {
    type Item = Vec<u8>;
    type Error = EncryptionError;

    fn transform(&mut self, data: &[u8]) -> Result<Self::Item, Self::Error> {
        decrypt(data)
    }
}

#[cfg(test)]
mod encryption_should {
    use super::*;

    #[test]
    fn encrypt_without_error() {
        let result = encrypt(b"Hello, World!");
        if let Ok(d) = result {
            println!("{:?}", d);
        }
        else {
            panic!();
        }
    }

    #[test]
    fn decrypt_to_correct_string() {
        let encrypted = encrypt(b"Hello, World!").unwrap();
        assert_eq!(b"Hello, World!", &*decrypt(&encrypted).unwrap());
    }
}

