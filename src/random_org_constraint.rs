use valid::{FieldName, RelatedFields, Validate, Validation, Value, invalid_relation};

use crate::property::HasSameVariant;

pub const INVALID_SAME_VARIANT: &str = "invalid-same-variant";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SameVariant;


impl<T> Validate<SameVariant, RelatedFields> for (T, T)
  where T: HasSameVariant + Into<Value> {
  fn validate(self, fields: impl Into<RelatedFields>, _constraint: &SameVariant) -> Validation<SameVariant, Self> {
    let RelatedFields(name1, name2) = fields.into();

    if self.0.same_variant(&self.1) {
      Validation::success(self)
    } else {
      Validation::failure(vec![invalid_relation(
        INVALID_SAME_VARIANT,
        name1,
        self.0,
        name2,
        self.1,
      )])
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use valid::Validate;

  use crate::SeqBound;
  use crate::random_org_constraint::SameVariant;

  #[test]
  fn test_same_variant_validate() {
    let uniform1 = SeqBound::Uniform(10);
    let uniform2 = SeqBound::Uniform(20);

    let uniform3 = SeqBound::Uniform(30);
    let multiform1 = SeqBound::Multiform(vec![1, 2, 3]);

    let res1 = (uniform1, uniform2).validate(("min", "max"), &SameVariant).result();
    println!("res: {:?}", res1);

    let res2 = (uniform3, multiform1).validate(("min", "max"), &SameVariant).result();
    println!("res: {:?}", res2);
  }
}
