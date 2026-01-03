pub struct Store {
    conn : Connection
}

impl Default for Store {
    fn default() -> Self {
        
    }
}

impl Store {
    pub fn create_user(&self) {
        print!("create user called");
    }
    pub fn create_website(&self) -> String {
        return String::from("123456789");
    }
}