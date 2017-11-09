use transform::Transform;

/// A wrapper around a `T` that implements `Transform`. This type
/// converts the transformed data of its inner type to *Base64*
pub struct ToBase64<T>(T);

/// A wrapper around a `T` that implements `Transform`. This type
/// converts the incoming data from *Base64*, before it is transformed
/// by its inner type.
pub struct FromBase64<T>(T);

#[link(name="crypt32")]
extern "system" {
    fn CryptBinaryToStringA(
        pbBinary: Option<&u8>,
        cbBinary: u32,
        dwFlags: u32,
        pszString: Option<&mut u8>,
        pcchString: Option<&mut u32>) -> bool;

    fn CryptStringToBinaryA(
        pszString: Option<&u8>,
        cchString: u32,
        dwFlags: u32,
        pbBinary: Option<&mut u8>,
        pcbBinary: Option<&mut u32>,
        pdwSkip: Option<&mut u32>,
        pdwFlags: Option<&mut u32>
    ) -> bool;
}

fn to_base64(data: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(data.len() * 2);
    let mut required_len = buffer.capacity() as u32;
    let result = unsafe {
        CryptBinaryToStringA(
            Some(&*data.as_ptr()),
            data.len() as u32,
            0x40000001,
            Some(&mut *buffer.as_mut_ptr()),
            Some(&mut required_len)
        )
    };

    if !result {
        panic!("CryptBinaryToStringA didn't return true");
    }

    unsafe { buffer.set_len(required_len as usize) };
    buffer
}

fn from_base64(data: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(data.len());
    let mut output_len = buffer.capacity() as u32;

    let result = unsafe {
        CryptStringToBinaryA(
            Some(&*data.as_ptr()),
            data.len() as u32,
            0x00000001,
            Some(&mut *buffer.as_mut_ptr()),
            Some(&mut output_len),
            None,
            None
        )
    };

    if !result {
        panic!("CryptStringToBinaryA didn't return true");
    }

    unsafe { buffer.set_len(output_len as usize) };

    buffer
}

impl<T> ToBase64<T> {
    pub fn new(inner: T) -> ToBase64<T> {
        ToBase64(inner)
    }
}

impl<T> FromBase64<T> {
    pub fn new(inner: T) -> FromBase64<T> {
        FromBase64(inner)
    }
}

impl<T> Transform for ToBase64<T> 
    where T: Transform,
          T::Item: IntoIterator<Item=u8>
{
    type Item = Vec<u8>;
    type Error = T::Error;

    fn transform(&mut self, data: &[u8]) -> Result<Self::Item, Self::Error> {
        let inner = self.0.transform(data)?
                          .into_iter()
                          .collect::<Vec<_>>();
        Ok(to_base64(&*inner))
    }
}

impl<T> Transform for FromBase64<T>
    where T: Transform
{
    type Item = T::Item;
    type Error = T::Error;

    fn transform(&mut self, data: &[u8]) -> Result<Self::Item, Self::Error> {
        self.0.transform(&*from_base64(data))
    }
}

#[cfg(test)]
mod to_base64_should {
    use super::*;

    #[test]
    fn convert_to_correct_base64() {
        let result = to_base64("Hello, World!".as_bytes());
        assert_eq!(b"SGVsbG8sIFdvcmxkIQ==", &*result);
    }

    #[test]
    fn convert_to_correct_string() {
        let result = from_base64("SGVsbG8sIFdvcmxkIQ==".as_bytes());
        assert_eq!(b"Hello, World!", &*result);
    }
}
