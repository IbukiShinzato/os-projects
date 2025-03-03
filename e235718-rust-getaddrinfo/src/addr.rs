#[derive(Debug)]
pub struct Addr {
    pub name: String,
    pub ipv4: String,
    pub ipv6: String,
}

impl Addr {
    pub fn new(name: String) -> Self {
        Addr {
            name: name,
            ipv4: String::from(""),
            ipv6: String::from(""),
        }
    }
}
