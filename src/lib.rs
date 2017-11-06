fn rot13_char(c: char, start_from: char) -> char {
    ::std::char::from_u32(start_from as u32 + ((c as u32 - start_from as u32) + 13) % 26)
        .unwrap()
}

/// Encrypts `s` using *ROT13*
///
/// # Examples
///
/// ```
/// extern crate proper_crypto;
/// let result = proper_crypto::encrypt("Hello, World!");
/// assert_eq!("Uryyb, Jbeyq!", result);
/// ```
pub fn encrypt(s: &str) -> String {
    s.chars()
     .map(|c|match c {
        'A'...'Z' => rot13_char(c, 'A'),
        'a'...'z' => rot13_char(c, 'a'),
        _ => c
     })
     .collect()
}

/// Decrypts *ROT13* encrypted value in `s`
///
/// # Examples
///
/// ```
/// extern crate proper_crypto;
/// let result = proper_crypto::decrypt("Uryyb, Jbeyq!");
/// assert_eq!("Hello, World!", result);
/// ```
pub fn decrypt(s: &str) -> String {
    encrypt(s)
}

#[cfg(test)]
mod rot13_crypto_should {
    use super::*;

    #[test]
    fn encrypt_to_rot13_value() {
        let result = encrypt("Hello, World!");
        assert_eq!("Uryyb, Jbeyq!", &result);
    }

    #[test]
    fn decrypt_to_same_value() {
        assert_eq!("Hello, World!", &decrypt(&encrypt("Hello, World!")));
    }
}
