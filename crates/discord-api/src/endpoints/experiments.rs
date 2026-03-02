use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::serialize_query_params;

pub fn GET_EXPERIMENT_ASSIGNMENTS(query_params: &GetExperimentAssignmentsQuery) -> String {
	format!("/experiments{}", serialize_query_params(query_params))
}

#[derive(Serialize, Deserialize)]
pub struct GetExperimentAssignmentsQuery {
	/// Whether to include guild experiments in the returned data
	#[serde(skip_serializing_if = "Option::is_none")]
	pub with_guild_experiments: Option<bool>,
	/// Whether to also include experiments for the given platform
	///
	/// Requires valid authentication if included
	#[serde(skip_serializing_if = "Option::is_none")]
	pub platform: Option<ExperimentPlatform>,
}

#[derive(Serialize, Deserialize)]
pub enum ExperimentPlatform {
	/// The developer portal
	DEVELOPER_PORTAL,
}

#[derive(Serialize, Deserialize)]
pub struct GetExperimentAssignmentsResponse {
	/// A generated fingerprint of the current date and time
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fingerprint: Option<String>,
	/// The experiment assignments for this user or fingerprint
	// pub assignments: Vec<UserExperiment>,
	pub assignments: Vec<Value>,
	/// Guild experiment rollouts for the client to assign
	#[serde(skip_serializing_if = "Option::is_none")]
	// pub guild_experiments: Option<Vec<GuildExperiment>>,
	pub guild_experiments: Option<Value>,
}

// todo: serialize/deserialize as an array
/// This object is represented as an array of the following fields:
#[derive(Serialize, Deserialize)]
pub struct UserExperiment {
	/// 32-bit unsigned Murmur3 hash of the experiment's name
	pub hash: i64,
	/// Current version of the rollout
	pub revision: i64,
	/// The requesting user or fingerprint's assigned experiment bucket
	pub bucket: i64,
	/// Whether the user or fingerprint has an override for the experiment ( -1 for false, 0 for true)
	pub r#override: i64,
	/// The internal population group the requesting user or fingerprint is in
	pub population: i64,
	/// The calculated rollout position to use, prioritized over local calculations
	pub hash_result: i64,
	/// The experiment's A/A testing mode, represented as an integer-casted boolean
	pub aa_mode: i64,
	/// Whether the experiment's analytics trigger debugging is enabled, represented as an integer-casted boolean
	pub trigger_debugging: i64,
	/// A human-readable experiment name (formatted as year-month_name) that disables the experiment
	pub holdout_name: Option<String>,
	/// The revision of the holdout experiment
	pub holdout_revision: Option<i64>,
	/// The requesting user or fingerprint's assigned bucket for the holdout experiment
	pub holdout_bucket: Option<i64>,
}

/// This object is represented as an array of the following fields:
#[derive(Serialize, Deserialize)]
pub struct GuildExperiment {
	/// 32-bit unsigned Murmur3 hash of the experiment's name
	pub hash: i64,
	/// A human-readable experiment name (formatted as year-month_name ) to use for hashing calculations, prioritized over the client name
	pub hash_key: Option<String>,
	/// Current version of the rollout
	pub revision: i64,
	/// The experiment rollout's populations
	pub populations: Vec<ExperimentPopulation>,
	/// Specific bucket overrides for the experiment
	pub overrides: Vec<ExperimentBucketOverride>,
	/// Populations of overrides for the experiment
	pub overrides_formatted: Vec<Vec<ExperimentPopulation>>,
	/// A human-readable experiment name (formatted as year-month_name ) that disables the experiment
	pub holdout_name: Option<String>,
	/// The holdout experiment bucket that disables the experiment
	pub holdout_bucket: Option<i64>,
	/// The experiment's A/A testing mode, represented as an integer-casted boolean
	pub aa_mode: i64,
	/// Whether the experiment's analytics trigger debugging is enabled, represented as an integer-casted boolean
	pub trigger_debugging: i64,
}

/// This object is represented as an array of the following fields:
#[derive(Serialize, Deserialize)]
pub struct ExperimentPopulation {
	/// The ranges for this population
	pub ranges: Vec<ExperimentPopulationRange>,
	/// The filters that the resource must satisfy to be in this population
	pub filters: ExperimentPopulationFilters,
}

/// This object is represented as an array of the following fields:
#[derive(Serialize, Deserialize)]
pub struct ExperimentPopulationRange {
	/// The bucket this range grants
	pub bucket: i64,
	/// The range rollout
	pub rollout: Vec<ExperimentPopulationRollout>,
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentPopulationRollout {
	/// The start of this range
	pub s: i64,
	/// The end of this range
	pub e: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentPopulationFilters {
	/// The guild features that are eligible
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_has_feature: Option<ExperimentPopulationGuildFeatureFilter>,
	/// The range of snowflake resource IDs that are eligible
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_id_range: Option<ExperimentPopulationRangeFilter>,
	/// The range of guild ages (in days) that are eligible
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_age_range_days: Option<ExperimentPopulationRangeFilter>,
	/// The range of guild member counts that are eligible
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_member_count_range: Option<ExperimentPopulationRangeFilter>,
	/// A list of resource IDs that are eligible
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_ids: Option<ExperimentPopulationIdFilter>,
	/// A list of hub types that are eligible
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_hub_types: Option<ExperimentPopulationHubTypeFilter>,
	/// Whether the guild must or must not have a vanity to be eligible
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_has_vanity_url: Option<ExperimentPopulationVanityUrlFilter>,
	/// The special rollout position limits on the population
	#[serde(skip_serializing_if = "Option::is_none")]
	pub guild_in_range_by_hash: Option<ExperimentPopulationRangeByHashFilter>,
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentPopulationGuildFeatureFilter {
	/// The guild features eligible for this population; only one feature is required for eligibility
	pub guild_features: Vec<String>,
	// pub guild_features: Vec<GuildFeatures>,
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentPopulationRangeFilter {
	/// The exclusive minimum for this range, if any
	pub min_id: Option<String>,
	// pub min_id: Option<Snowflake>,
	/// The exclusive maximum for this range, if any
	pub max_id: Option<String>,
	// pub max_id: Option<Snowflake>,
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentPopulationIdFilter {
	/// The list of snowflake resource IDs that are eligible for this population
	pub guild_ids: Vec<String>,
	// pub guild_ids: Vec<Snowflake>,
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentPopulationHubTypeFilter {
	/// The type of hubs that are eligible for this population
	// pub guild_hub_types: Vec<HubType>,
	pub guild_hub_types: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentPopulationVanityUrlFilter {
	/// The required vanity URL holding status for this population
	pub guild_has_vanity_url: bool,
}

/// This filter is used to limit rollout position by an additional hash key.
/// The calculated rollout position must be less than the given target.
#[derive(Serialize, Deserialize)]
pub struct ExperimentPopulationRangeByHashFilter {
	/// The 32-bit unsigned Murmur3 hash of the key used to determine eligibility
	pub hash_key: i64,
	/// The rollout position limit for this population
	pub target: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentBucketOverride {
	/// Bucket assigned to these resources
	pub b: i64,
	/// Resources granted access to this bucket
	// pub k: Vec<Snowflake>,
	pub k: Vec<String>,
}
