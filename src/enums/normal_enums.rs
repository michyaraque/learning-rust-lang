struct User {
    name: String,
    email: String,
    year: i32,
    is_active: bool,
    user_role: UserRole,
    social_media: SocialMedia,
}

enum UserRole {
    BASIC,
    ADMIN,
}

enum SocialMedia {
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String),
}

fn main() {
    let user = User {
        name: String::from("Michael"),
        email: String::from("test@test.com"),
        year: 1997,
        is_active: false,
        user_role: UserRole::BASIC,
        social_media: SocialMedia::INSTAGRAM(String::from("@michyaraque")),
    };

    let _access = has_access(user.user_role);
    println!("Has access?: {}", _access);
}

fn has_access(user_role: UserRole) -> bool {
    match user_role {
        UserRole::ADMIN => true,
        UserRole::BASIC => false,
    }
}