// Implements a CLI based todo list app that stores todo items onto JSON files.
use std::io;
use std::fs;
use std::process;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn get_task_from_user() -> String {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line.");

    user_input.trim().to_string()
}

fn add_task(todo_list: &mut Vec<String>) {
    println!("You want to add a new task.");
    println!("Enter the new task:");
    let new_task = get_task_from_user();
    todo_list.push(new_task);
}

fn remove_task(todo_list: &mut Vec<String>) {
    println!("You want to remove an old task.");
    let old_task = get_task_from_user();
    todo_list.retain(|s| s != &old_task);
}

fn list_all_tasks(todo_list: &mut Vec<String>) {
    println!("You want to list all tasks.");
    for (index, item) in todo_list.iter().enumerate() {
        println!("{}. {}", index, item);
    }
}

fn load_file() -> Vec<String> {
    let path = "output.json";
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return vec![],
        Err(e) => panic!("Unable to read file: {e:?}"),
    };
    serde_json::from_str(&data).expect("Unable to parse file.")
}

fn save(todo_list: &mut Vec<String>) -> std::io::Result<()> {
    println!("Saving...");
    let file = OpenOptions::new().create(true).write(true).truncate(true).open("output.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &todo_list)?;
    writer.flush()?;
    Ok(())
}

fn main() {

    let mut todo_list = load_file();

    loop {
        println!("Choose an option:
                    1. Add new task.
                    2. Remove old task.
                    3. List all available tasks.
                    4. Save and exit.");

        let mut user_choise = String::new();
        io::stdin().read_line(&mut user_choise).expect("Failed to read line.");
        let option = user_choise.trim();

        match option {
            "1" => add_task(&mut todo_list),
            "2" => remove_task(&mut todo_list),
            "3" => list_all_tasks(&mut todo_list),
            "4" => { match save(&mut todo_list) {
                        Ok(()) => { process::exit(1); }
                        Err(e) => { println!("Error: {}", e); } 
                     }
                    }
            _ => println!("You entered the wrong option.")
        }
    }
}
