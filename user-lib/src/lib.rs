use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub age: u8,
}

impl User {
    pub fn new(id: u32, name: String, email: String, age: u8) -> Self {
        Self { id, name, email, age }
    }

    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(anyhow!("Name cannot be empty"));
        }
        if !self.email.contains('@') {
            return Err(anyhow!("Invalid email format"));
        }
        if self.age > 120 {
            return Err(anyhow!("Age must be realistic"));
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct UserManager {
    users: HashMap<u32, User>,
    next_id: u32,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_user(&mut self, name: String, email: String, age: u8) -> Result<u32> {
        let user = User::new(self.next_id, name, email, age);
        user.validate()?;
        
        let id = user.id;
        self.users.insert(id, user);
        self.next_id += 1;
        Ok(id)
    }

    pub fn get_user(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    pub fn remove_user(&mut self, id: u32) -> Result<User> {
        self.users
            .remove(&id)
            .ok_or_else(|| anyhow!("User with id {} not found", id))
    }

    pub fn update_user(&mut self, id: u32, name: Option<String>, email: Option<String>, age: Option<u8>) -> Result<()> {
        let user = self.users
            .get_mut(&id)
            .ok_or_else(|| anyhow!("User with id {} not found", id))?;

        if let Some(name) = name {
            user.name = name;
        }
        if let Some(email) = email {
            user.email = email;
        }
        if let Some(age) = age {
            user.age = age;
        }

        user.validate()?;
        Ok(())
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.users)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let content = fs::read_to_string(path)?;
        self.users = serde_json::from_str(&content)?;
        
        // Update next_id to be one more than the highest existing id
        self.next_id = self.users.keys().max().unwrap_or(&0) + 1;
        Ok(())
    }

    pub fn search_by_name(&self, query: &str) -> Vec<&User> {
        self.users
            .values()
            .filter(|user| user.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut manager = UserManager::new();
        let id = manager.add_user("John Doe".to_string(), "john@example.com".to_string(), 30).unwrap();
        assert_eq!(id, 1);
        assert!(manager.get_user(id).is_some());
    }

    #[test]
    fn test_user_validation() {
        let mut manager = UserManager::new();
        assert!(manager.add_user("".to_string(), "john@example.com".to_string(), 30).is_err());
        assert!(manager.add_user("John".to_string(), "invalid-email".to_string(), 30).is_err());
        assert!(manager.add_user("John".to_string(), "john@example.com".to_string(), 150).is_err());
    }
}
