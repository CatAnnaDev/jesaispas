#![allow(dead_code)]
use requestty::{Answer, OnEsc, prompt_one, Question};

use cat::CatInfo;

mod color;
mod race;
mod cat;
mod log_color;

fn main() {
    let mut cats = CatInfo::spawn_new_cat(2);
    for cat in &cats {
        cat.minimal_info()
    }

    loop {
        let x = build_question(&cats);
        if let Ok(x) = x {
            if let Ok(nb) = x.try_into_list_items() {
                if nb.len() > 1 && nb.len() < 3 {
                    let chat1 = cats.iter().find(|s| nb.first().unwrap().text.contains(s.name));
                    let chat2 = cats.iter().find(|s| nb.last().unwrap().text.contains(s.name));

                    if let (Some(first_cat), Some(second_cat)) = (chat1, chat2) {
                        if let Some(kitten) = first_cat.mate(second_cat) {
                            info!("\nA new kitten is born! from {} and {}\n{}", first_cat.name, second_cat.name, kitten);
                            cats.push(kitten);
                        }
                    }

                } else if nb.len() == 1 {
                    if let Some(chat1) = cats.iter_mut().find(|s| nb.first().unwrap().text.contains(s.name)) {
                        info!("{chat1}");
                        if let Ok(rep) = prompt_one(ask_cat()) {
                            match rep.as_list_item().unwrap().text.as_str() {
                                "Feed" => chat1.feed(),
                                "Sleep" => chat1.toggle_sleep(),
                                "Play" => chat1.play(),
                                "Age" => chat1.age(),
                                _ => {
                                    warn!("No cat select")
                                }
                            }
                        }
                    }else {
                        match nb.first().unwrap().text.as_str() {
                            "Stats" => {
                                for cat in &cats {
                                    info!("{}", cat);
                                }
                            }
                            "Minimal Stats" => {
                                for cat in &cats {
                                    cat.minimal_info()
                                }
                            }
                            "Feed All" => {
                                for cat in &mut cats {
                                    cat.feed()
                                }
                            }
                            _ => {}
                        }
                    }
                } else {
                    warn!("select 1 or 2 cats not more !");
                }
            }
        }
    }
}

fn ask_cat() -> Question<'static> {
    Question::select("Interaction").message("What do you want to do?")
        .choice("Feed")
        .choice("Sleep")
        .choice("Play")
        .choice("Age")
        .build()
}


fn build_question(find: &[CatInfo]) -> requestty::Result<Answer> {
    let multi_select = Question::multi_select("Cat Interaction")
        .message("What's Cat do you want?")
        .choices(find.iter().map(|s| s.name ).collect::<Vec<_>>())
        .choice("Stats")
        .choice("Minimal Stats")
        .choice("Feed All")
        .on_esc(OnEsc::Terminate)
        .page_size(20)
        .should_loop(false)
        .build();

    prompt_one(multi_select)
}