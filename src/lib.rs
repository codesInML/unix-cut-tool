use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};

#[derive(Debug)]
pub struct Config {
    filename: Option<String>,
    delimiter: char,
    fields: Vec<usize>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("not enough arguments".to_string());
        }

        let err = String::from("invalid argument passed");
        let mut filename = None;
        let mut delimiter = '\t';
        let mut fields = Vec::new();

        for arg in args {
            if arg.starts_with("-f") {
                let range: Vec<&str> = if arg.contains(" ") {
                    arg[2..].split(" ").collect()
                } else {
                    arg[2..].split(",").collect()
                };

                fields = range
                    .iter()
                    .map(|num| num.trim().parse::<usize>().ok().unwrap() - 1)
                    .collect();
            } else if arg.starts_with("-d") {
                if arg[2..].chars().count() != 1 {
                    return Err("invalid delimiter".to_string());
                }

                delimiter = match arg.chars().nth(2) {
                    Some(del) => del,
                    None => return Err("invalid delimiter".to_string()),
                };
            } else if !arg.starts_with("-") {
                if filename != None {
                    return Err(err);
                }
                filename = Some(arg.clone());
            } else if arg == "-" {
                // This signals that content should be loaded from the standard input
            } else {
                return Err(err);
            }
        }

        Ok(Config {
            filename,
            delimiter,
            fields,
        })
    }
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    let mut contents = String::new();
    if let Some(name) = config.filename {
        let mut f = File::open(name)?;
        f.read_to_string(&mut contents)?;
    } else {
        read_from_stdin(&mut contents)?;
    }

    let contents = to_2d_vector(&contents, config.delimiter);
    let mut built_content = String::new();

    for line in contents {
        let mut built_line = Vec::new();

        for (i, field) in line.into_iter().enumerate() {
            if config.fields.contains(&i) {
                built_line.push(field);
            }
        }

        if built_line.len() > 0 {
            built_content.push_str(&built_line.join(&config.delimiter.to_string()));
            built_content.push_str("\n")
        }
    }

    Ok(built_content)
}

fn to_2d_vector(content: &str, delimiter: char) -> Vec<Vec<String>> {
    content
        .lines()
        .map(|line| {
            line.split(delimiter)
                .map(|value| value.trim().to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

fn read_from_stdin(contents: &mut String) -> Result<(), String> {
    match io::stdin().read_to_string(contents) {
        Ok(_) => Ok(()),
        Err(_) => Err("could not read from standard input".to_string()),
    }
}
