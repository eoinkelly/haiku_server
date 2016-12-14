extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::Read;
use yaml_rust::YamlLoader;

#[derive(Debug)]
#[derive(RustcEncodable)]
pub struct Haiku {
    pub lines: Vec<String>,
}

lazy_static! {
    static ref FALLBACK_HAIKU: Haiku = {
        println!("Building fallback Haiku");

        let lines = vec![String::from("Could not find"), String::from("A proper haiku"), String::from("So sorry friend")];

        Haiku { lines: lines }
    };
}

impl Haiku {
    pub fn choose_random(haikus: &Vec<Haiku>) -> &Self {
        match rand::thread_rng().choose(haikus) {
            Some(haiku) => haiku,
            None => &FALLBACK_HAIKU,
        }
    }

    pub fn load_from_yaml(haiku_file_path: &str) -> Vec<Haiku> {
        print!("Loading haiku file from {} ... ", haiku_file_path);
        let mut f = File::open(haiku_file_path).expect("Failed to open file");
        let mut buffer = String::new();

        f.read_to_string(&mut buffer).expect("Failed to read file into buffer");

        let mut v = Vec::new();
        let yaml_data = YamlLoader::load_from_str(&buffer).expect("Failed to load Yaml file");
        for haikus in yaml_data {
            for haiku in haikus {
                let mut lines = Vec::new();
                for line in haiku {
                    lines.push(line.into_string().expect("Failed to convert Yaml line into string"));
                }
                v.push(Haiku { lines: lines });
            }
        }

        println!("done");
        v
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_load_from_works() {
//         assert_eq!(Haiku::load_from("./data/haiku.yml"), Ok(()))
//     }
// }
