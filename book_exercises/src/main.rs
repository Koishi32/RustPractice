mod exercises;
pub mod input_mod;
fn main() {
    choose_option_main_menu();
}

fn choose_option_main_menu(){
    println!("1) Fibonachi");
    println!("2) Pig Latin");
    println!("3) Median and mode");
    println!("4) Company terminal");
    println!("5) Exit");
    let option = input_mod::string_input();
    match option{
        val if val == "1".to_string()=> {
            select_exercise_fibonachi();
            choose_option_main_menu();
        },
        val if val == "2".to_string()=> {
            select_exercise_pig_latin();
            choose_option_main_menu();
        },
        val if val == "3".to_string()=> {
            select_exercise_median_mode();
            choose_option_main_menu();
        },
        val if val =="4".to_string()=> {
            select_exercise_company_terminal();
            choose_option_main_menu();
        },
        val if val =="5".to_string()=> {
            println!("Exiting");
        },
        String{..} =>{
            println!("Non valid option");
            choose_option_main_menu();
        },
    }
}

fn select_exercise_fibonachi(){
    println!("Do Fibonachi: ");
    exercises::fibonachi::count_fibonachi();  
}
fn select_exercise_pig_latin(){
    println!("Do pig latin: ");
    exercises::pig_latin::test_pig_latin();
}
fn select_exercise_median_mode(){
    println!("Do median and mode: ");
    exercises::median_mode::do_median_mode();
}
fn select_exercise_company_terminal(){
    println!("Do company terminal: ");
    exercises::employee_terminal::text_interface();
}
