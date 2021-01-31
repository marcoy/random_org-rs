use anyhow::Result;
use valid::{Validate, ValidationResult};
use valid::constraint::{Bound, Length};

use crate::random_org_constraint::EveryElement;
use crate::SeqBound;

fn to_result<C, T>(validation_res: ValidationResult<C, T>) -> Result<T> {
  validation_res
    .map(|v| v.unwrap())
    .map_err(|e| e.into())
}

pub fn generate_integers(n: u16, min: i32, max: i32) -> Result<(u16, i32, i32)> {
  let n_bound = Bound::ClosedRange(1, 1000);
  let min_max_bound = Bound::ClosedRange(-1_000_000_000, 1_000_000_000);

  let validation_res: ValidationResult<(), (u16, i32, i32)> = n.validate("n", &n_bound)
    .and(min.validate("min", &min_max_bound))
    .and(max.validate("max", &min_max_bound))
    .map(|((n, min), max)| (n, min, max))
    .result();

  to_result(validation_res)
}

pub fn generate_strings(n: u16, length: u8) -> Result<(u16, u8)> {
  let n_bound = Bound::ClosedRange(1, 10_000);
  let length_bound = Bound::ClosedRange(1, 32);

  let validation_res = n.validate("n", &n_bound)
    .and(length.validate("length", &length_bound))
    .result();

  to_result(validation_res)
}

pub fn generate_gaussians(n: u16, mean: i32, std_dev: i32, sig_digits: u8) -> Result<(u16, i32, i32, u8)> {
  let n_bound = Bound::ClosedRange(1, 10_000);
  let mean_std_dev_bound = Bound::ClosedRange(-1_000_000, 1_000_000);
  let sig_digits_bound = Bound::ClosedRange(2, 14);

  let validation_res: ValidationResult<(), (u16, i32, i32, u8)> = n.validate("n", &n_bound)
    .and(mean.validate("mean", &mean_std_dev_bound))
    .and(std_dev.validate("std_dev", &mean_std_dev_bound))
    .and(sig_digits.validate("sig_digits", &sig_digits_bound))
    .map(|(((n, mean), std_dev), sig_digits)| (n, mean, std_dev, sig_digits))
    .result();

  to_result(validation_res)
}

pub fn generate_uuids(n: u16) -> Result<u16> {
  let n_bound = Bound::ClosedRange(1, 1000);
  let validation_res = n.validate("n", &n_bound).result();

  to_result(validation_res)
}

pub fn generate_integer_sequences(n: u16, length: SeqBound, min: SeqBound, max: SeqBound) -> Result<(u16, SeqBound, SeqBound, SeqBound)> {
  let n_bound = Bound::ClosedRange(1, 1000);
  let len_bound = Bound::ClosedRange(1, 10_000);
  let min_max_bound = Bound::ClosedRange(-1_000_000_000, 1_000_000_000);

  match (length, min, max) {
    (SeqBound::Uniform(length), SeqBound::Uniform(min), SeqBound::Uniform(max)) => {
      let validation_res: ValidationResult<(), (u16, SeqBound, SeqBound, SeqBound)> = n.validate("n", &n_bound)
        .and(length.validate("length", &len_bound))
        .and(min.validate("min", &min_max_bound))
        .and(max.validate("max", &min_max_bound))
        .map(|(((n, length), min), max)| (n, SeqBound::Uniform(length), SeqBound::Uniform(min), SeqBound::Uniform(max)))
        .result();
      to_result(validation_res)
    }
    (SeqBound::Multiform(length), SeqBound::Multiform(min), SeqBound::Multiform(max)) => {
      let validation_res: ValidationResult<(), (u16, SeqBound, SeqBound, SeqBound)> = n.validate("n", &n_bound)
        .and_then(|n| {
          let exact_len = Length::Exact(n as usize);
          let every = EveryElement(len_bound.clone());
          length.validate("length", &exact_len)
            .and_then(|ls| ls.validate("length", &every))
            .combine(n)
        })
        .and_then(|(n, length)| {
          let exact_len = Length::Exact(n as usize);
          let every = EveryElement(min_max_bound.clone());
          min.validate("min", &exact_len)
            .and_then(|ls| ls.validate("min", &every))
            .combine((n, length))
        })
        .and_then(|((n, length), min)| {
          let exact_len = Length::Exact(n as usize);
          let every = EveryElement(min_max_bound.clone());
          max.validate("max", &exact_len)
            .and_then(|ls| ls.validate("max", &every))
            .combine((n, length, min))
        })
        .map(|((n, length, min), max)| (n, SeqBound::Multiform(length), SeqBound::Multiform(min), SeqBound::Multiform(max))).result();

      to_result(validation_res)
    }
    _ => Err(anyhow::Error::msg("Mismatch parameter variants"))
  }
}
