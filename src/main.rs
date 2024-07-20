mod color;
mod race;
mod cat;

use rand::{Rng, thread_rng};
use cat::{CatInfo, Chat};
use crate::color::ColorType;
use crate::race::Race;

fn main() {

    let _test = CatInfo::new_cat(CatInfo{
        arrived_date: Default::default(),
        name: "",
        age: 0,
        color_type: ColorType::CodeBarre,
        race: Race::Persan,
        weight: 0.0,
        sleep: false,
        health: 0,
        female: false,
    });

    let mut cats = CatInfo::spawn_new_cat(10);

    for cat in &cats {
        println!("{}", cat);
    }

    if let Some(first_cat) = cats.get_mut(0) {
        first_cat.feed();
        first_cat.play();
        first_cat.toggle_sleep();
        first_cat.play();
        first_cat.age();
    }

    if let (Some(first_cat), Some(second_cat)) = (cats.get( thread_rng().gen_range(0..=cats.len())), cats.get(thread_rng().gen_range(0..=cats.len()))) {
        if let Some(kitten) = first_cat.mate(second_cat) {
            println!("\nA new kitten is born! from {} and {}\n{}", first_cat.name, second_cat.name, kitten);
        }
    }
}
