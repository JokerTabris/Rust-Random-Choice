use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use colored::*;

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}

fn print_with_delay(message: &str, repetitions: usize, delay_ms: u64) {
    for _ in 0..repetitions {
        println!("{}", message);
        sleep(Duration::from_millis(delay_ms));
    }
}

fn main() {
    let mut choices = Vec::new();

    // 最初の選択肢を入力
    let first_choice_prompt = "\n１つ目の選択肢を入力してください。";
    let first_choice = get_user_input(first_choice_prompt);
    choices.push(first_choice);

    // 追加の選択肢を入力するかどうか
    loop {
        let prompt = "\n追加の選択肢を入力しますか？ (y/n): ";
        let user_input = get_user_input(prompt);

        if user_input.to_ascii_lowercase() == "n" {
            break;
        }

        let index = choices.len() + 1;
        let new_choice_prompt = format!("\n{}つ目の選択肢を入力してください。: ", index);
        let new_choice = get_user_input(&new_choice_prompt);
        choices.push(new_choice);
    }

    // ランダムで選ばれた選択肢を表示
    let mut rnd = rand::thread_rng();
    let index = rnd.gen_range(0..choices.len());

    println!("");
    print_with_delay("↓", 5, 100);
    print_with_delay("選ばれた選択肢は…", 1, 100);
    print_with_delay("↓", 5, 100);

    let chosen_word = &choices[index];
    let colored_output = format!("{}", chosen_word.red());

    println!("\n{}\n", colored_output);
}
