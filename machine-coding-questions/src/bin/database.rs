use std::collections::HashMap;
use std::io::{self, Write};

//
// ===== MODEL =====
//

#[derive(Clone, Debug)]
pub struct Row {
    pub values: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
}

#[derive(Clone, Debug)]
pub struct Database {
    pub name: String,
    pub tables: HashMap<String, Table>,
}

//
// ===== STORAGE TRAIT (DIP) =====
//

trait Storage {
    fn create_database(&mut self, name: &str);
    fn use_database(&mut self, name: &str) -> Result<(), String>;
    fn create_table(&mut self, name: &str, columns: Vec<String>) -> Result<(), String>;
    fn insert_row(&mut self, table: &str, values: Vec<String>) -> Result<(), String>;
    fn select_all(&self, table: &str) -> Result<Vec<Row>, String>;
    fn get_columns(&self, table: &str) -> Result<Vec<String>, String>;
}

//
// ===== IN-MEMORY STORAGE =====
//

struct InMemoryStorage {
    databases: HashMap<String, Database>,
    current_db: Option<String>,
}

impl InMemoryStorage {
    fn new() -> Self {
        Self {
            databases: HashMap::new(),
            current_db: None,
        }
    }

    fn current_db_mut(&mut self) -> Result<&mut Database, String> {
        let name = self.current_db.clone().ok_or("No database selected")?;
        self.databases
            .get_mut(&name)
            .ok_or("Database not found".to_string())
    }

    fn current_db_ref(&self) -> Result<&Database, String> {
        let name = self.current_db.clone().ok_or("No database selected")?;
        self.databases
            .get(&name)
            .ok_or("Database not found".to_string())
    }
}

impl Storage for InMemoryStorage {
    fn create_database(&mut self, name: &str) {
        self.databases.insert(
            name.to_string(),
            Database {
                name: name.to_string(),
                tables: HashMap::new(),
            },
        );
    }

    fn use_database(&mut self, name: &str) -> Result<(), String> {
        if self.databases.contains_key(name) {
            self.current_db = Some(name.to_string());
            Ok(())
        } else {
            Err("Database does not exist".into())
        }
    }

    fn create_table(&mut self, name: &str, columns: Vec<String>) -> Result<(), String> {
        let db = self.current_db_mut()?;
        db.tables.insert(
            name.to_string(),
            Table {
                name: name.to_string(),
                columns,
                rows: vec![],
            },
        );
        Ok(())
    }

    fn insert_row(&mut self, table: &str, values: Vec<String>) -> Result<(), String> {
        let db = self.current_db_mut()?;
        let tbl = db.tables.get_mut(table).ok_or("Table not found")?;

        if tbl.columns.len() != values.len() {
            return Err("Column count mismatch".into());
        }

        tbl.rows.push(Row { values });
        Ok(())
    }

    fn select_all(&self, table: &str) -> Result<Vec<Row>, String> {
        let db = self.current_db_ref()?;
        let tbl = db.tables.get(table).ok_or("Table not found")?;
        Ok(tbl.rows.clone())
    }

    fn get_columns(&self, table: &str) -> Result<Vec<String>, String> {
        let db = self.current_db_ref()?;
        let tbl = db.tables.get(table).ok_or("Table not found")?;
        Ok(tbl.columns.clone())
    }
}

//
// ===== COMMAND =====
//

enum Command {
    CreateDatabase(String),
    UseDatabase(String),
    CreateTable { name: String, columns: Vec<String> },
    Insert { table: String, values: Vec<String> },
    Select { table: String },
    Exit,
}

//
// ===== PARSER =====
//

struct Parser;

impl Parser {
    fn parse(input: &str) -> Result<Command, String> {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.is_empty() {
            return Err("Empty command".into());
        }

        match tokens[0].to_uppercase().as_str() {
            "CREATE" => match tokens.get(1).map(|s| s.to_uppercase()) {
                Some(ref t) if t == "DATABASE" => {
                    Ok(Command::CreateDatabase(tokens[2].to_string()))
                }
                Some(ref t) if t == "TABLE" => Ok(Command::CreateTable {
                    name: tokens[2].to_string(),
                    columns: tokens[3..].iter().map(|s| s.to_string()).collect(),
                }),
                _ => Err("Invalid CREATE syntax".into()),
            },
            "USE" => Ok(Command::UseDatabase(tokens[1].to_string())),
            "INSERT" => Ok(Command::Insert {
                table: tokens[1].to_string(),
                values: tokens[2..].iter().map(|s| s.to_string()).collect(),
            }),
            "SELECT" => Ok(Command::Select {
                table: tokens[1].to_string(),
            }),
            "EXIT" => Ok(Command::Exit),
            _ => Err("Unknown command".into()),
        }
    }
}

//
// ===== ENGINE =====
//

struct Engine<S: Storage> {
    storage: S,
}

impl<S: Storage> Engine<S> {
    fn new(storage: S) -> Self {
        Self { storage }
    }

    fn execute(&mut self, cmd: Command) -> Result<(), String> {
        match cmd {
            Command::CreateDatabase(name) => {
                self.storage.create_database(&name);
                println!("Database created");
            }
            Command::UseDatabase(name) => {
                self.storage.use_database(&name)?;
                println!("Using database {}", name);
            }
            Command::CreateTable { name, columns } => {
                self.storage.create_table(&name, columns)?;
                println!("Table created");
            }
            Command::Insert { table, values } => {
                self.storage.insert_row(&table, values)?;
                println!("Row inserted");
            }
            Command::Select { table } => {
                let columns = self.storage.get_columns(&table)?;
                let rows = self.storage.select_all(&table)?;

                println!("{:?}", columns);
                for r in rows {
                    println!("{:?}", r.values);
                }
            }
            Command::Exit => {
                println!("Bye");
                std::process::exit(0);
            }
        }
        Ok(())
    }
}

//
// ===== MAIN (REPL) =====
//

fn main() {
    let storage = InMemoryStorage::new();
    let mut engine = Engine::new(storage);

    loop {
        print!("db> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match Parser::parse(input.trim()) {
            Ok(cmd) => {
                if let Err(e) = engine.execute(cmd) {
                    println!("Error: {}", e);
                }
            }
            Err(e) => println!("Parse error: {}", e),
        }
    }
}
