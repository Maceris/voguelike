mod terminal_util;

fn main() {
    
    terminal_util::clear_screen();

    let size = match crossterm::terminal::size() {
        Ok(value) => value,
        Err(_) => panic!("Cannot obtain terminal size")
    };

    let sample_term = terminal_util::Terminal {
        width: size.0,
        height: size.1
    };
   
    let screen = match terminal_util::create_screen(sample_term) {
        Ok(value) => value,
        Err(e) => panic!("{}", e)
    };

    terminal_util::print_screen(screen);
}
