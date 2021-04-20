use std::str::FromStr;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    action: Action,
    arg: String,
}
#[derive(Debug)]
enum Action {
    Add,
    Remove,
    Update,
}
impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Action, Self::Err> {
        match s {
            "Add" => Ok(Action::Add),
            "Remove" => Ok(Action::Remove),
            "Update" => Ok(Action::Update),
            _ => Err(String::from("invalid action")),
        }
    }
}
#[derive(Debug)]
struct TodoItem {
    message: String,
}
#[derive(Debug)]
struct Todo {
    todo_items: Vec<TodoItem>,
}
impl Todo {
    fn add(&mut self, message: String) {
        self.todo_items.push(TodoItem { message: message });
    }
    fn remove(&mut self, id: usize) {
        let mut delete_index: Option<usize> = None;
        for (index, _) in self.todo_items.iter().enumerate() {
            if index == id {
                delete_index = Some(index);
                break;
            }
        }
        if let Some(index) = delete_index {
            self.todo_items.remove(index);
        }
    }
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args);
    let mut todo = Todo {
        todo_items: Vec::new(),
    };
    match args.action {
        Action::Add => todo.add(String::from(args.arg)),
        Action::Remove => todo.remove(1),
        Action::Update => todo.add(String::from(args.arg)),
    }
    todo.add(String::from("hello"));
    println!("{:?}", todo);
}
