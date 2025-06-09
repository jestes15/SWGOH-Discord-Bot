use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    pub message: Value,
    #[serde(rename = "total_count")]
    pub total_count: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "guild_id")]
    pub guild_id: String,
    pub name: String,
    #[serde(rename = "external_message")]
    pub external_message: String,
    #[serde(rename = "banner_color_id")]
    pub banner_color_id: String,
    #[serde(rename = "banner_logo_id")]
    pub banner_logo_id: String,
    #[serde(rename = "enrollment_status")]
    pub enrollment_status: i64,
    #[serde(rename = "galactic_power")]
    pub galactic_power: i64,
    #[serde(rename = "guild_type")]
    pub guild_type: Value,
    #[serde(rename = "level_requirement")]
    pub level_requirement: i64,
    #[serde(rename = "member_count")]
    pub member_count: i64,
    pub members: Vec<Member>,
    #[serde(rename = "avg_galactic_power")]
    pub avg_galactic_power: i64,
    #[serde(rename = "avg_arena_rank")]
    pub avg_arena_rank: f64,
    #[serde(rename = "avg_fleet_arena_rank")]
    pub avg_fleet_arena_rank: f64,
    #[serde(rename = "avg_skill_rating")]
    pub avg_skill_rating: i64,
    #[serde(rename = "last_sync")]
    pub last_sync: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    #[serde(rename = "galactic_power")]
    pub galactic_power: i64,
    #[serde(rename = "guild_join_time")]
    pub guild_join_time: String,
    #[serde(rename = "lifetime_season_score")]
    pub lifetime_season_score: i64,
    #[serde(rename = "member_level")]
    pub member_level: i64,
    #[serde(rename = "ally_code")]
    pub ally_code: i64,
    #[serde(rename = "player_level")]
    pub player_level: i64,
    #[serde(rename = "player_name")]
    pub player_name: String,
    #[serde(rename = "league_id")]
    pub league_id: String,
    #[serde(rename = "league_name")]
    pub league_name: String,
    #[serde(rename = "league_frame_image")]
    pub league_frame_image: String,
    #[serde(rename = "portrait_image")]
    pub portrait_image: String,
    pub title: String,
    #[serde(rename = "squad_power")]
    pub squad_power: i64,
}