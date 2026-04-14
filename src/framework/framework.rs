use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum Frameworks {
    Vanilla,
    Vue,
    React,
    Preact,
    Lit,
    Svelte,
    Solid,
    Ember,
    Qwik,
    Angular,
    Marko,
}
