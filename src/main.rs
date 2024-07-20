mod color;
mod race;
mod cat;

use rand::{Rng, thread_rng};
use cat::{CatInfo, Chat};

fn main() {

    let _test = CatInfo::default_cat();

    let mut cats = CatInfo::spawn_new_cat(10);

    for cat in &cats {
        println!("{}", cat);
    }

    if let Some(first_cat) = cats.get_mut(0) {
        first_cat.feed();
        first_cat.play();
        first_cat.toggle_sleep();
        first_cat.age();
    }

    let chat1 = cats.get( thread_rng().gen_range(0..cats.len()));
    let chat2 = cats.get( thread_rng().gen_range(0..cats.len()));

    if let (Some(first_cat), Some(second_cat)) = (chat1, chat2) {
        if let Some(kitten) = first_cat.mate(second_cat) {
            println!("\nA new kitten is born! from {} and {}\n{}", first_cat.name, second_cat.name, kitten);
            cats.push(kitten);
        }
    }
}
