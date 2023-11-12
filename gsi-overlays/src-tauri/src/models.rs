use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TeamDraft {
    home_team: bool,
    pick0_id: u16,
    pick0_class: String,
    pick1_id: u16,
    pick1_class: String,
    pick2_id: u16,
    pick2_class: String,
    pick3_id: u16,
    pick3_class: String,
    pick4_id: u16,
    pick4_class: String,
    ban0_id: u16,
    ban0_class: String,
    ban1_id: u16,
    ban1_class: String,
    ban2_id: u16,
    ban2_class: String,
    ban3_id: u16,
    ban3_class: String,
    ban4_id: u16,
    ban4_class: String,
    ban5_id: u16,
    ban5_class: String,
    ban6_id: u16,
    ban6_class: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Draft {
    activeteam: u8,
    pick: bool,
    activeteam_time_remaining: u16,
    radiant_bonus_time: u16,
    dire_bonus_time: u16,
    team2: TeamDraft,
    team3: TeamDraft,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    draft: Option<Draft>,
}
