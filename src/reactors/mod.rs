mod errors;

pub use errors::*;
use hlua::Lua;
use log::debug;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string};
use walkdir::WalkDir;

pub fn load_reactors() -> Result<ReactorList, ReactorParseError> {
    let mut reactor_list = ReactorList::new();
    for file_path in WalkDir::new("reactions")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().unwrap() == "lua")
    {
        debug!("Opening lua file {}", file_path.path().display());

        let reactor_def = match read_reactor(file_path.path().to_str().unwrap()) {
            Ok(e) => e,
            Err(e) => {
                return Err(e);
            }
        };

        let reactor: Reactor = reactor_def.clone().into();

        let structure = match reactor_def.structure {
            Some(a) => a,
            None => {
                return Err(ReactorParseError {
                    error: Box::new(NoStructureError {}),
                    file: file_path.path().to_str().unwrap().to_owned(),
                });
            }
        };

        let mut reactors = match reactor_list.get(&structure) {
            Some(a) => a.to_owned(),
            None => {
                vec![]
            }
        };

        if reactor.mutating && reactors.iter().filter(|e| e.mutating).count() > 0 {
            return Err(ReactorParseError {
                file: file_path.path().to_str().unwrap().to_string(),
                error: Box::new(TooManyMutatingReactorsError {
                    structure: structure,
                }),
            });
        }

        reactors.push(reactor);

        reactor_list.insert(structure, reactors);
    }

    Ok(reactor_list)
}

fn read_reactor(file_path: &str) -> Result<ReactorDef, ReactorParseError> {
    let mut lua = Lua::new();
    lua.openlibs();
    lua.open_package();

    let source = read_to_string(file_path).unwrap();

    lua.execute::<()>(include_str!("../json.lua")).unwrap();
    lua.execute::<()>("json = setupJson();").unwrap();
    match lua.execute::<()>(source.as_str()) {
        Ok(_) => {}
        Err(err) => {
            return Err(ReactorParseError {
                error: Box::new(err),
                file: file_path.to_string(),
            })
        }
    };

    match lua.execute::<()>("initStr = json.encode(reactorDef)") {
        Ok(_) => {}
        Err(err) => {
            return Err(ReactorParseError {
                error: Box::new(err),
                file: file_path.to_string(),
            })
        }
    };

    let reactor_def_string: String = lua.get("initStr").unwrap();
    let mut reactor_def: ReactorDef = match serde_json::from_str(&reactor_def_string) {
        Ok(v) => v,
        Err(err) => {
            return Err(ReactorParseError {
                error: Box::new(err),
                file: file_path.to_string(),
            })
        }
    };

    reactor_def.set_source(&source);

    Ok(reactor_def.clone())
}

type ReactorList = HashMap<String, Vec<Reactor>>;

#[derive(Debug, Clone, Deserialize)]
struct ReactorDef {
    #[serde(rename = "Structure")]
    structure: Option<String>,
    #[serde(rename = "Mutating")]
    mutating: Option<bool>,
    #[serde(rename = "Posting")]
    posting: Option<bool>,
    #[serde(rename = "References")]
    refs: Option<Vec<String>>,
    #[serde(skip)]
    source: String,
}

impl ReactorDef {
    pub fn set_source(&mut self, source: &String) {
        self.source = source.to_owned();
    }
}

impl Into<Reactor> for ReactorDef {
    fn into(self) -> Reactor {
        Reactor {
            mutating: match self.mutating {
                Some(v) => v,
                None => false,
            },
            posting: match self.posting {
                Some(v) => v,
                None => false,
            },
            refs: match self.refs {
                Some(v) => v,
                None => vec![],
            },
            source: self.source,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reactor {
    mutating: bool,
    posting: bool,
    refs: Vec<String>,
    source: String,
}
