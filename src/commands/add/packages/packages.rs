use serde::Deserialize;

#[derive(Deserialize)]
pub struct Packages {
    pub name: String,
    pub steps: Vec<Step>,
}

#[derive(Deserialize)]
pub struct Step {
    pub cmd: String,
    pub command: Option<String>,
    pub arg: Option<String>,
    pub path: Option<String>,

    pub content: Option<String>,
    pub insert: Option<String>,
    pub needle: Option<String>,
}
