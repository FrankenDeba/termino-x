use reqwest::blocking::Body;
use serde_json::{json, Value};
use std::io::{self, Write};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _commands = vec![
        "cd", "pwd", "ls", "..", "mkdir", "rmdir", "cp", "mv", "cat", "less", "head", "tail", "ps",
        "top", "jobs", "fg", "bg", "grep", "sed", "awk", "tr", "date", "time", "whoami",
        "hostname", "ping", "netstat", "curl", "wget", "zle", "alias", "history",
    ];

    let http_client = reqwest::blocking::Client::new();
    let url = format!("http://localhost:11434/api/generate");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin: io::Stdin = io::stdin();
        println!("Enter the operation you want to execute in natural language: \n");

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let data = json!({
          "model": "llama3.2:3b",
          "prompt": format!("create the command to {}, answer in a single command only, remove 'bash' part, remove ` also", input),
          "stream": false
        });

        let body_bytes = serde_json::to_vec(&data).unwrap(); // Convert JSON to bytes

        let body = Body::from(body_bytes);

        let response = http_client
            // form a get request with get(url)
            .post(&url)
            .body(body)
            // send the request and get Response or else return the error
            .send()?
            // get text from response or else return the error
            .text()?;

        let val: &Value = &serde_json::from_str(&response.to_string()).unwrap();

        let cmd = &String::from(val["response"].to_string());
        match input {
            _ => {
                let cmd2: Vec<&str> = cmd
                    .as_str()
                    .trim_start_matches('"')
                    .trim_end_matches('"')
                    .trim_start_matches("`")
                    .trim()
                    .split_whitespace()
                    .collect();

                println!("{:?}", &cmd2[0].trim_matches('"'));
                let output = Command::new(&cmd2[0])
                    .args(&cmd2[1..])
                    .output() // arg passed separately
                    .expect("Failed to execute command!");
                io::stdout().write_all(&output.stdout).unwrap();
            }
        };
    }
}
