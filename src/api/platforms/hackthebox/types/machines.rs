use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MachineProfile {
    #[serde(flatten)]
    pub info: HashMap<String, Info>,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    id: u32,
    name: String,
    os: String,
    active: u8,
    retired: u8,
    ip: String,
    points: u32,
    static_points: u32,
    release: String,
    user_owns_count: u32,
    root_owns_count: u32,
    free: bool,
    #[serde(rename = "authUserInUserOwns")]
    auth_user_in_user_owns: Option<String>,
    #[serde(rename = "authUserInRootOwns")]
    auth_user_in_root_owns: Option<String>,
    #[serde(rename = "authUserHasReviewed")]
    auth_user_has_reviewed: bool,
    #[serde(rename = "authUserHasSubmittedMatrix")]
    auth_user_has_submitted_matrix: bool,
    stars: f32,
    reviews_count: u32,
    difficulty: u32,
    avatar: String,
    #[serde(rename = "feedbackForChart")]
    pub feedback_for_chart: FeedBackForChart,
    #[serde(rename = "difficultyText")]
    difficulty_text: String,
    #[serde(rename = "isCompleted")]
    is_completed: bool,
    last_reset_time: Option<String>,
    playInfo: PlayInfo,
    maker: Maker,
    maker2: Option<Maker>,
    info_status: Option<String>,
    #[serde(rename = "authUserFirstUserTime")]
    auth_user_first_user_time: Option<String>,
    #[serde(rename = "authUserFirstRootTime")]
    auth_user_first_root_time: Option<String>,
    user_can_review: bool,
    synopsis: String,
    can_access_walkthrough: bool,
    has_changelog: bool,
    #[serde(rename = "userBlood")]
    user_blood: UserBlood,
    #[serde(rename = "userBloodAvatar")]
    user_blood_avatar: String,
    #[serde(rename = "rootBlood")]
    root_blood: UserBlood,
    #[serde(rename = "rootBloodAvatar")]
    root_blood_avatar: String,
    #[serde(rename = "firstUserBloodTime")]
    first_user_blood_time: String,
    #[serde(rename = "firstRootBloodTime")]
    first_root_blood_time: String,
    recommended: u32,
    sp_flag: u32,
    season_id: Option<u32>,
    isGuidedEnabled: bool,
    is_todo: u32,
    start_mode: String,
    show_go_vip: bool,
    show_go_vip_server: bool,
    #[serde(rename = "ownRank")]
    own_rank: Option<String>,
    academy_modules: Vec<AcademyModule>,
    machine_mode: Option<String>,
    lab_server: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedBackForChart {
    counter_cake: u32,
    counter_very_easy: u32,
    counter_easy: u32,
    counter_too_easy: u32,
    counter_medium: u32,
    counter_bit_hard: u32,
    counter_hard: u32,
    counter_too_hard: u32,
    counter_ex_hard: u32,
    counter_brain_fuck: u32,
}

#[derive(Debug, Deserialize)]
pub struct PlayInfo {
    #[serde(rename = "isSpawned")]
    is_spawned: Option<String>,
    #[serde(rename = "isSpawning")]
    is_spawning: Option<String>,
    #[serde(rename = "isActive")]
    is_active: bool,
    active_player_count: Option<u32>,
    expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Maker {
    id: u64,
    name: String,
    avatar: String,
    #[serde(rename = "isRespected")]
    is_respected: bool,
}

#[derive(Debug, Deserialize)]
struct UserBlood {
    user: User,
    created_at: String,
    blood_difference: String,
}

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    id: u64,
    avatar: String,
}

#[derive(Debug, Deserialize)]
struct AcademyModule {
    id: u32,
    name: String,
    logo: String,
    avatar: String,
    difficulty: Difficulty,
    tier: Tier,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Difficulty {
    title: String,
    color: String,
    level: u32,
}

#[derive(Debug, Deserialize)]
struct Tier {
    name: String,
    number: u32,
    color: String,
}
