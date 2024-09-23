mod database;
use console::{Key, Term};
use database::TodoList;
use dialoguer::Input;
use std::io;

const OPTS: &[(&str, &str)] = &[
    ("'a'", "to add"),
    ("'d'", "to delete"),
    ("'Esc'", "to exit"),
];
//TODO database integration
fn main() -> io::Result<()> {
    let _ = run_app();
    Ok(())
}

fn bold_str(text: &str) -> String {
    format!("\x1b[1m{text}\x1b[0m")
}

fn italic_str(text: &str) -> String {
    format!("\x1b[3m{text}\x1b[0m")
}

fn render_list(items: &Vec<String>, term: &Term) -> io::Result<()> {
    if items.len() > 0 {
        for (i, item) in items.iter().enumerate() {
            term.write_line(format!("[{i}] {item}").as_str())?;
        }
    } else {
        term.write_line(italic_str("No items in the list. Please add one.").as_str())?;
    }
    Ok(())
}

fn handle_delete(item_list: &mut Vec<String>, db: &TodoList) -> io::Result<()> {
    let index: usize = Input::new()
        .with_prompt("\nEnter item number to delete")
        .validate_with(|input: &usize| -> Result<(), &str> {
            if *input < item_list.len() {
                Ok(())
            } else {
                Err("\nInvalid input. Please enter a number within the range.")
            }
        })
        .interact()
        .unwrap();
    let _ = db.delete_todo(index.try_into().unwrap());
    item_list.remove(index);
    Ok(())
}

fn handle_add(item_list: &mut Vec<String>, db: &TodoList) -> io::Result<()> {
    let item: String = Input::new()
        .with_prompt("\nEnter a new item")
        .interact_text()
        .unwrap();
    let _ = db.add_todo(item_list.len().try_into().unwrap(), &item);
    item_list.push(item);
    Ok(())
}

fn render_and_prompt(
    item_list: &Vec<String>,
    term: &Term,
    app_title: &String,
    app_opts: String,
) -> io::Result<()> {
    term.clear_screen()?;
    term.write_line(app_title)?;
    render_list(item_list, term)?;
    term.write_line(format!("\nPress any key: {app_opts}").as_str())?;
    Ok(())
}

fn gen_opts(empty_list: bool) -> String {
    OPTS.iter()
        .filter(|&&(key, _)| !(empty_list && key == "'d'"))
        .map(|&(key, description)| format!("{} {}", bold_str(key), description))
        .collect::<Vec<String>>()
        .join(", ")
}

fn run_app() -> io::Result<()> {
    let todo_list = TodoList::new("todos.db").unwrap();
    let mut item_list: Vec<String> = vec![];
    for item in todo_list.get_all_todos().unwrap() {
        item_list.push(item.item);
    }
    let term = Term::stdout();
    let app_title = bold_str("Rust Todo\n");
    render_and_prompt(
        &item_list,
        &term,
        &app_title,
        gen_opts(item_list.is_empty()),
    )?;
    loop {
        let key = term.read_key()?;
        match key {
            Key::Escape => {
                break;
            }
            Key::Char('a') => {
                handle_add(&mut item_list, &todo_list)?;
            }
            Key::Char('d') => {
                // Pass a mutable reference of `item_list` to `handle_delete`
                if item_list.len() > 0 {
                    handle_delete(&mut item_list, &todo_list)?;
                }
            }
            _ => {}
        }
        render_and_prompt(
            &item_list,
            &term,
            &app_title,
            gen_opts(item_list.is_empty()),
        )?;
    }
    Ok(())
}
