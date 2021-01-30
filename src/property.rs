use std::slice::Iter;

pub trait HasSameVariant {
  fn same_variant(&self, other: &Self) -> bool;
}

pub trait HasIter<T> {
  fn iterator(&self) -> Iter<'_, T>;
}

impl<T> HasIter<T> for &[T] {
  fn iterator(&self) -> Iter<'_, T> {
    self.iter()
  }
}
