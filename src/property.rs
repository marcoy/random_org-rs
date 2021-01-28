pub trait HasSameVariant {
  fn same_variant(&self, other: &Self) -> bool;
}
