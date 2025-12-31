use database::models::{tag::Tag, user::User};
use zpi::{
    dto::{
        achievement::AchievementPayload,
        goal::GoalPayload,
        service::{ServicePayloadAdmin, ServicePayloadUser},
        user::UserProfile,
    },
    extractors::AuthenticatedUser,
};

pub struct TestObjects;

impl TestObjects {
    pub fn authenticated_user_1() -> AuthenticatedUser {
        AuthenticatedUser {
            id: 1,
            username: "cheese".into(),
            admin: false,
        }
    }

    pub fn admin_user_1() -> AuthenticatedUser {
        AuthenticatedUser {
            id: 1,
            username: "cheese".into(),
            admin: true,
        }
    }

    pub fn user_1() -> User {
        User {
            id: 1,
            username: "cheese".to_string(),
            about: "Just a test user, doing its job... and fantasizing about a life outside the test environment.".to_string(),
        }
    }

    pub fn user_2() -> User {
        User {
            id: 2,
            username: "wafel".into(),
            about: "I like cheese.".into(),
        }
    }

    pub fn user_profile_1() -> UserProfile {
        UserProfile {
            id: 1,
            username: "cheese".into(),
            about: "Just a test user, doing its job... and fantasizing about a life outside the test environment.".to_string(),
            tags: Vec::new(),
        }
    }

    pub fn user_profile_2() -> UserProfile {
        UserProfile {
            id: 2,
            username: "wafel".into(),
            about: "I like cheese.".into(),
            tags: Self::tags(),
        }
    }

    fn tags() -> Vec<Tag> {
        vec![Self::tag_1(), Self::tag_2()]
    }

    fn tag_1() -> Tag {
        Tag {
            id: 1,
            name: "bestuur".into(),
            category: "bestuur".into(),
            description: Some("Ik ben huidig bestuur".into()),
        }
    }

    fn tag_2() -> Tag {
        Tag {
            id: 2,
            name: "boekentoren".into(),
            category: "toren".into(),
            description: Some("Ik ben een boekentoren".into()),
        }
    }

    pub fn admin_services() -> Vec<ServicePayloadAdmin> {
        vec![Self::admin_service_1(), Self::admin_service_2()]
    }

    pub fn admin_service_1() -> ServicePayloadAdmin {
        ServicePayloadAdmin {
            id: 1,
            name: "zpi".to_string(),
            api_key: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        }
    }

    fn admin_service_2() -> ServicePayloadAdmin {
        ServicePayloadAdmin {
            id: 2,
            name: "zodom".to_string(),
            api_key: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
        }
    }

    pub fn services() -> Vec<ServicePayloadUser> {
        vec![Self::service_1(), Self::service_2()]
    }

    pub fn service_1() -> ServicePayloadUser {
        ServicePayloadUser {
            id: 1,
            name: "zpi".to_string(),
        }
    }

    fn service_2() -> ServicePayloadUser {
        ServicePayloadUser {
            id: 2,
            name: "zodom".to_string(),
        }
    }

    pub fn achievement_1() -> AchievementPayload {
        AchievementPayload {
            id: 1,
            name: "Achievements".into(),
            goals: vec![
                GoalPayload {
                    id: 1,
                    description: "Get 1 achievement".into(),
                    sequence: 0,
                },
                GoalPayload {
                    id: 2,
                    description: "Get 2 achievements".into(),
                    sequence: 1,
                },
            ],
        }
    }

    pub fn achievement_2() -> AchievementPayload {
        AchievementPayload {
            id: 2,
            name: "Profile Picture".into(),
            goals: vec![GoalPayload {
                id: 3,
                description: "Upload a profile picture".into(),
                sequence: 0,
            }],
        }
    }

    pub fn achievement_3() -> AchievementPayload {
        AchievementPayload {
            id: 3,
            name: "Votes".into(),
            goals: vec![GoalPayload {
                id: 4,
                description: "Vote 1 time".into(),
                sequence: 1,
            }],
        }
    }
}
