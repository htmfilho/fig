use serde::Serialize;
use serde::Deserialize;

use std::fs;
use std::io;
use std::io::Error;
use std::io::prelude::*;
use std::path::Path;
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FigConfig {
    fig: String,
    version: String,
    gitignore: Option<String>,
    mappings: Vec<Mapping>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Mapping {
    source: String,
    target: String,
    description: String,
    profiles: Option<Vec<Profile>>,
}

impl Mapping {
    fn process(&self) -> Result<(), Error> {
        let source_lines = read_lines(&self.source);
        
        let path_target = Path::new(&self.target);
        if !path_target.exists() && !source_lines.is_empty() {
            let target_file = fs::File::create(path_target)?;
            let mut target_file = io::LineWriter::new(target_file);

            for line in source_lines {                
                target_file.write_all(line.as_bytes())?;
                target_file.write_all(b"\n")?;
            }
        }
        
        Ok(())
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let lines = match fs::read_to_string(filename) {
        Ok(content) => content.lines().map(String::from).collect(),
        Err(e) => {
            println!("Error reading file {}: {}", filename, e);
            Vec::new()
        },
    };
    return lines
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Profile {
    name: String,
    description: String,
    entries: HashMap<String, String>,
}

/// Loads the `fig.json` file in the current directory. If it doesn't exist, 
/// it creates a new one and leaves the app, giving time for the user to add the mappings.
fn load_config() -> FigConfig {
    let fig_json_path = Path::new("fig.json");
    let fig_config = match fs::File::open(&fig_json_path) {
        Err(why) => {
            println!("couldn't open fig.json: {}. \nCreating a new fig.json for this directory.", why);
            let config = FigConfig {
                fig: "0.1.0".to_string(),
                version: "1.0".to_string(),
                gitignore: None,
                mappings: vec!(Mapping {
                    source: "".to_string(),
                    target: "".to_string(),
                    description: "".to_string(),
                    profiles: Some(vec!(Profile {
                        name: "".to_string(),
                        description: "".to_string(),
                        entries: HashMap::from([
                            ("key".to_string(), "value".to_string()),
                        ]),
                    })),
                }),
            };
            let file = fs::File::create(&fig_json_path).unwrap();
            serde_json::to_writer_pretty(&file, &config).unwrap();
            config
        },
        Ok(fig_json) => serde_json::from_reader(fig_json).unwrap(),
    };
    return fig_config;
}

fn process_mappings(mappings: Vec<Mapping>) {
    for mapping in mappings {
        println!("\n{}\nCreating {} based on {}", mapping.description, mapping.target, mapping.source);
        let _ = mapping.process();
    }
}

fn main() {
    let config = load_config();
    println!("Considering Fig's config version {}", config.version);
    process_mappings(config.mappings);
}
