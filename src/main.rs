mod features;
mod io_utils;
use crate::features::panel::domain::models::MineBoxPanel;
use io_utils::stdin::Prompt;

const MAIN_MENU: &str = "Bienvenido al buscaminas:
1: Jugar
2: Salir";

const DIFFICULTY_MENU: &str = "Seleccione la dificultad
1: Facil
2: Normal
3: Dificil";

const PROMPT_SYMBOL: &str = "->";

fn main() {
    let prompt: Prompt = Prompt::new(PROMPT_SYMBOL);
    loop {
        match prompt.input_as::<i32>(MAIN_MENU) {
            1 => launch(),
            2 => {
                println!("Thanks for playing");
                break;
            }
            _ => println!("Incorrect value"),
        }
    }
}

fn launch() {
    let prompt = Prompt::new(PROMPT_SYMBOL);
    loop {
        match prompt.input_as::<i32>(DIFFICULTY_MENU) {
            1 => {
                start_game(16);
                break;
            }
            2 => {
                start_game(32);
                break;
            }
            3 => {
                start_game(48);
                break;
            }
            _ => println!("Incorrect value"),
        }
    }
}

/// Starts game with specified panel size
fn start_game(size: usize) {
    let panel = MineBoxPanel::new(size);
    println!("{}", panel);
}
