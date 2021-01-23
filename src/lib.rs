use std::ops::Add;

use anyhow::Result;
use serde::Deserialize;
use uuid::Uuid;

use json_rpc::{JsonRpc, RpcCall};

pub mod json_rpc;

pub struct RandomOrg {
  api_key: String,
  base_uri: String,
  json_rpc: JsonRpc,
}

#[derive(Debug, Deserialize)]
pub struct RandomData<T> {
  #[serde(rename = "completionTime")]
  completion_time: String,

  pub data: T,
}

#[derive(Debug, Deserialize)]
struct RandomOrgResponse<T> {
  #[serde(rename = "requestsLeft")]
  requests_left: u32,
  #[serde(rename = "bitsUsed")]
  bits_used: u64,
  #[serde(rename = "bitsLeft")]
  bits_left: u64,
  #[serde(rename = "advisoryDelay")]
  advisory_delay_millis: u64,
  random: RandomData<T>,
}

#[derive(Debug)]
pub enum RandomStringCharSet {
  Number,
  LowerAlphabet,
  UpperAlphabet,
  Custom(String),
}

impl RandomStringCharSet {
  pub fn to_str(&self) -> &str {
    match self {
      RandomStringCharSet::Number => "0123456789",
      RandomStringCharSet::LowerAlphabet => "abcdefghijklmnopqrstuvwxyz",
      RandomStringCharSet::UpperAlphabet => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
      RandomStringCharSet::Custom(s) => s,
    }
  }
}

impl Add for RandomStringCharSet {
  type Output = RandomStringCharSet;

  fn add(self, rhs: RandomStringCharSet) -> Self::Output {
    let mut cset = String::new();
    cset.push_str(self.to_str());
    cset.push_str(rhs.to_str());

    RandomStringCharSet::Custom(cset)
  }
}

impl RandomOrg {
  pub fn new(api_key: String, base_uri: String, json_rpc: JsonRpc) -> RandomOrg {
    RandomOrg {
      api_key,
      base_uri,
      json_rpc,
    }
  }

  pub async fn generate_integers(&self, n: u32, min: i32, max: i32, replacement: bool) -> Result<RandomData<Vec<i32>>> {
    let api_key: &str = self.api_key.as_str();
    let mut params = serde_json::Map::new();
    params.insert("apiKey".into(), api_key.into());
    params.insert("n".into(), n.into());
    params.insert("min".into(), min.into());
    params.insert("max".into(), max.into());
    params.insert("replacement".into(), replacement.into());

    let call = RpcCall::new("generateIntegers".into(), params);

    let resp = self
      .json_rpc
      .execute(&self.base_uri, call, |v| {
        let rand_resp: RandomOrgResponse<Vec<i32>> = serde_json::from_value(v)?;
        Ok(rand_resp)
      })
      .await?;

    Ok(resp.random)
  }


  pub async fn generate_strings(
    &self,
    n: u32,
    length: u16,
    char_set: RandomStringCharSet,
    replacement: bool,
  ) -> Result<RandomData<Vec<String>>> {
    let api_key: &str = self.api_key.as_str();
    let mut params = serde_json::Map::new();
    params.insert("apiKey".into(), api_key.into());
    params.insert("n".into(), n.into());
    params.insert("length".into(), length.into());
    params.insert("characters".into(), char_set.to_str().into());
    params.insert("replacement".into(), replacement.into());

    let call = RpcCall::new("generateStrings".into(), params);

    let resp = self
      .json_rpc
      .execute(&self.base_uri, call, |v| {
        let rand_resp: RandomOrgResponse<Vec<String>> = serde_json::from_value(v)?;
        Ok(rand_resp)
      })
      .await?;

    Ok(resp.random)
  }

  pub async fn generate_gaussians(
    &self,
    n: u32,
    mean: f32,
    std_dev: f32,
    sig_digits: u8,
  ) -> Result<RandomData<Vec<f64>>> {
    let api_key = self.api_key.as_str();
    let mut params = serde_json::Map::new();
    params.insert("apiKey".into(), api_key.into());
    params.insert("n".into(), n.into());
    params.insert("mean".into(), mean.into());
    params.insert("standardDeviation".into(), std_dev.into());
    params.insert("significantDigits".into(), sig_digits.into());

    let call = RpcCall::new("generateGaussians".into(), params);

    let resp = self
      .json_rpc
      .execute(&self.base_uri, call, |v| {
        let rand_resp: RandomOrgResponse<Vec<f64>> = serde_json::from_value(v)?;
        Ok(rand_resp)
      })
      .await?;

    Ok(resp.random)
  }

  pub async fn generate_uuids(&self, n: u32) -> Result<RandomData<Vec<Uuid>>> {
    let api_key = self.api_key.as_str();
    let mut params = serde_json::Map::new();
    params.insert("apiKey".into(), api_key.into());
    params.insert("n".into(), n.into());

    let call = RpcCall::new("generateUUIDs".into(), params);

    let resp = self
      .json_rpc
      .execute(&self.base_uri, call, |v| {
        let rand_resp: RandomOrgResponse<Vec<Uuid>> = serde_json::from_value(v)?;
        Ok(rand_resp)
      })
      .await?;

    Ok(resp.random)
  }
}
