use transform::{Transform, TransformInPlace};

fn rot13_byte(b: u8, start_from: u8) -> u8 {
    start_from + ((b - start_from) + 13) % 26
}

fn transform(s: &[u8]) -> Vec<u8> {
    s.iter()
     .map(|c| match *c {
        b'A'...b'Z' => rot13_byte(*c, b'A'),
        b'a'...b'z' => rot13_byte(*c, b'a'),
        _ => *c
     })
     .collect()
}

/// A *ROT13* transformation
pub struct Rot13;

impl Transform for Rot13 {
    type Item = Vec<u8>;
    type Error = ();

    fn transform(&mut self, data: &[u8]) -> Result<Self::Item, Self::Error> {
        Ok(transform(data))
    }
}

impl Rot13 {
    pub fn new() -> Rot13 {
        Rot13
    }
}

///// A *ROT13* transformation that operates on its
///// data in-place, without any extra allocations.
//pub struct InPlaceRot13;
//
//impl InPlaceRot13 {
//    pub fn new() -> InPlaceRot13 {
//        InPlaceRot13
//    }
//}

impl TransformInPlace for Rot13 {
    type Error = ();

    fn transform_in_place(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        for b in data.iter_mut() {
            *b = match *b {
                b'A'...b'Z' => rot13_byte(*b, b'A'),
                b'a'...b'z' => rot13_byte(*b, b'a'),
                _ => *b ,
            };
        }

        Ok(())
    }
}


#[cfg(test)]
mod rot13_crypto_should {
    use super::*;

    #[test]
    fn encrypt_to_rot13_value() {
        let result = transform(b"Hello, World!");
        assert_eq!(b"Uryyb, Jbeyq!", &*result);
    }

    #[test]
    fn decrypt_to_same_value() {
        assert_eq!(b"Hello, World!", &*transform(&transform(b"Hello, World!")));
    }
}
