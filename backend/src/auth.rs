use argon2;

#[derive(Debug)]
pub struct Users {
    users: Vec<User>,
}

#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

impl Users {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
        }
    }

    pub fn add_user(&mut self, name: String, pass: String) {
        if !self.user_exists(name.as_str()) {
            self.users.push(User::new(name, hash_pass(pass.as_bytes())))
        }
    }

    pub fn remove_user(&mut self, name: String) {
        for i in 0..self.users.len() {
            if self.users[i].name_matches(name.as_str()) {
                self.users.remove(i);
                return
            }
        }
    }

    pub fn change_name(&mut self, name: String, new_name: String) {
        if self.user_exists(new_name.as_str()) { return }
        for i in 0..self.users.len() {
            if self.users[i].name_matches(name.as_str()) {
                self.users[i].username = new_name;
                return
            }
        }
    }

    pub fn change_pass(&mut self, name: String, new_pass: String) {
        for i in 0..self.users.len() {
            if self.users[i].name_matches(name.as_str()) {
                self.users[i].password = hash_pass(new_pass.as_bytes());
                return
            }
        }
    }

    pub fn login(&self, user: String, pass: String) -> bool {
        for i in 0..self.users.len() {
            if self.users[i].name_matches(user.as_str()) {
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
        Self{
            username,
            password,
        }
    }

    fn name_matches(&self, name: &str) -> bool {
        self.username.as_str() == name
    }
}

fn hash_pass(pass: &[u8]) -> String {
    let salt = b"iuethxks";
    let config = argon2::Config::default();
    argon2::hash_encoded(pass, salt, &config).unwrap()
}