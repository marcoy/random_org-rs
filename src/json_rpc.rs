use anyhow::Result;
use rand::{self, Rng};
use serde::{Deserialize, ser::SerializeStruct, Serialize, Serializer};
use serde_json::{Map, Value};

pub struct JsonRpc {
  client: reqwest::Client,
}

impl JsonRpc {
  pub fn new(client: reqwest::Client) -> JsonRpc {
    JsonRpc { client }
  }

  pub async fn execute<F, T>(&self, api_url: &String, call: RpcCall, f: F) -> Result<T>
    where
      F: Fn(serde_json::Value) -> Result<T>,
  {
    let resp_json: serde_json::Value = self
      .client
      .post(api_url)
      .header("Content-Type", "application/json")
      .json(&RpcPayload::from(call))
      .send()
      .await?
      .json()
      .await?;

    let rpc_resp = serde_json::from_value::<RpcResponse>(resp_json)?;

    log::trace!("RpcResponse: {:#?}", rpc_resp);

    f(rpc_resp.result)
  }
}

#[derive(Debug)]
pub struct RpcCall {
  name: String,
  params: Map<String, Value>,
}

impl RpcCall {
  pub fn new(name: String, params: Map<String, Value>) -> RpcCall {
    RpcCall { name, params }
  }
}

#[derive(Debug)]
struct RpcPayload {
  method: String,
  params: Map<String, Value>,
  id: u32,
}

impl RpcPayload {
  fn new(method: String, params: Map<String, Value>, id: u32) -> RpcPayload {
    RpcPayload { method, params, id }
  }
}

impl From<RpcCall> for RpcPayload {
  fn from(rpc_call: RpcCall) -> RpcPayload {
    let id = rand::thread_rng().gen();
    RpcPayload::new(rpc_call.name, rpc_call.params, id)
  }
}

impl Serialize for RpcPayload {
  fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
      S: Serializer,
  {
    let mut state = serializer.serialize_struct("RpcPayload", 4)?;
    state.serialize_field("jsonrpc", "2.0")?;
    state.serialize_field("method", &self.method)?;
    state.serialize_field("params", &self.params)?;
    state.serialize_field("id", &self.id)?;
    state.end()
  }
}

#[derive(Debug, Deserialize)]
struct RpcResponse {
  id: u32,
  result: serde_json::Value,
}

#[cfg(test)]
mod tests {
  use rand::Rng;
  use serde_json::{Map, Value};

  use crate::json_rpc::RpcPayload;

  #[test]
  fn test_rpc_payload_serialization() {
    let id: u32 = rand::thread_rng().gen();
    let mut params = Map::new();
    params.insert(String::from("n"), Value::from(100));
    params.insert(String::from("replacement"), Value::from(true));
    params.insert(String::from("min"), Value::from(0));
    params.insert(String::from("max"), Value::from(100));

    let p1 = RpcPayload {
      method: String::from("test"),
      params,
      id,
    };

    let json_p1 = serde_json::to_value(p1);

    assert_eq!(json_p1.is_ok(), true);

    // println!("{:?}", &json_p1.map(|v| v.to_string()));
  }
}
