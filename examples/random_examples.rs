extern crate random_org;

use std::env;

use anyhow::Result;

use random_org::json_rpc::JsonRpc;
use random_org::RandomOrg;
use uuid::Uuid;

async fn rand_ints(random: &RandomOrg) -> Result<Vec<i32>> {
  let rand_data = random.generate_integers(10, 0, 1000, true).await?;
  Ok(rand_data.data)
}

async fn rand_uuids(random: &RandomOrg) -> Result<Vec<Uuid>> {
  let rand_data = random.generate_uuids(5).await?;
  Ok(rand_data.data)
}

#[tokio::main]
async fn main() -> Result<()> {
  let api_key = env::var("RANDOM_ORG_API")?;
  let base_uri = "https://api.random.org/json-rpc/2/invoke".to_string();

  let client = reqwest::Client::new();
  let json_rpc = JsonRpc::new(client.clone());

  let random = RandomOrg::new(api_key, base_uri, json_rpc);

  let ints = rand_ints(&random).await?;
  println!("Ints: {:?}", ints);

  let uuids = rand_uuids(&random).await?;
  println!("UUIDs: {:?}", uuids);

  Ok(())
}
