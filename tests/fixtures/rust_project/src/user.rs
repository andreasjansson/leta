use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct User {
    name: String,
    email: String,
    age: u32,
}

impl User {
    pub fn new(name: String, email: String, age: u32) -> Self {
        Self { name, email, age }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn age(&self) -> u32 {
        self.age
    }
}

pub struct UserRepository {
    users: HashMap<String, User>,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.email.clone(), user);
    }

    pub fn get_user(&self, email: &str) -> Option<&User> {
        self.users.get(email)
    }

    pub fn delete_user(&mut self, email: &str) -> bool {
        self.users.remove(email).is_some()
    }

    pub fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}
