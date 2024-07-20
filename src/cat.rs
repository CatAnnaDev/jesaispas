use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

use chrono::{Datelike, Duration, Local, NaiveDate};
use rand::{random, Rng, thread_rng};

use crate::{bool_state, interaction, warn};
use crate::cat::Gender::{Female, Male};
use crate::color::ColorType;
use crate::race::Race;

const GENDER_MALE: [&str; 21] = [
    "Mittens", "Whiskers", "Shadow", "Smokey",
    "Tiger", "Oreo", "Simba", "Ginger", "Felix",
    "Jack", "Jasper", "Leo", "Loki", "Lucky",
    "Max", "Milo", "Chamallow", "Oscar",
    "Peanut", "Rocky", "Surf",
];

const GENDER_FEMALE: [&str; 17] = [
    "Fluffy", "Luna", "Suzie", "Princess", "Marelle",
    "Bella", "Chloe", "Coco", "Daisy", "Nyx", "Dinah",
    "Nala", "Pepper", "Zoe", "Callie", "Angel", "Kitty",
];

#[derive(Eq, PartialEq, Default)]
pub enum Gender {
    #[default]
    Female,
    Male,
}


impl Display for Gender{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Female => "\x1B[35mFemale\x1B[0m",
            Male => "\x1B[36mMale\x1B[0m"
        })
    }
}

impl Gender {
    pub fn get_random_gender() -> Gender {
        if random() { Female } else { Male }
    }
}

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
    pub gender: Gender,
}

fn generate_random_date_in_range(start_date: NaiveDate, end_date: NaiveDate) -> NaiveDate {
    let mut rng = thread_rng();
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

impl CatInfo {
    fn default_cat() -> Self {
        CatInfo { ..Default::default() }
    }

    fn new_cat(cat_info: CatInfo) -> Self {
        cat_info
    }

    pub(crate) fn spawn_new_cat(nb_cat: u8) -> Vec<Self> {
        let mut cat_vec = Vec::new();


        for _ in 0..nb_cat {
            let color: ColorType = random();
            let race: Race = random();
            let sleep = random();
            let health = random::<u8>();
            let gender = Gender::get_random_gender();
            let (birth_date, arrival_date) = generate_dates();
            cat_vec.push(Self::new_cat(CatInfo {
                arrived_date: arrival_date,
                name: match gender {
                    Male => {GENDER_MALE[thread_rng().gen_range(0..GENDER_MALE.len())]}
                    Female => {GENDER_FEMALE[thread_rng().gen_range(0..GENDER_FEMALE.len())]}
                },
                age: calculate_age(birth_date, Local::now().naive_utc().date()),
                color_type: color,
                race,
                weight: thread_rng().gen_range(3.0..6.0),
                sleep,
                health,
                gender,
            }));
        }
        cat_vec
    }

    pub(crate) fn feed(&mut self) {
        self.weight += 0.1;
        self.health += 5;
        interaction!("{} a été nourri. Nouveau poids: {:.1} kg, Santé: {}", self.name, self.weight, self.health);
    }

    pub(crate) fn play(&mut self) {
        if !self.sleep {
            self.weight -= 0.05;
            self.health += 2;
            interaction!("{} a joué. Nouveau poids: {:.1} kg, Santé: {}", self.name, self.weight, self.health);
        } else {
            interaction!("{} dort et ne peut pas jouer.", self.name);
        }
    }

    pub(crate) fn toggle_sleep(&mut self) {
        self.sleep = !self.sleep;
        if self.sleep {
            self.health += 10;
            interaction!("{} fait maintenant dodo. Santé: {}", self.name, self.health);
        } else {
            interaction!("{} est maintenant réveillé.", self.name);
        }
    }

    pub(crate) fn age(&mut self) {
        self.age += 1;
        self.health -= 5;
        interaction!("{} a vieilli. Nouvel âge: {}, Santé: {}", self.name, self.age, self.health);
    }

    pub(crate) fn mate(&self, other: &Self) -> Option<Self> {
        if self.gender == other.gender && self.sleep || other.sleep {
            warn!("\nCan't mate {} with {}\nbecause:", self.name, other.name);
            if self.gender == other.gender {
                warn!("- Same Sexe");
            }
            if self.sleep {
                warn!("- {} sleep", self.name)
            }
            if other.sleep {
                warn!("- {} sleep", other.name)
            }

            return None;
        }

        let name = format!("{}{}", &self.name[0..self.name.len() / 2], &other.name[other.name.len() / 2..]);

        let color = if random() { self.color_type } else { other.color_type };

        let race = if random() { self.race } else { other.race };

        Some(CatInfo {
            arrived_date: Local::now().naive_utc().date(),
            name: Box::leak(name.into_boxed_str()),
            age: 1,
            color_type: color,
            race,
            weight: 1.0,
            sleep: false,
            health: 100,
            gender: Gender::get_random_gender(),
        })
    }
}

impl Display for CatInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\n- Age: {} an(s)\n- Color: {}\n- Race: {}\n- Weight: {:.2} kg\n- Sleep: {}\x1B[32m\n- Health: {}\n- Sexe: {}\x1B[32m\n- Entrance Date: {}\n",
            self.name, self.age, self.color_type, self.race, self.weight, bool_state!("31mYES", "32mNO", self.sleep), self.health, self.gender, self.arrived_date
        )
    }
}
