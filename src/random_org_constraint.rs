use serde::__private::fmt::Debug;
use valid::{FieldName, invalid_relation, RelatedFields, Validate, Validation, Value};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EveryElement<C>(pub C);

impl<T, C> Validate<EveryElement<C>, FieldName> for Vec<T>
  where
    T: Validate<C, FieldName> + Copy + Into<Value>, {
  fn validate(self, context: impl Into<FieldName>, constraint: &EveryElement<C>) -> Validation<EveryElement<C>, Self> {
    let name = context.into().0;

    for (i, &v) in self.iter().enumerate() {
      let validate_result = v.validate(format!("{}[{}]", &name, &i), &constraint.0).result();

      if validate_result.is_err() {
        let err = validate_result.err().unwrap();
        return Validation::failure(err.violations);
      }
    }

    Validation::success(self)
  }
}

#[cfg(test)]
mod tests {
  use valid::constraint;
  use valid::Validate;

  use crate::random_org_constraint::SameVariant;
  use crate::SeqBound;

  use super::*;

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

  #[test]
  fn test_every_element_validate() {
    let bound = constraint::Bound::ClosedRange(1, 10);
    let every_element = EveryElement(bound.clone());

    let vs1 = vec![1, 4, 10];
    let out1 = vs1.validate("vs1", &every_element).result();
    assert!(out1.is_ok());

    let vs2 = vec![1, 11];
    let every2 = EveryElement(bound);
    let out2 = vs2.validate("vs2", &every2).result();
    assert!(out2.is_err());
  }
}
