use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MachineProfile {
    #[serde(flatten)]
    pub info: HashMap<String, ProfileInfo>,
}

#[derive(Debug, Deserialize)]
pub struct MachineList {
    pub data: Vec<ListInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileInfo {
    pub id: u32,
    pub name: String,
    pub os: String,
    pub active: u8,
    pub retired: u8,
    pub ip: String,
    pub points: u32,
    pub static_points: u32,
    pub release: String,
    pub user_owns_count: u32,
    pub root_owns_count: u32,
    pub free: bool,
    #[serde(rename = "authUserInUserOwns")]
    pub auth_user_in_user_owns: Option<String>,
    #[serde(rename = "authUserInRootOwns")]
    pub auth_user_in_root_owns: Option<String>,
    #[serde(rename = "authUserHasReviewed")]
    pub auth_user_has_reviewed: bool,
    #[serde(rename = "authUserHasSubmittedMatrix")]
    pub auth_user_has_submitted_matrix: bool,
    pub stars: f32,
    pub reviews_count: u32,
    pub difficulty: u32,
    pub avatar: String,
    #[serde(rename = "feedbackForChart")]
    pub feedback_for_chart: FeedBackForChart,
    #[serde(rename = "difficultyText")]
    pub difficulty_text: String,
    #[serde(rename = "isCompleted")]
    pub is_completed: bool,
    pub last_reset_time: Option<String>,
    #[serde(rename = "playInfo")]
    pub play_info: PlayInfo,
    pub maker: Maker,
    pub maker2: Option<Maker>,
    pub info_status: Option<String>,
    #[serde(rename = "authUserFirstUserTime")]
    pub auth_user_first_user_time: Option<String>,
    #[serde(rename = "authUserFirstRootTime")]
    pub auth_user_first_root_time: Option<String>,
    pub user_can_review: bool,
    pub synopsis: Option<String>,
    pub can_access_walkthrough: bool,
    pub has_changelog: bool,
    #[serde(rename = "userBlood")]
    pub user_blood: UserBlood,
    #[serde(rename = "userBloodAvatar")]
    pub user_blood_avatar: String,
    #[serde(rename = "rootBlood")]
    pub root_blood: UserBlood,
    #[serde(rename = "rootBloodAvatar")]
    pub root_blood_avatar: String,
    #[serde(rename = "firstUserBloodTime")]
    pub first_user_blood_time: String,
    #[serde(rename = "firstRootBloodTime")]
    pub first_root_blood_time: String,
    pub recommended: u32,
    pub sp_flag: u32,
    pub season_id: Option<u32>,
    #[serde(rename = "isGuidedEnabled")]
    pub is_guided_enabled: bool,
    pub is_todo: u32,
    pub start_mode: String,
    pub show_go_vip: bool,
    pub show_go_vip_server: bool,
    #[serde(rename = "ownRank")]
    pub own_rank: Option<String>,
    pub academy_modules: Vec<AcademyModule>,
    pub machine_mode: Option<String>,
    pub lab_server: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListInfo {
    pub id: u32,
    pub avatar: String,
    pub name: String,
    pub static_points: u32,
    pub sp_flag: u8,
    pub os: String,
    pub points: u32,
    pub star: f32,
    pub release: String,
    pub easy_month: u8,
    pub poweroff: u8,
    pub free: bool,
    pub difficulty: u32,
    #[serde(rename = "difficultyText")]
    pub difficulty_text: String,
    pub user_owns_count: u32,
    #[serde(rename = "authUserInUserOwns")]
    pub auth_user_in_user_owns: bool,
    pub root_owns_count: u32,
    #[serde(rename = "authUserHasReviewed")]
    pub auth_user_has_reviewed: bool,
    #[serde(rename = "authUserInRootOwns")]
    pub auth_user_in_root_owns: bool,
    #[serde(rename = "isTodo")]
    pub is_todo: bool,
    pub is_competitive: bool,
    pub active: Option<bool>,
    #[serde(rename = "feedbackForChart")]
    pub feedback_for_chart: FeedBackForChart,
    pub ip: Option<String>,
    #[serde(rename = "playInfo")]
    pub play_info: PlayInfo,
    pub labels: Vec<Label>,
    pub recommended: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedBackForChart {
    pub counter_cake: u32,
    pub counter_very_easy: u32,
    pub counter_easy: u32,
    pub counter_too_easy: u32,
    pub counter_medium: u32,
    pub counter_bit_hard: u32,
    pub counter_hard: u32,
    pub counter_too_hard: u32,
    pub counter_ex_hard: u32,
    pub counter_brain_fuck: u32,
}

#[derive(Debug, Deserialize)]
pub struct PlayInfo {
    #[serde(rename = "isSpawned")]
    pub is_spawned: Option<String>,
    #[serde(rename = "isSpawning")]
    pub is_spawning: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
    pub active_player_count: Option<u32>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Maker {
    pub id: u64,
    pub name: String,
    pub avatar: String,
    #[serde(rename = "isRespected")]
    pub is_respected: bool,
}

#[derive(Debug, Deserialize)]
pub struct UserBlood {
    pub user: User,
    pub created_at: String,
    pub blood_difference: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
    pub id: u64,
    pub avatar: String,
}

#[derive(Debug, Deserialize)]
pub struct AcademyModule {
    pub id: u32,
    pub name: String,
    pub logo: String,
    pub avatar: String,
    pub difficulty: Difficulty,
    pub tier: Tier,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Difficulty {
    pub title: String,
    pub color: String,
    pub level: u32,
}

#[derive(Debug, Deserialize)]
pub struct Tier {
    pub name: String,
    pub number: u32,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct Label {
    pub color: String,
    pub name: String,
}
