use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub age: u8,
    pub is_vip: bool,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: u64,
    pub name: String,
    pub min_age: u8,
    pub max_age: Option<u8>,
    pub vip_only: bool,
    pub allowed_regions: Vec<String>,
    pub start_time: DateTime<FixedOffset>,
    pub end_time: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EligibilityRequest {
    pub user_id: u64,
    pub activity_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EligibilityResponse {
    pub eligible: bool,
    pub user_id: u64,
    pub activity_id: u64,
    pub reasons: Vec<String>,
}

impl User {
    pub fn mock_users() -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                age: 25,
                is_vip: true,
                region: "北京".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
                age: 17,
                is_vip: false,
                region: "上海".to_string(),
            },
            User {
                id: 3,
                name: "Charlie".to_string(),
                age: 30,
                is_vip: false,
                region: "北京".to_string(),
            },
            User {
                id: 4,
                name: "Diana".to_string(),
                age: 45,
                is_vip: true,
                region: "广州".to_string(),
            },
        ]
    }
}

impl Activity {
    pub fn mock_activities() -> Vec<Activity> {
        vec![
            Activity {
                id: 101,
                name: "青少年编程大赛".to_string(),
                min_age: 18,
                max_age: Some(25),
                vip_only: false,
                allowed_regions: vec!["北京".to_string(), "上海".to_string()],
                start_time: DateTime::parse_from_rfc3339("2026-01-01T00:00:00+08:00").unwrap(),
                end_time: DateTime::parse_from_rfc3339("2026-12-31T23:59:59+08:00").unwrap(),
            },
            Activity {
                id: 102,
                name: "VIP专属高端峰会".to_string(),
                min_age: 21,
                max_age: None,
                vip_only: true,
                allowed_regions: vec!["北京".to_string(), "上海".to_string(), "广州".to_string()],
                start_time: DateTime::parse_from_rfc3339("2026-06-01T09:00:00+08:00").unwrap(),
                end_time: DateTime::parse_from_rfc3339("2026-06-30T18:00:00+08:00").unwrap(),
            },
            Activity {
                id: 103,
                name: "全民马拉松".to_string(),
                min_age: 16,
                max_age: Some(60),
                vip_only: false,
                allowed_regions: vec![],
                start_time: DateTime::parse_from_rfc3339("2026-03-01T06:00:00+08:00").unwrap(),
                end_time: DateTime::parse_from_rfc3339("2026-03-01T14:00:00+08:00").unwrap(),
            },
        ]
    }
}
