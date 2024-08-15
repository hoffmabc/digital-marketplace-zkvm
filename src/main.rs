use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, Value};

fn main() {
    let _program_id = hex::decode(deploy_program()).expect("failed to deploy program");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeployProgramParams {
    elf: Vec<u8>,
}

fn deploy_program() -> String {
    let arch_node_url = "<ARCH_NODE_URL>";

    let elf =
        fs::read("target/digital-marketplace.elf").expect("failed to read digital-marketplace.elf");
    let params = DeployProgramParams { elf };

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(arch_node_url)
        .header("content-type", "application/json")
        .json(&json!({
            "jsonrpc": "2.0",
            "id": "curlycurl",
            "method": "deploy_program",
            "params": params
        }))
        .send()
        .expect("failed to deploy program");

    let result = from_str::<Value>(&res.text().unwrap()).unwrap();
    result["result"].as_str().unwrap().to_string()
}
