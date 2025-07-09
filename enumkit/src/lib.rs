#[cfg(test)]
mod test;

pub use enumkit_derive::{EnumValues, EnumMapping};

pub trait EnumValues {
    type Enum;
    fn values() -> impl Iterator<Item=Self::Enum>;
    fn len() -> usize;
}

pub trait EnumMapping<T> {
    type Enum;
    type EnumIter<'a>: Iterator<Item=(Self::Enum, &'a T)>
    where
        T: 'a,
        Self: 'a;
    fn get(&self, var: Self::Enum) -> &T;
    fn get_mut(&mut self, var: Self::Enum) -> &mut T;
    fn put(&mut self, var: Self::Enum, val: T);
    fn new<F: FnMut(Self::Enum) -> T>(f: F) -> Self;
    fn iter(&self) -> Self::EnumIter<'_>;
}