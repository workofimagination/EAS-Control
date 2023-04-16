use std::{fs::File, io::{BufRead, BufReader, self }, io::{Write}};

pub struct Coordinate {
    pub x: f32,
    pub y: f32,
    pub post_draw: PostDraw,
}

pub enum PostDraw {
    Run,
    Pause,
}


pub struct FileParser {
}

impl FileParser {
    pub fn parse() -> Vec<Coordinate> {
        // .gs file parser v1.0 - Eric
        let mut coords: Vec<Coordinate> = vec![];
        for line in FileParser::read_file() {
            let l = line.unwrap();
            let split: Vec<&str> = l.split(' ').collect();
            if split[0].parse::<f32>().is_ok() {
                let mut x: f32 = 0.0;
                let mut y: f32;
                let mut post_draw: PostDraw;
                let mut coord: Coordinate;

                for entry in split {
                    if entry.contains(";") {
                        let entry_split: Vec<&str> = entry.split(";").collect();
                        y = entry_split[0].parse::<f32>().unwrap();
                        post_draw = PostDraw::Pause;
                        coord = Coordinate {x, y, post_draw};
                        coords.push(coord);
                        if entry_split[1].parse::<f32>().is_ok() {
                            x = entry_split[1].parse::<f32>().unwrap();
                        }
                    } else if entry.contains("-") {
                        let entry_split: Vec<&str> = entry.split("-").collect();
                        y = entry_split[0].parse::<f32>().unwrap();
                        post_draw = PostDraw::Run;
                        coord = Coordinate {x, y, post_draw};
                        coords.push(coord);
                        if entry_split[1].parse::<f32>().is_ok() {
                            x = entry_split[1].parse::<f32>().unwrap();
                        }
                    } else {
                        if entry.parse::<f32>().is_ok() {
                            x = entry.parse::<f32>().unwrap();
                        }
                    }
                }
            }
        }
        return coords;
    }

    fn read_file() -> io::Lines<BufReader<File>> { return io::BufReader::new(File::open("./first.gs").unwrap()).lines(); }

    fn write_to_file(contents: &String) -> bool {
        let mut file = match File::create("./first.gs") {
            Ok(file) => file,
            Err(_) => return false
        };

        match write!(file, "{}", contents) {
            Ok(_) => return true,
            Err(_) => return false
        };

    }

}



