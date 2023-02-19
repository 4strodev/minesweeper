mod io_utils;
mod mine;
mod panel;

const PROMPT: &str = "-> ";
const MAIN_MENU: &str = "Bienvenido al buscaminas:
1: Jugar
2: Salir
";

fn main() {
    let option: i32 = io_utils::stdin::input_as(MAIN_MENU);
    println!("{}", option);
}
