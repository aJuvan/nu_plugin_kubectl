use crate::Kubectl;

use std::fs::File;
use std::io::BufReader;
use indexmap::IndexMap;
use indexmap::indexmap;
use serde::{Serialize, Deserialize};
use nu_protocol::{ReturnValue, ReturnSuccess, UntaggedValue, Dictionary};
use nu_errors::ShellError;
use nu_source::Tag;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct KubeClusterMeta{
	cluster: KubeCluster,
	name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct KubeCluster {
	certificate_authority_data: String,
	server: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct KubeContextMeta {
	context: KubeContext,
	name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct KubeContext {
	cluster: String,
	user: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct KubeUserMeta {
	name: String,
	user: KubeUser,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct KubeUser {
	#[serde(skip_serializing_if = "Option::is_none")]
	client_certificate_data: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	client_key_data: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	token: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct KubeConfig {
	#[serde(rename = "apiVersion")]
	api_version: String,
	clusters: Vec<KubeClusterMeta>,
	contexts: Vec<KubeContextMeta>,
	#[serde(rename = "current-context")]
	current_context: String,
	kind: String,
	users: Vec<KubeUserMeta>,
}


impl Kubectl {

	fn _config_load(& self) -> Result<KubeConfig, ShellError> {
		match File::open(&self.kubeconfig) {
			Ok(file) => {
				let reader = BufReader::new(file);
				Ok(serde_yaml::from_reader(reader)?)
			},
			Err(_) => Err(ShellError::unexpected(format!("Failed to open config file {}", self.kubeconfig)))
		}
	}

	pub fn config_view(& self) -> Result<Vec<ReturnValue>, ShellError>{
		let config: KubeConfig = self._config_load()?;

		let tag: Tag = Tag::unknown();
		let mut map = IndexMap::new();

		map.insert(
			"apiVersion".to_string(),
			UntaggedValue::string(config.api_version).into_value(&tag),
		);

		let mut clusters = Vec::new();
		for cluster in config.clusters {
			let mut clustermap = IndexMap::new();
			clustermap.insert(
				"cluster".to_string(),
				UntaggedValue::Row(Dictionary::from(indexmap!{
					"certificate-authority-data".to_string() =>
							UntaggedValue::string(cluster.cluster.certificate_authority_data).into_value(&tag),
					"server".to_string() =>
							UntaggedValue::string(cluster.cluster.server).into_value(&tag),
				})).into_value(&tag),
			);
			clustermap.insert(
				"name".to_string(),
				UntaggedValue::string(cluster.name).into_value(&tag),
			);

			clusters.push(UntaggedValue::Row(Dictionary::from(clustermap)).into_value(&tag));
		}
		

		map.insert(
			"clusters".to_string(),
			UntaggedValue::Table(clusters).into_value(&tag),
		);

		let mut contexts = Vec::new();
		for context in config.contexts {
			let mut contextmap = IndexMap::new();
			contextmap.insert(
				"context".to_string(),
				UntaggedValue::Row(Dictionary::from(indexmap!{
					"cluster".to_string() => 
							UntaggedValue::string(context.context.cluster).into_value(&tag),
					"user".to_string() =>
							UntaggedValue::string(context.context.user).into_value(&tag),
				})).into_value(&tag)
			);
			contextmap.insert(
				"name".to_string(),
				UntaggedValue::string(context.name).into_value(&tag),
			);

			contexts.push(UntaggedValue::Row(Dictionary::from(contextmap)).into_value(&tag));
		}

		map.insert(
			"contexts".to_string(),
			UntaggedValue::Table(contexts).into_value(&tag),
		);

		map.insert(
			"current_context".to_string(),
			UntaggedValue::string(config.current_context).into_value(&tag),
		);

		map.insert(
			"kind".to_string(),
			UntaggedValue::string(config.kind).into_value(&tag),
		);

		let mut users = Vec::new();
		for user in config.users {
			let mut usermap = IndexMap::new();
			usermap.insert(
				"name".to_string(),
				UntaggedValue::string(user.name).into_value(&tag),
			);
			usermap.insert(
				"user".to_string(),
				UntaggedValue::Row(Dictionary::from(indexmap!{
					"client_certificate_data".to_string() =>
							UntaggedValue::string(user.user.client_certificate_data.unwrap_or("".to_string())).into_value(&tag),
					"client_key_data".to_string() =>
							UntaggedValue::string(user.user.client_key_data.unwrap_or("".to_string())).into_value(&tag),
					"token".to_string() =>
							UntaggedValue::string(user.user.token.unwrap_or("".to_string())).into_value(&tag),
				})).into_value(&tag)
			);

			users.push(UntaggedValue::Row(Dictionary::from(usermap)).into_value(&tag));
		}

		map.insert(
			"users".to_string(),
			UntaggedValue::Table(users).into_value(&tag),
		);

		Ok(vec![
			ReturnSuccess::value(
				UntaggedValue::Row(
					Dictionary::from(map)
				).into_value(&tag)
			)
		])
	}

}
