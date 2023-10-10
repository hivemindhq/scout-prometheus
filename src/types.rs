use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Average {
    pub auto_points: i32,
    pub total_points: i32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TeamStats {
    pub wins: i32,
    pub losses: i32,
    pub rank: i32,
    pub average: Average,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TeamEvent {
    pub event_code: String,
    pub states: TeamStats,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TeamStruct {
    pub number: i32,
    pub name: String,
    pub events: Vec<TeamEvent>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Team {
    pub team: TeamStruct,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EventByCode {
    pub teams: Vec<Team>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EventByCodeResponse {
    event_by_code: EventByCode,
}
