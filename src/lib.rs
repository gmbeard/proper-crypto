mod rot13;
mod transform;
mod source;
pub mod native;

pub use transform::{Transform, TransformInPlace};
pub use rot13::Rot13;
#[cfg(target_os = "windows")]
pub use native::{NativeEncrypt, NativeDecrypt};
pub use source::{Source, SourceMut};

/// Transforms `data` using `T`. You would typically require this function
/// if `T` must allocate space to provide the result (E.g. platform native
/// encryption).
///
/// # Examples
/// ```
/// if cfg!(windows) {
///     use proper_crypto::{transform, NativeEncrypt};
///     assert!(transform("Uryyb, Jbeyq!".as_bytes(), NativeEncrypt::new()).is_ok());
/// }
/// else {
///     use proper_crypto::{transform, Rot13};
///     let result = transform("Uryyb, Jbeyq!".as_bytes(), Rot13::new());
///
///     assert_eq!(Ok("Hello, World!".as_bytes()), result.as_ref().map(|b| &**b));
/// }
/// ```
pub fn transform<D, T>(data: D, mut t: T) -> Result<T::Item, T::Error>
    where D: AsRef<[u8]>,
          T: Transform
{
    t.transform(data.as_ref())
}

/// Transforms `data` using `T`. Use this function if the transformation 
/// can operate on the contents of `data` directly, without allocating any 
/// more space (E.g. *ROT13*).
///
/// # Examples
/// ```
/// use proper_crypto::{transform_in_place, Rot13};
///
/// let mut v = "Hello, World!".as_bytes()
///                            .to_vec();
/// assert!(transform_in_place(&mut v, Rot13::new()).is_ok());
///
/// assert_eq!("Uryyb, Jbeyq!".as_bytes(), &*v);
/// ```
pub fn transform_in_place<T>(data: &mut [u8],
                             mut t: T) -> Result<(), T::Error>
    where T: TransformInPlace
{
    t.transform_in_place(data)
}
