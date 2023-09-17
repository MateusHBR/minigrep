use std::fs;

#[derive(Default)]
pub struct ConfigBuilder {
    args: Vec<String>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_args(mut self, args: &[String]) -> Self {
        self.args = args.into();
        self
    }

    pub fn build(self) -> Result<Config, &'static str> {
        let (Some(query), Some(file_path)) = (self.args.get(1), self.args.get(2)) else {
            return Err("Expected args <query> <file-path>");
        };
        let settings: SearchSettings =
            self.args
                .get(3)
                .map_or(SearchSettings::default(), |arg| match arg.as_str() {
                    "--case-insensitive" | "-i" => SearchSettings::CaseInsensitive,
                    _ => SearchSettings::default(),
                });

        Ok(Config::new(query, file_path, settings))
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub settings: SearchSettings,
}

impl Config {
    pub fn new(
        query: impl Into<String>,
        file_path: impl Into<String>,
        settings: SearchSettings,
    ) -> Self {
        Self {
            query: query.into(),
            file_path: file_path.into(),
            settings,
        }
    }
}

pub fn run(config: &Config) -> Result<(), &'static str> {
    let Ok(content) = fs::read_to_string(&config.file_path) else {
        return Err("Could not read the file");
    };

    search(&config.query, &content, &config.settings)
        .iter()
        .for_each(|line| {
            println!("{}", line);
        });

    Ok(())
}

#[derive(Default)]
pub enum SearchSettings {
    #[default]
    CaseSensitive,

    CaseInsensitive,
}

pub fn search<'a>(query: &str, content: &'a str, settings: &SearchSettings) -> Vec<&'a str> {
    let query = match settings {
        SearchSettings::CaseSensitive => query.to_owned(),
        SearchSettings::CaseInsensitive => query.to_lowercase(),
    };

    content
        .lines()
        .filter(|line| match settings {
            SearchSettings::CaseSensitive => line.contains(&query),
            SearchSettings::CaseInsensitive => line.to_lowercase().contains(&query),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod search {
        use super::*;

        const CONTENT: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
Duct.";

        #[test]
        fn case_sensitive() {
            let query = "duct";

            assert_eq!(
                vec!["safe, fast, productive."],
                search(query, CONTENT, &SearchSettings::CaseSensitive)
            );
        }

        #[test]
        fn case_insensitive() {
            let query = "rUsT";

            assert_eq!(
                vec!["Rust:", "Trust me."],
                search(query, CONTENT, &SearchSettings::CaseInsensitive)
            );
        }
    }
}
