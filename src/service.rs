use chrono::{DateTime, FixedOffset, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use crate::models::{Activity, EligibilityResponse, User};

const SERVER_TIMEZONE_OFFSET: i32 = 8 * 3600;

fn get_server_now() -> DateTime<FixedOffset> {
    let cst = FixedOffset::east_opt(SERVER_TIMEZONE_OFFSET).unwrap();
    Utc::now().with_timezone(&cst)
}

pub struct EligibilityService {
    users: Vec<User>,
    activities: Vec<Activity>,
    registrations: Mutex<HashMap<u64, HashSet<u64>>>,
}

impl EligibilityService {
    pub fn new() -> Self {
        Self {
            users: User::mock_users(),
            activities: Activity::mock_activities(),
            registrations: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_user(&self, user_id: u64) -> Option<&User> {
        self.users.iter().find(|u| u.id == user_id)
    }

    pub fn get_activity(&self, activity_id: u64) -> Option<&Activity> {
        self.activities.iter().find(|a| a.id == activity_id)
    }

    pub fn get_registration_stats(&self, activity_id: u64) -> (u32, u32, u32) {
        let activity = self.get_activity(activity_id);
        let total = activity.map(|a| a.total_slots).unwrap_or(0);
        let registered = self
            .registrations
            .lock()
            .unwrap()
            .get(&activity_id)
            .map(|set| set.len() as u32)
            .unwrap_or(0);
        let remaining = total.saturating_sub(registered);
        (total, registered, remaining)
    }

    pub fn is_user_registered(&self, user_id: u64, activity_id: u64) -> bool {
        self.registrations
            .lock()
            .unwrap()
            .get(&activity_id)
            .map(|set| set.contains(&user_id))
            .unwrap_or(false)
    }

    pub fn register_user(&self, user_id: u64, activity_id: u64) -> Result<(u32, u32, u32), String> {
        let eligibility = self.check_eligibility(user_id, activity_id);
        if !eligibility.eligible {
            return Err(if eligibility.reasons.is_empty() {
                "不符合参与资格".to_string()
            } else {
                eligibility.reasons.join("; ")
            });
        }

        let mut regs = self.registrations.lock().unwrap();
        let entry = regs.entry(activity_id).or_insert_with(HashSet::new);

        if entry.contains(&user_id) {
            return Err("用户已报名该活动".to_string());
        }

        let activity = self.get_activity(activity_id).unwrap();
        if entry.len() as u32 >= activity.total_slots {
            return Err("活动名额已满".to_string());
        }

        entry.insert(user_id);
        let registered = entry.len() as u32;
        let total = activity.total_slots;
        let remaining = total.saturating_sub(registered);
        Ok((total, registered, remaining))
    }

    pub fn check_eligibility(&self, user_id: u64, activity_id: u64) -> EligibilityResponse {
        let mut reasons: Vec<String> = Vec::new();

        let user = match self.get_user(user_id) {
            Some(u) => u,
            None => {
                reasons.push(format!("用户 ID {} 不存在", user_id));
                return EligibilityResponse {
                    eligible: false,
                    user_id,
                    activity_id,
                    reasons,
                    remaining_slots: 0,
                    total_slots: 0,
                    registered_count: 0,
                };
            }
        };

        let activity = match self.get_activity(activity_id) {
            Some(a) => a,
            None => {
                reasons.push(format!("活动 ID {} 不存在", activity_id));
                return EligibilityResponse {
                    eligible: false,
                    user_id,
                    activity_id,
                    reasons,
                    remaining_slots: 0,
                    total_slots: 0,
                    registered_count: 0,
                };
            }
        };

        if user.age < activity.min_age {
            reasons.push(format!(
                "年龄不足：用户 {} 岁，活动要求最低 {} 岁",
                user.age, activity.min_age
            ));
        }

        if let Some(max_age) = activity.max_age {
            if user.age > max_age {
                reasons.push(format!(
                    "年龄超限：用户 {} 岁，活动要求最高 {} 岁",
                    user.age, max_age
                ));
            }
        }

        if activity.vip_only && !user.is_vip {
            reasons.push("该活动仅限 VIP 用户参与".to_string());
        }

        if !activity.allowed_regions.is_empty() {
            if !activity.allowed_regions.contains(&user.region) {
                reasons.push(format!(
                    "所在地区 {} 不在活动允许范围内",
                    user.region
                ));
            }
        }

        let now = get_server_now();
        if now < activity.start_time {
            reasons.push(format!(
                "活动尚未开始：活动开始时间为 {}（服务器时间），当前时间为 {}",
                activity.start_time.format("%Y-%m-%d %H:%M:%S %:z"),
                now.format("%Y-%m-%d %H:%M:%S %:z")
            ));
        }
        if now > activity.end_time {
            reasons.push(format!(
                "活动已结束：活动结束时间为 {}（服务器时间），当前时间为 {}",
                activity.end_time.format("%Y-%m-%d %H:%M:%S %:z"),
                now.format("%Y-%m-%d %H:%M:%S %:z")
            ));
        }

        let (total_slots, registered_count, remaining_slots) =
            self.get_registration_stats(activity_id);

        if remaining_slots == 0 {
            reasons.push(format!(
                "活动名额已满：总名额 {}，已报名 {}",
                total_slots, registered_count
            ));
        }

        if self.is_user_registered(user_id, activity_id) {
            reasons.push("用户已报名该活动".to_string());
        }

        EligibilityResponse {
            eligible: reasons.is_empty(),
            user_id,
            activity_id,
            reasons,
            remaining_slots,
            total_slots,
            registered_count,
        }
    }
}
