use std::time::SystemTime;

use crate::rpc_macro::RpcDescription;

impl RpcDescription {
	/// Generate an Insomnia collection from the RPC description.
	pub fn generate_insomnia_collection(&self) -> serde_json::Value {
		let mut json = serde_json::json!({
			"_type": "export",
			"__export_format": 4,
			"__export_date": "2021-03-31T14:00:00.000Z",
			"__export_source": "insomnia.desktop.app:v2021.3.0",
		});

		let collection_id = format!("wrk_{}", uuid::Uuid::new_v4().as_simple().to_string());
		let mut resources = vec![serde_json::json!({
			"_id": collection_id,
			"parentId": null,
			"modified": SystemTime::now(),
			"created": unix_timestamp(),
			"name": "sovereign_sdk_module",
			"description": "",
			"scope": "collection",
			"_type": "workspace",
		})];
		let mut meta_sort_key = 0;
		for method in &self.methods {
			let req_id = format!("req_{}", uuid::Uuid::new_v4().as_simple().to_string());
			let resource = serde_json::json!({
				"_id": req_id,
				"parentId": collection_id,
				"modified": unix_timestamp(),
				"created": unix_timestamp(),
				"url": "http://localhost:3030",
				"name": method.name,
				"description": method.docs.to_string(),
				"method": "POST",
				"body": {
					"mimeType": "application/json",
					"text": r#"
				{
					"jsonrpc": "2.0",
					"method": "method",
					"id": 1,
					"params: []
				}
					"#
				},
				"parameters": [],
				"headers": [
					{
						"name": "Content-Type",
						"value": "application/json"
					}
				],
				"authentication": {},
				"metaSortKey": meta_sort_key,
				"isPrivate": false,
				"settingStoreCookies": true,
				"settingSendCookies": true,
				"settingDisableRenderRequestBody": false,
				"settingEncodeUrl": true,
				"settingRebuildPath": true,
				"settingFollowRedirects": "global",
				"_type": "request"
			});
			meta_sort_key += 1;
			resources.push(resource);
		}

		json["resources"] = serde_json::Value::Array(resources);

		json
	}
}

fn unix_timestamp() -> i64 {
	SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64
}
