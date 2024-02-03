use console::Term;
fn main() -> std::io::Result<()>{
    let term = Term::stdout();
    term.clear_screen()?;
    let height = term.size().1 - 1;
    term.move_cursor_to(0, height.into())?;
    Ok(())
}
