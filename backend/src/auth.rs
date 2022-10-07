use argon2;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zxcvbn;

#[derive(Debug)]
pub struct Users {
    users: Vec<User>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserName {
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserNameChange {
    pub username: String,
    pub new_username: String,
}

#[derive(Serialize)]
pub struct SessionId {
    pub session_id: String,
}

#[derive(Default, Debug)]
pub struct SessionStorage {
    pub sessions: HashMap<String, String>,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    UserAlreadyExists,
    UserNotFound,
    WeakPassword,
}

impl Users {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn add_user(&mut self, name: String, pass: String) -> Result<(), Error> {
        if self.get_idx_by_name(name.as_str()) != None {
            Err(Error::UserAlreadyExists)
        } else if !is_strong(pass.as_str()) {
            Err(Error::WeakPassword)
        } else {
            Ok(self.users.push(User::new(name, pass)))
        }
    }

    pub fn remove_user(&mut self, name: String) -> Result<(), Error> {
        if let Some(i) = self.get_idx_by_name(name.as_str()) {
            self.users.remove(i);
            Ok(())
        } else {
            Err(Error::UserNotFound)
        }
    }

    pub fn change_name(&mut self, name: String, new_name: String) -> Result<(), Error> {
        if let Some(_) = self.get_idx_by_name(new_name.as_str()) {
            Err(Error::UserAlreadyExists)
        } else if let Some(i) = self.get_idx_by_name(name.as_str()) {
            Ok(self.users[i].username = new_name)
        } else {
            Err(Error::UserNotFound)
        }
    }

    pub fn change_pass(&mut self, name: String, new_pass: String) -> Result<(), Error> {
        if !is_strong(new_pass.as_str()) {
            Err(Error::WeakPassword)
        } else if let Some(i) = self.get_idx_by_name(name.as_str()) {
            Ok(self.users[i].password = hash_pass(new_pass.as_bytes()))
        } else {
            Err(Error::UserNotFound)
        }
    }

    pub fn verify(&self, name: &str, pass: &str) -> bool {
        if let Some(i) = self.get_idx_by_name(name) {
            argon2::verify_encoded(self.users[i].password.as_str(), pass.as_bytes()).unwrap()
        } else {
            false
        }
    }

    fn get_idx_by_name(&self, name: &str) -> Option<usize> {
        self.users.iter().position(|x| x.username == name)
    }
}

impl User {
    fn new(username: String, password: String) -> Self {
        Self {
            username,
            password: hash_pass(password.as_bytes()),
        }
    }
}

fn hash_pass(pass: &[u8]) -> String {
    let config = argon2::Config::default();
    argon2::hash_encoded(pass, random_salt().as_bytes(), &config).unwrap()
}

fn random_salt() -> String {
    let mut rng = thread_rng();
    (0..8).map(|_| rng.sample(Alphanumeric) as char).collect()
}

fn is_strong(pass: &str) -> bool {
    let score = zxcvbn::zxcvbn(pass, &[]);
    match score {
        Ok(_) => score.unwrap().score() >= 3,
        Err(_) => false,
    }
}

#[test]
fn test() {
    let mut users = Users::new();
    assert_eq!(users.add_user("a".to_string(), "abcdef".to_string()), Err(Error::WeakPassword));
    let _ = users.add_user("a".to_string(), "example_#pass#word#__a".to_string());
    let _ = users.add_user("b".to_string(), "example_#pass#word#__b".to_string());
    let _ = users.add_user("c".to_string(), "example_#pass#word#__c".to_string());

    let _ = users.add_user("a".to_string(), "example_#pass#word#__i".to_string());
    assert!(users.verify("a", "example_#pass#word#__a"));
    assert!(!users.verify("a", "example_#pass#word#__i"));

    assert!(users.verify("b", "example_#pass#word#__b"));
    let _ = users.change_name("b".to_string(), "i".to_string());
    assert!(users.verify("i", "example_#pass#word#__b"));
    assert!(!users.verify("b", "example_#pass#word#__b"));

    assert!(users.verify("c", "example_#pass#word#__c"));
    assert_eq!(users.change_pass("c".to_string(), "abcdef".to_string()), Err(Error::WeakPassword));
    let _ = users.change_pass("c".to_string(), "example_#pass#word#__i".to_string());
    assert!(users.verify("c", "example_#pass#word#__i"));
    assert!(!users.verify("c", "example_#pass#word#__c"));

    let _ = users.remove_user("i".to_string());
    assert!(!users.verify("i", "example_#pass#word#__b"));

    let _ = users.add_user("d".to_string(), "example_#pass#word#__a".to_string());
    assert!(users.verify("a", "example_#pass#word#__a"));
    assert!(users.verify("d", "example_#pass#word#__a"));
}
