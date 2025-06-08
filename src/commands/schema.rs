use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    pub units: Vec<Unit>,
    pub mods: Vec<Mod>,
    pub datacrons: Vec<Datacron>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "ally_code")]
    pub ally_code: i64,
    #[serde(rename = "arena_leader_base_id")]
    pub arena_leader_base_id: String,
    #[serde(rename = "arena_rank")]
    pub arena_rank: i64,
    pub level: i64,
    pub name: String,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "galactic_power")]
    pub galactic_power: i64,
    #[serde(rename = "character_galactic_power")]
    pub character_galactic_power: i64,
    #[serde(rename = "ship_galactic_power")]
    pub ship_galactic_power: i64,
    #[serde(rename = "ship_battles_won")]
    pub ship_battles_won: i64,
    #[serde(rename = "pvp_battles_won")]
    pub pvp_battles_won: i64,
    #[serde(rename = "pve_battles_won")]
    pub pve_battles_won: i64,
    #[serde(rename = "pve_hard_won")]
    pub pve_hard_won: i64,
    #[serde(rename = "galactic_war_won")]
    pub galactic_war_won: i64,
    #[serde(rename = "guild_raid_won")]
    pub guild_raid_won: i64,
    #[serde(rename = "guild_contribution")]
    pub guild_contribution: i64,
    #[serde(rename = "guild_exchange_donations")]
    pub guild_exchange_donations: i64,
    #[serde(rename = "season_full_clears")]
    pub season_full_clears: i64,
    #[serde(rename = "season_successful_defends")]
    pub season_successful_defends: i64,
    #[serde(rename = "season_league_score")]
    pub season_league_score: i64,
    #[serde(rename = "season_undersized_squad_wins")]
    pub season_undersized_squad_wins: i64,
    #[serde(rename = "season_promotions_earned")]
    pub season_promotions_earned: i64,
    #[serde(rename = "season_banners_earned")]
    pub season_banners_earned: i64,
    #[serde(rename = "season_offensive_battles_won")]
    pub season_offensive_battles_won: i64,
    #[serde(rename = "season_territories_defeated")]
    pub season_territories_defeated: i64,
    pub url: String,
    pub arena: Arena,
    #[serde(rename = "fleet_arena")]
    pub fleet_arena: FleetArena,
    #[serde(rename = "skill_rating")]
    pub skill_rating: i64,
    #[serde(rename = "league_name")]
    pub league_name: String,
    #[serde(rename = "league_frame_image")]
    pub league_frame_image: String,
    #[serde(rename = "league_blank_image")]
    pub league_blank_image: String,
    #[serde(rename = "league_image")]
    pub league_image: String,
    #[serde(rename = "division_number")]
    pub division_number: i64,
    #[serde(rename = "division_image")]
    pub division_image: String,
    #[serde(rename = "portrait_image")]
    pub portrait_image: String,
    pub title: String,
    #[serde(rename = "guild_id")]
    pub guild_id: String,
    #[serde(rename = "guild_name")]
    pub guild_name: String,
    #[serde(rename = "guild_url")]
    pub guild_url: String,
    pub mods: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arena {
    pub rank: i64,
    pub leader: String,
    pub members: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FleetArena {
    pub rank: i64,
    pub leader: String,
    pub members: Vec<String>,
    pub reinforcements: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub data: Data2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data2 {
    #[serde(rename = "base_id")]
    pub base_id: String,
    pub name: String,
    #[serde(rename = "gear_level")]
    pub gear_level: i64,
    pub level: i64,
    pub power: i64,
    pub rarity: i64,
    pub gear: Vec<Gear>,
    pub url: String,
    pub stats: Stats,
    #[serde(rename = "stat_diffs")]
    pub stat_diffs: StatDiffs,
    #[serde(rename = "zeta_abilities")]
    pub zeta_abilities: Vec<String>,
    #[serde(rename = "omicron_abilities")]
    pub omicron_abilities: Vec<String>,
    #[serde(rename = "ability_data")]
    pub ability_data: Vec<AbilityDaum>,
    #[serde(rename = "mod_set_ids")]
    pub mod_set_ids: Vec<String>,
    #[serde(rename = "combat_type")]
    pub combat_type: i64,
    #[serde(rename = "relic_tier")]
    pub relic_tier: Option<i64>,
    #[serde(rename = "has_ultimate")]
    pub has_ultimate: bool,
    #[serde(rename = "is_galactic_legend")]
    pub is_galactic_legend: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gear {
    pub slot: i64,
    #[serde(rename = "is_obtained")]
    pub is_obtained: bool,
    #[serde(rename = "base_id")]
    pub base_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    #[serde(rename = "2")]
    pub n2: f64,
    #[serde(rename = "3")]
    pub n3: f64,
    #[serde(rename = "4")]
    pub n4: f64,
    #[serde(rename = "1")]
    pub n1: f64,
    #[serde(rename = "28")]
    pub n28: Option<f64>,
    #[serde(rename = "5")]
    pub n5: f64,
    #[serde(rename = "16")]
    pub n16: f64,
    #[serde(rename = "17")]
    pub n17: Option<f64>,
    #[serde(rename = "18")]
    pub n18: f64,
    #[serde(rename = "27")]
    pub n27: Option<f64>,
    #[serde(rename = "6")]
    pub n6: f64,
    #[serde(rename = "14")]
    pub n14: f64,
    #[serde(rename = "10")]
    pub n10: Option<f64>,
    #[serde(rename = "37")]
    pub n37: i64,
    #[serde(rename = "8")]
    pub n8: f64,
    #[serde(rename = "12")]
    pub n12: i64,
    #[serde(rename = "39")]
    pub n39: i64,
    #[serde(rename = "7")]
    pub n7: f64,
    #[serde(rename = "15")]
    pub n15: f64,
    #[serde(rename = "11")]
    pub n11: Option<f64>,
    #[serde(rename = "38")]
    pub n38: i64,
    #[serde(rename = "9")]
    pub n9: f64,
    #[serde(rename = "13")]
    pub n13: i64,
    #[serde(rename = "40")]
    pub n40: i64,
    #[serde(rename = "61")]
    pub n61: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatDiffs {
    #[serde(rename = "1")]
    pub n1: Option<f64>,
    #[serde(rename = "28")]
    pub n28: Option<f64>,
    #[serde(rename = "17")]
    pub n17: Option<f64>,
    #[serde(rename = "6")]
    pub n6: Option<f64>,
    #[serde(rename = "14")]
    pub n14: Option<f64>,
    #[serde(rename = "8")]
    pub n8: Option<f64>,
    #[serde(rename = "7")]
    pub n7: Option<f64>,
    #[serde(rename = "15")]
    pub n15: Option<f64>,
    #[serde(rename = "9")]
    pub n9: Option<f64>,
    #[serde(rename = "5")]
    pub n5: Option<f64>,
    #[serde(rename = "18")]
    pub n18: Option<f64>,
    #[serde(rename = "16")]
    pub n16: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityDaum {
    pub id: String,
    #[serde(rename = "ability_tier")]
    pub ability_tier: i64,
    #[serde(rename = "is_omega")]
    pub is_omega: bool,
    #[serde(rename = "is_zeta")]
    pub is_zeta: bool,
    #[serde(rename = "is_omicron")]
    pub is_omicron: bool,
    #[serde(rename = "has_omicron_learned")]
    pub has_omicron_learned: bool,
    #[serde(rename = "has_zeta_learned")]
    pub has_zeta_learned: bool,
    pub name: String,
    #[serde(rename = "tier_max")]
    pub tier_max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    pub id: String,
    pub level: i64,
    pub tier: i64,
    pub rarity: i64,
    pub set: String,
    pub slot: i64,
    #[serde(rename = "primary_stat")]
    pub primary_stat: PrimaryStat,
    pub character: String,
    #[serde(rename = "secondary_stats")]
    pub secondary_stats: Vec<SecondaryStat>,
    #[serde(rename = "reroll_count")]
    pub reroll_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryStat {
    pub name: String,
    #[serde(rename = "stat_id")]
    pub stat_id: i64,
    pub value: f64,
    #[serde(rename = "display_value")]
    pub display_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryStat {
    pub name: String,
    #[serde(rename = "stat_id")]
    pub stat_id: i64,
    pub value: f64,
    #[serde(rename = "display_value")]
    pub display_value: String,
    pub roll: i64,
    #[serde(rename = "unscaled_roll_values")]
    pub unscaled_roll_values: Vec<i64>,
    #[serde(rename = "stat_max")]
    pub stat_max: i64,
    #[serde(rename = "stat_min")]
    pub stat_min: i64,
    #[serde(rename = "stat_rolls")]
    pub stat_rolls: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datacron {
    pub id: String,
    #[serde(rename = "set_id")]
    pub set_id: i64,
    #[serde(rename = "template_base_id")]
    pub template_base_id: String,
    #[serde(rename = "reroll_count")]
    pub reroll_count: i64,
    #[serde(rename = "reroll_index")]
    pub reroll_index: i64,
    pub locked: bool,
    pub tier: i64,
    pub tiers: Vec<Tier>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tier {
    #[serde(rename = "scope_identifier")]
    pub scope_identifier: i64,
    #[serde(rename = "scope_icon")]
    pub scope_icon: String,
    #[serde(rename = "scope_target_name")]
    pub scope_target_name: String,
    #[serde(rename = "target_rule_id")]
    pub target_rule_id: Option<String>,
    #[serde(rename = "ability_id")]
    pub ability_id: Option<String>,
    #[serde(rename = "stat_type")]
    pub stat_type: i64,
    #[serde(rename = "stat_value")]
    pub stat_value: f64,
    #[serde(rename = "required_unit_tier")]
    pub required_unit_tier: i64,
    #[serde(rename = "required_relic_tier")]
    pub required_relic_tier: i64,
    #[serde(rename = "ability_description")]
    pub ability_description: Option<String>,
}

#[derive(Default, Debug)]
pub struct Ability {
    pub name: String,
    pub level: i64,
    pub progress: String,
    pub max_ab_level: i64
}
