use crate::models::{Activity, EligibilityResponse, User};

pub struct EligibilityService {
    users: Vec<User>,
    activities: Vec<Activity>,
}

impl EligibilityService {
    pub fn new() -> Self {
        Self {
            users: User::mock_users(),
            activities: Activity::mock_activities(),
        }
    }

    pub fn get_user(&self, user_id: u64) -> Option<&User> {
        self.users.iter().find(|u| u.id == user_id)
    }

    pub fn get_activity(&self, activity_id: u64) -> Option<&Activity> {
        self.activities.iter().find(|a| a.id == activity_id)
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

        EligibilityResponse {
            eligible: reasons.is_empty(),
            user_id,
            activity_id,
            reasons,
        }
    }
}
