use argon2;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

#[derive(Debug)]
pub struct Users {
    users: Vec<User>,
}

#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

enum Error {
    UserAlreadyExists,
    UserNotFound,
}

impl Users {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn add_user(&mut self, name: String, pass: String) -> Result<(), Error> {
        if !self.user_exists(name.as_str()) {
            Ok(self.users.push(User::new(name, hash_pass(pass.as_bytes()))))
        } else { Err(Error::UserAlreadyExists) }
    }

    pub fn remove_user(&mut self, name: String) -> Result<(), Error> {
        for i in 0..self.users.len() {
            if self.users[i].username == name {
                self.users.remove(i);
                return Ok(())
            }
        }
        Err(Error::UserNotFound)
    }

    pub fn change_name(&mut self, name: String, new_name: String) -> Result<(), Error> {
        if self.user_exists(new_name.as_str()) { return Err(Error::UserAlreadyExists) }
        for i in 0..self.users.len() {
            if self.users[i].username == name {
                return Ok(self.users[i].username = new_name)
            }
        }
        Err(Error::UserNotFound)
    }

    pub fn change_pass(&mut self, name: String, new_pass: String) -> Result<(), Error> {
        for i in 0..self.users.len() {
            if self.users[i].username == name {
                return Ok(self.users[i].password = hash_pass(new_pass.as_bytes()))
            }
        }
        Err(Error::UserNotFound)
    }

    pub fn verify(&self, name: String, pass: String) -> bool {
        for i in 0..self.users.len() {
            if self.users[i].username == name {
                return argon2::verify_encoded(self.users[i].password.as_str(), pass.as_bytes()).unwrap()
            }
        }
        false
    }

    fn user_exists(&self, name: &str) -> bool {
        self.users.iter().filter(|&x| x.username.as_str() == name).count() != 0
    }
}

impl User {
    fn new(username: String, password: String) -> Self {
        Self{ username, password }
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

#[test]
fn test() {
    let mut users = Users::new();
    users.add_user("a".to_string(), "a".to_string());
    users.add_user("b".to_string(), "b".to_string());
    users.add_user("c".to_string(), "c".to_string());

    users.add_user("a".to_string(), "i".to_string());
    assert!(users.verify("a".to_string(), "a".to_string()));
    assert!(!users.verify("a".to_string(), "i".to_string()));

    assert!(users.verify("b".to_string(), "b".to_string()));
    users.change_name("b".to_string(), "i".to_string());
    assert!(users.verify("i".to_string(), "b".to_string()));
    assert!(!users.verify("b".to_string(), "b".to_string()));

    assert!(users.verify("c".to_string(), "c".to_string()));
    users.change_pass("c".to_string(), "i".to_string());
    assert!(users.verify("c".to_string(), "i".to_string()));
    assert!(!users.verify("c".to_string(), "c".to_string()));

    users.remove_user("i".to_string());
    assert!(!users.verify("i".to_string(), "b".to_string()));

    users.add_user("d".to_string(), "a".to_string());
    assert!(users.verify("a".to_string(), "a".to_string()));
    assert!(users.verify("d".to_string(), "a".to_string()));
}