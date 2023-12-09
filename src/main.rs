use serde::Serialize;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::Path;
use serde_json;
use std::fs::File;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Profile {
    name: String,
    description: String,
    entries: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Mapping {
    source: String,
    target: String,
    description: String,
    profiles: Vec<Profile>,
}

impl Mapping {
    fn process(&self) {
        let source_lines = read_lines(&self.source);
        
        for line in source_lines {
            println!("{}", line);
        }
        
        for profile in &self.profiles {
            println!("Profile: {}", profile.name);
            println!("Description: {}", profile.description);
            for (key, value) in &self.profiles[0].entries {
                println!("{}: {}", key, value);
            }
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let lines = match read_to_string(filename) {
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
struct FigConfig {
    fig: String,
    version: String,
    mappings: Vec<Mapping>,
}

/// Loads the `fig.json` file in the current directory. If it doesn't exist, 
/// it creates a new one and leaves the app, giving time for the user to add the mappings.
fn load_config() -> FigConfig {
    let fig_json_path = Path::new("fig.json");
    let fig_config = match File::open(&fig_json_path) {
        Err(why) => {
            println!("couldn't open fig.json: {}. \nCreating a new fig.json for this directory.", why);
            let config = FigConfig {
                fig: "0.1.0".to_string(),
                version: "1.0".to_string(),
                mappings: vec!(Mapping {
                    source: "".to_string(),
                    target: "".to_string(),
                    description: "".to_string(),
                    profiles: vec!(Profile {
                        name: "".to_string(),
                        description: "".to_string(),
                        entries: HashMap::from([
                            ("key".to_string(), "value".to_string()),
                        ]),
                    }),
                }),
            };
            let file = File::create(&fig_json_path).unwrap();
            serde_json::to_writer_pretty(&file, &config).unwrap();
            config
        },
        Ok(fig_json) => serde_json::from_reader(fig_json).unwrap(),
    };
    return fig_config;
}

fn process_mappings(mappings: Vec<Mapping>) {
    for mapping in mappings {
        println!("\nCreating {} based on {}\n{}", mapping.target, mapping.source, mapping.description);
        mapping.process();
    }
}

fn main() {
    let config = load_config();
    println!("Considering Fig's config version {}", config.version);
    process_mappings(config.mappings);
}
