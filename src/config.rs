pub const TOKEN: &'static str = "TOKEN_HERE";
pub const PREFIX: &'static str = "PREFIX_HERE";

use ron::{ser, de};
use serde::{Serialize, Deserialize};
#[derive(Debug,Serialize,Deserialize)]
pub struct Config {
    token: &'static str,
    prefix: &'static str
} impl Config {
    pub fn new() -> Self {
        return Config {
            token: private::TOKEN,
            prefix: private::PREFIX
        }
    }

    pub fn save(&self) -> std::io::Result<()> {   
        let pretty = PrettyConfig::new()
            .with_depth_limit(2)
            .with_seperate_tuple_members(true)
            .with_enum_arrays(true);
    
        let serial = ser::to_string_pretty(&self, pretty)
            .expect("Serialization failed!");
    
        let mut file = std::fs::File::create("config.ron")?;
    
        if let Err(why) = write!(file,"{}", serial) {
            println!("Failed writing to file: {}", why);
        } else {
            println!("Write operation successful");
        }
    
        return Ok();
    }
    
    pub fn load() -> std::io::Result<Config> {
        let input_path = format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"));
    
        let file = std::fs::File::open(&input_path)
            .expect("Failed opening file");
    
        let config : Config = match de::from_reader(file){
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };
    
        return Ok(Config);
    }

    pub fn token(&self) -> &'static str {
        return self.token;
    }

    pub fn prefix(&self) -> &'static str {
        return self.prefix;
    }
}