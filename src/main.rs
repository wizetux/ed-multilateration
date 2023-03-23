use serde::Deserialize;
use reqwest::Error;
use std::collections::HashSet;
use std::cmp::Eq;
use futures::join;
use std::io;

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
    let mut system_name = String::new();
    let mut min_radius = String::new();
    let mut max_radius = String::new();
    println!("System Name:");
    io::stdin()
        .read_line(&mut system_name)
        .expect("Failed to read line");

    println!("Min Radius:");
    io::stdin()
        .read_line(&mut min_radius)
        .expect("Failed to read line");

    println!("Max Radius:");
    io::stdin()
        .read_line(&mut max_radius)
        .expect("Failed to read line");

    (system_name.trim().to_string(), min_radius.trim().to_string(), max_radius.trim().to_string())
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
