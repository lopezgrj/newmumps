mod domain {
    pub mod patient;
    pub mod user;
    pub mod institution;
    pub mod facility;
    pub mod provider;
}

use domain::patient::Patient;

fn main() {
    println!("Hello, world!");
}
