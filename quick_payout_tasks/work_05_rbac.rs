//! Work 5: RBAC Mock
#[derive(PartialEq)]
pub enum Role { Admin, User }

pub fn has_permission(user_role: Role, required_role: Role) -> bool {
    match (user_role, required_role) {
        (Role::Admin, _) => true,
        (Role::User, Role::User) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rbac() {
        assert!(has_permission(Role::Admin, Role::User));
        assert!(!has_permission(Role::User, Role::Admin));
    }
}
