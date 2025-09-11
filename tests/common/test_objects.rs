use database::models::{
    tag::Tag,
    user::{User, UserProfile},
};
use zpi::extractors::AuthenticatedUser;

pub struct TestObjects;

impl TestObjects {
    pub fn authenticated_user_1() -> AuthenticatedUser {
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
}
