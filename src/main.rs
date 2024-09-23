use console::{Key, Term};
use std::io;

const OPTS: &[(&str, &str)] = &[("'a'", "to add"), ("'d'", "to delete"), ("'Esc'", "to exit")];
//TODO add handler
//TODO database integration
fn main() -> io::Result<()> {
    let _ = run_app();
    Ok(())
}

fn bold_str(text: &str) -> String {
    format!("\x1b[1m{text}\x1b[0m")
}

fn render_list(items: &Vec<&str>, term: &Term) -> io::Result<()> {
    for (i, item) in items.iter().enumerate() {
        term.write_line(format!("[{i}] {item}").as_str())?;
    }
    Ok(())
}

fn handle_delete(item_list: &mut Vec<&str>, term: &Term) -> io::Result<()> {
    loop {
        term.write_line("\nEnter item number to delete:")?;
        let key = term.read_key()?;

        match key {
            Key::Char(c) if c.is_digit(10) => {
                let num = c.to_digit(10).unwrap();
                let num = usize::try_from(num).unwrap();
                if num < item_list.len() {
                    let _ = item_list.remove(num);
                    break;
                } else {
                    term.write_line("\nNumber out of bounds. Please try again.")?;
                }
            }
            _ => {
                term.write_line("\nInvalid input. Please enter a number.")?;
            }
        }
    }
    Ok(())
}

fn render_and_prompt(
    item_list: &Vec<&str>,
    term: &Term,
    app_title: &String,
    app_opts: &String,
) -> io::Result<()> {
    term.clear_screen()?;
    term.write_line(app_title)?;
    render_list(item_list, term)?;
    term.write_line(format!("\nPress any key. {app_opts}").as_str())?;
    Ok(())
}

fn run_app() -> io::Result<()> {
    let mut item_list = vec!["foo", "bar", "baz", "qux"];
    let term = Term::stdout();
    let joined_opts = OPTS
        .iter()
        .map(|&(key, description)| format!("{} {}", bold_str(key), description))
        .collect::<Vec<String>>()
        .join(", ");
    let app_title = bold_str("Rust Todo\n");
    render_and_prompt(&item_list, &term, &app_title, &joined_opts)?;
    loop {
        let key = term.read_key()?;
        match key {
            Key::Escape => {
                break;
            }
            Key::Char('d') => {
                // Pass a mutable reference of `item_list` to `handle_delete`
                handle_delete(&mut item_list, &term)?;
            }
            _ => {}
        }
        render_and_prompt(&item_list, &term, &app_title, &joined_opts)?;
    }
    Ok(())
}
