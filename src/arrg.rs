use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct Arrg {
    pub argument_list: HashMap<String, String>,
    pub command_list: HashMap<String, String>,
}

impl Arrg {
    /// Create a new argument parser.
    pub fn new() -> Arrg {
        Arrg {
            argument_list: HashMap::new(),
            command_list: HashMap::new()
        }
    }

    /// Add a new command to be parsed.
    pub fn command<'a>(&mut self, key: &'a str, label: &'a str) -> &mut Self {
        self.command_list.insert(String::from(key), String::from(label));

        self
    }

    /// Parse all configured commands found in the argument string.
    pub fn parse(&mut self) -> HashMap<String, String> {
        let mut holding: Option<String> = None;

        let mut arguments = env::args();

        // the first argument is the script call
        arguments.next();

        for argument in arguments {
            match holding {
                Some(key) => {
                    match self.command_list.get(key.as_str()) {
                        Some(label) => {
                            self.argument_list.insert(label.clone(), argument.clone());
                        },
                        None => {}
                    };

                    holding = None;
                },
                None => {
                    holding = Some(argument.clone());
                }
            };
        }

        self.argument_list.clone()
    }
}
