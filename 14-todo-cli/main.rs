// 14 - todo CLI: เก็บงานใน todos.txt บรรทัดละ "0|text" (0=ค้าง, 1=เสร็จ)
use std::env;
use std::fs;
use std::path::Path;

const FILE: &str = "todos.txt";

fn load() -> Vec<(bool, String)> {
    if !Path::new(FILE).exists() {
        return Vec::new();
    }
    fs::read_to_string(FILE)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| {
            let (flag, text) = line.split_once('|')?; // ตัดที่ | ตัวแรก
            Some((flag == "1", text.to_string()))
        })
        .collect()
}

fn save(todos: &[(bool, String)]) {
    let body: Vec<String> = todos
        .iter()
        .map(|(done, text)| format!("{}|{text}", if *done { "1" } else { "0" }))
        .collect();
    let out = if body.is_empty() {
        String::new()
    } else {
        body.join("\n") + "\n"
    };
    fs::write(FILE, out).expect("write todos.txt");
}

fn list() {
    let todos = load();
    if todos.is_empty() {
        println!("(empty) add one with: ./main add \"task\"");
        return;
    }
    for (i, (done, text)) in todos.iter().enumerate() {
        let mark = if *done { "x" } else { " " };
        println!("{}. [{mark}] {text}", i + 1);
    }
}

// แกะเลขลำดับจาก arg (1-based) กัน input มั่ว
fn parse_index(arg: Option<&String>, len: usize) -> Option<usize> {
    let n: usize = arg?.parse().ok()?;
    if n >= 1 && n <= len {
        Some(n - 1)
    } else {
        None
    }
}

fn usage() {
    println!("usage:");
    println!("  ./main add \"task\"   add todo");
    println!("  ./main list          list todos");
    println!("  ./main done <n>      mark n as done");
    println!("  ./main del <n>       delete n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            if args.len() < 3 {
                eprintln!("missing task: ./main add \"task\"");
                return;
            }
            let mut todos = load();
            todos.push((false, args[2..].join(" ")));
            save(&todos);
            println!("added #{:?}", todos.len());
        }
        Some("list") => list(),
        Some("done") => {
            let mut todos = load();
            match parse_index(args.get(2), todos.len()) {
                Some(i) => {
                    todos[i].0 = true;
                    save(&todos);
                    println!("done #{}", i + 1);
                }
                None => eprintln!("bad index: ./main done <1..={}>", todos.len()),
            }
        }
        Some("del") => {
            let mut todos = load();
            match parse_index(args.get(2), todos.len()) {
                Some(i) => {
                    let (_, text) = todos.remove(i);
                    save(&todos);
                    println!("deleted: {text}");
                }
                None => eprintln!("bad index: ./main del <1..={}>", todos.len()),
            }
        }
        _ => usage(),
    }
}
