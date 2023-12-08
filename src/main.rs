use serde::Serialize;
use serde::Deserialize;
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

fn main() {
    let config = load_config();
    println!("Config version {} loaded.", config.version);
    
}
