pub trait Transform {
    type Item;
    type Error;

    fn transform(&mut self, data: &[u8]) -> Result<Self::Item, Self::Error>;
}

pub trait TransformInPlace {
    type Error;

    fn transform_in_place(&mut self, data: &mut [u8]) -> Result<(), Self::Error>;
}

impl<'a, T> Transform for &'a mut T
    where T: Transform
{
    type Item = T::Item;
    type Error = T::Error;

    fn transform(&mut self, data: &[u8]) -> Result<Self::Item, Self::Error> {
        T::transform(*self, data)
    }
}

impl<'a, T> TransformInPlace for &'a mut T
    where T: TransformInPlace
{
    type Error = T::Error;

    fn transform_in_place(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        T::transform_in_place(*self, data)
    }
}

