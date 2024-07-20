use std::fmt::{Display, Formatter};
use rand::{random, Rng, thread_rng};
use chrono::{NaiveDate, Local, Duration, Datelike};
use crate::color::ColorType;
use crate::race::Race;

#[derive(Default)]
pub struct CatInfo {
    pub arrived_date: NaiveDate,
    pub name: &'static str,
    pub age: u8,
    pub color_type: ColorType,
    pub race: Race,
    pub weight: f32,
    pub sleep: bool,
    pub health: u8,
    pub female: bool,
}

fn generate_random_date_in_range(start_date: NaiveDate, end_date: NaiveDate) -> NaiveDate {
    let mut rng = rand::thread_rng();
    let days_range = (end_date - start_date).num_days();
    let random_days = rng.gen_range(0..=days_range);
    start_date + Duration::days(random_days)
}

fn generate_dates() -> (NaiveDate, NaiveDate) {
    let today = Local::now().naive_utc().date();
    let earliest_birth_date = NaiveDate::from_ymd_opt(2010, 1, 1).unwrap();
    let earliest_arrival_date = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();

    let birth_date = generate_random_date_in_range(earliest_birth_date, today);

    let arrival_start_date = std::cmp::max(earliest_arrival_date, birth_date);
    let arrival_date = generate_random_date_in_range(arrival_start_date, today);

    (birth_date, arrival_date)
}

fn calculate_age(birth_date: NaiveDate, current_date: NaiveDate) -> u8 {
    let mut age = current_date.year() - birth_date.year();
    if (current_date.month() < birth_date.month()) || (current_date.month() == birth_date.month() && current_date.day() < birth_date.day()) {
        age -= 1;
    }
    age as u8
}

pub trait Chat {
    type Output;
    fn default_cat() -> Self::Output;
    fn new_cat(cat_info: CatInfo) -> Self::Output;
    fn spawn_new_cat(nb_cat: u8) -> Vec<Self::Output>;
    fn feed(&mut self);
    fn play(&mut self);
    fn toggle_sleep(&mut self);
    fn age(&mut self);
    fn mate(&self, other: &Self) -> Option<Self::Output>;
}


impl Chat for CatInfo {
    type Output = CatInfo;

    fn default_cat() -> Self::Output{
       CatInfo{..Default::default()}
    }

    fn new_cat(cat_info: CatInfo) -> Self::Output {
        cat_info
    }

    fn spawn_new_cat(nb_cat: u8) -> Vec<Self::Output> {
        let mut cat_vec = Vec::new();
        const NAMES: [&str; 30] = [
            "Fluffy", "Mittens", "Whiskers", "Shadow", "Smokey",
            "Tiger", "Oreo", "Luna", "Simba", "Ginger",
            "Bella", "Chloe", "Coco", "Daisy", "Felix",
            "Jack", "Jasper", "Leo", "Loki", "Lucky",
            "Max", "Milo", "Nala", "Chamallow", "Oscar",
            "Peanut", "Pepper", "Rocky", "Surf", "Zoe"
        ];

        for _ in 0..nb_cat {
            let color: ColorType = random();
            let race: Race = random();
            let name = NAMES[thread_rng().gen_range(0..NAMES.len())];
            let sleep = thread_rng().gen_bool(1.0 / 3.0);
            let health = random::<u8>();
            let female = thread_rng().gen_bool(1.0 / 3.0);
            let (birth_date, arrival_date) = generate_dates();
            cat_vec.push(Self::new_cat(CatInfo{
                arrived_date: arrival_date,
                name,
                age: calculate_age(birth_date, Local::now().naive_utc().date()),
                color_type: color,
                race,
                weight: thread_rng().gen_range(3.0..6.0),
                sleep,
                health,
                female
            }));
        }
        cat_vec
    }

    fn feed(&mut self) {
        self.weight += 0.1;
        self.health += 5;
        println!("{} a été nourri. Nouveau poids: {:.1} kg, Santé: {}", self.name, self.weight, self.health);
    }

    fn play(&mut self) {
        if !self.sleep {
            self.weight -= 0.05;
            self.health += 2;
            println!("{} a joué. Nouveau poids: {:.1} kg, Santé: {}", self.name, self.weight, self.health);
        } else {
            println!("{} dort et ne peut pas jouer.", self.name);
        }
    }

    fn toggle_sleep(&mut self) {
        self.sleep = !self.sleep;
        if self.sleep {
            self.health += 10;
            println!("{} fait maintenant dodo. Santé: {}", self.name, self.health);
        } else {
            println!("{} est maintenant réveillé.", self.name);
        }
    }

    fn age(&mut self) {
        self.age += 1;
        self.health -= 5;
        println!("{} a vieilli. Nouvel âge: {}, Santé: {}", self.name, self.age, self.health);
    }

    fn mate(&self, other: &Self) -> Option<Self::Output> {

        if self.female == other.female && self.sleep || other.sleep {
            println!("\nCan't mate {} with {}\nbecause:", self.name, other.name);
            if self.female == other.female{
                println!("- Same Sexe");
            }
            if self.sleep{
                println!("- {} sleep", self.name)
            }
            if other.sleep{
                println!("- {} sleep", other.name)
            }

            return None;
        }

        let name = format!("{}{}", &self.name[0..self.name.len() / 2], &other.name[other.name.len() / 2..]);

        let color = if random() { self.color_type } else { other.color_type };

        let race = if random() { self.race } else { other.race };

        let age = 0;
        let weight = 1.0;
        let female = random();
        Some(CatInfo {
            arrived_date: Local::now().naive_utc().date(),
            name: Box::leak(name.into_boxed_str()),
            age,
            color_type: color,
            race,
            weight,
            sleep: false,
            health: 100,
            female
        })
    }
}

impl Display for CatInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\n- Age: {} an(s)\n- Color: {}\n- Race: {}\n- Weight: {:.2} kg\n- Sleep: {}\n- Health: {}\n- Sexe: {}\n- Entrance Date: {}\n",
            self.name, self.age, self.color_type, self.race, self.weight, if self.sleep { "Yes" } else { "No" }, self.health, if self.female { "Female" } else {"Male"}, self.arrived_date
        )
    }
}
