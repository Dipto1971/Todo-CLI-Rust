use std::env;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::process;

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: String,
    pub todo_backup: String,
    pub no_backup: bool,
}

