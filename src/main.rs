use serde::Deserialize;
use reqwest::Error;
use std::collections::HashSet;
use std::cmp::Eq;
use futures::join;
use std::io::{self, Write};

#[derive(Deserialize, Debug)]
struct System {
    name: String,
}

impl PartialEq for System {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for System {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let system_1 = get_systems_and_range();
    let system_2 = get_systems_and_range();
    let system_3 = get_systems_and_range();
    let systems = join!(
        get_bodies_in_sphere(system_1.0, system_1.1, system_1.2),
        get_bodies_in_sphere(system_2.0, system_2.1, system_2.2),
        get_bodies_in_sphere(system_3.0, system_3.1, system_3.2));
    let system_1_names: HashSet<String> = systems.0.unwrap().into_iter().map(|system| { system.name } ).collect();
    let system_2_names: HashSet<String> = systems.1.unwrap().into_iter().map(|system| { system.name } ).collect();
    let system_3_names: HashSet<String> = systems.2.unwrap().into_iter().map(|system| { system.name } ).collect();
    let intersect: HashSet<_> = system_1_names.intersection(&system_2_names).cloned().collect();
    let final_intersect = intersect.intersection(&system_3_names);
    println!("{:?}", final_intersect);

    println!("Press return key to exit");
    let mut pause = String::new();
    io::stdin().read_line(&mut pause).expect("");
    Ok(())
}

fn get_systems_and_range() -> (String, String, String) {
    let system_name = get_input_from_user("System Name:".to_string(), "Failed to read line".to_string(), "Please enter a valid system name.".to_string());
    let min_radius = get_input_from_user("Min Radius:".to_string(), "Failed to read line".to_string(), "Please enter a valid min radius".to_string());
    let max_radius = get_input_from_user("Max Radius:".to_string(), "Failed to read line".to_string(), "Please enter a valid max radius".to_string());
    (system_name, min_radius, max_radius)
}

fn get_input_from_user(input_string: String, error_string: String, empty_string: String) -> String {
    let mut input = String::new();
    while input.is_empty() {
        io::stdout().write_all(&input_string.as_bytes()).expect("Failed to write input string");
        io::stdout().flush().expect("failed");
        io::stdin()
            .read_line(&mut input)
            .expect(&error_string);

        input = input.trim().to_string();
        if input.is_empty() {
            println!("{}", empty_string)
        }
    }

    return input;
}
async fn get_bodies_in_sphere(system_name: String, min_radius: String, radius: String) -> Result<Vec<System>, Error> {
    let client = reqwest::Client::new();
    let response = client.get("https://www.edsm.net/api-v1/sphere-systems")
        .query(&[("systemName", system_name), ("minRadius", min_radius), ("radius", radius)])
        .send()
        .await?; 

    let systems: Vec<System> = response.json::<Vec<System>>().await?;
    Ok(systems)
}
