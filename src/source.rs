pub trait Source {
    type Data : ?Sized;
    type Error;

    fn read(&mut self) -> Result<&Self::Data, Self::Error>;
}

pub trait SourceMut : Source {
    fn read_mut(&mut self) -> Result<&mut Self::Data, Self::Error>;
}

impl<'a, T: Source> Source for &'a mut T {
    type Data = T::Data;
    type Error = T::Error;

    fn read(&mut self) -> Result<&Self::Data, Self::Error> {
        T::read(*self)
    }
}

impl<'a, T: SourceMut> SourceMut for &'a mut T {
    fn read_mut(&mut self) -> Result<&mut Self::Data, Self::Error> {
        T::read_mut(*self)
    }
}

impl Source for [u8] {
    type Data = [u8];
    type Error = ();

    fn read(&mut self) -> Result<&Self::Data, Self::Error> {
        Ok(self)
    }
}

impl<'a> Source for &'a [u8] {
    type Data = [u8];
    type Error = ();

    fn read(&mut self) -> Result<&Self::Data, Self::Error> {
        Ok(self)
    }
}

impl SourceMut for [u8] {
    fn read_mut(&mut self) -> Result<&mut Self::Data, Self::Error> {
        Ok(self)
    }
}

impl Source for Vec<u8> {
    type Data = [u8];
    type Error = ();

    fn read(&mut self) -> Result<&Self::Data, Self::Error> {
        Ok(&*self)
    }
}

impl SourceMut for Vec<u8> {
    fn read_mut(&mut self) -> Result<&mut Self::Data, Self::Error> {
        Ok(&mut *self)
    }
}

#[cfg(test)]
mod source_should {
    use super::*;
    
    fn read_bytes_from<S>(mut s: S) -> Result<Vec<u8>, S::Error>
        where S: Source<Data=[u8]>
    {
        Ok(s.read()?.to_vec())
    }

    #[test]
    fn read_from_vec() {
        let mut v = vec![0_u8; 5];
        assert!(read_bytes_from(&mut v).is_ok());
    }
}
