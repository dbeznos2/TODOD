use std::io;

fn main() {
    let mut todos: Vec<String> = Vec::new();
    let mut arch_todo: Vec<String> = Vec::new();

    loop {
        for (task, todo) in todos.iter().enumerate() {
            println!("{}. {}", task + 1, todo);
        }

        println!("n: create a new todo");
        println!("a: to see archived todos");
        println!("x: to archive one todo");
        println!("q: to quit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_string();

        match input.as_str() {
            "n" => {
                println!("Enter a new todo");
                let mut new_todo = String::new();
                io::stdin()
                    .read_line(&mut new_todo)
                    .expect("Failed to read line");
                todos.push(new_todo.trim().to_string());
            }
            "a" => {
                println!("Archived TODOs");
                for (archiv, todo_archive) in arch_todo.iter().enumerate() {
                    println!("{}. {}", archiv + 1, todo_archive);
                }
                let mut quit = String::new();
                println!("Press 'q' to quit the archive");
                io::stdin()
                    .read_line(&mut quit)
                    .expect("Error reading input");
                if quit.trim() == "q" {
                    continue; // Exit the loop and the program
                }
            }
            "x" => {
                println!("Enter the number of the todo to archive:");
                let mut archive_input = String::new();
                io::stdin()
                    .read_line(&mut archive_input)
                    .expect("Failed to read line");
                if let Ok(index) = archive_input.trim().parse::<usize>() {
                    if index >= 1 && index <= todos.len() {
                        let archived_todo = todos.remove(index - 1);
                        arch_todo.push(archived_todo);
                        println!("Task {} archived.", index);
                    } else {
                        println!("Invalid task number.");
                    }
                } else {
                    println!("Invalid input. Please enter a valid number.");
                }
            }
            "q" => {
                println!("Exiting the program.");
                return; // Exit the program
            }
            _ => {
                println!("Invalid choice. Please enter 'n', 'a', 'x', or 'q'.");
            }
        }
    }
}
