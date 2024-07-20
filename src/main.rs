mod color;
mod race;
mod cat;

use requestty::{Answer, OnEsc, prompt_one, Question};
use cat::{CatInfo, Chat};

fn main() {

    let mut cats = CatInfo::spawn_new_cat(10);

    loop {
        let x = build_question(&cats);
        if let Ok(x) = x {
            if let Ok(nb) = x.try_into_list_items() {
                if nb.len() > 1 {
                    let chat1 = cats.iter().find(|s| nb.first().unwrap().text.contains(s.name));
                    let chat2 = cats.iter().find(|s| nb.last().unwrap().text.contains(s.name));

                    if let (Some(first_cat), Some(second_cat)) = (chat1, chat2) {
                        if let Some(kitten) = first_cat.mate(second_cat) {
                            println!("\nA new kitten is born! from {} and {}\n{}", first_cat.name, second_cat.name, kitten);
                            cats.push(kitten);
                        }
                    }
                } else if nb.len() == 1 {
                    let questions = Question::select("Interaction").message("What do you want to do?").choices::<Vec<String>, _>(vec![
                        "Feed".into(),
                        "Sleep".into(),
                        "Play".into(),
                        "Age".into(),
                        "Stats".into(),
                    ]).build();

                    if let Some(chat1) = cats.iter_mut().find(|s| nb.first().unwrap().text.contains(s.name)) {
                        println!("{chat1}");
                        if let Ok(rep) = prompt_one(questions) {
                            match rep.as_list_item().unwrap().text.as_str() {
                                "Feed" => chat1.feed(),
                                "Sleep" => chat1.toggle_sleep(),
                                "Play" => chat1.play(),
                                "Age" => chat1.age(),
                                "Stats" => {
                                    for cat in &cats {
                                        println!("{}", cat);
                                    }
                                }
                                _ => {
                                    println!("No cat select")
                                }
                            }
                        }
                    }
                } else {
                    println!("select 1 or 2 cats not more !");
                }
            }
        }
    }

}


fn build_question(find: &Vec<CatInfo>) -> requestty::Result<Answer> {
    let multi_select = Question::multi_select("Cat Interaction")
        .message("What's Cat do you want?")
        .choices(find.iter().map(|s| { format!("{}", s.name) }).collect::<Vec<_>>(), )
        .on_esc(OnEsc::Terminate)
        .page_size(20)
        .should_loop(false)
        .build();

    prompt_one(multi_select)
}