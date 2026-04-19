use crate::framework::framework::Frameworks;

pub fn tailwindcss(framework: Frameworks) -> &'static str {
    match framework {
        Frameworks::Vanilla => include_str!("instructions/tailwind_vanilla.toml"),
        Frameworks::Vue => include_str!("instructions/tailwind_vue.toml"),
        Frameworks::React => include_str!("instructions/tailwind_react.toml"),
        Frameworks::Preact => include_str!("instructions/tailwind_preact.toml"),
        Frameworks::Lit => include_str!("instructions/tailwind_lit.toml"),
        Frameworks::Svelte => include_str!("instructions/tailwind_svelte.toml"),
        Frameworks::Solid => include_str!("instructions/tailwind_solid.toml"),
        Frameworks::Ember => include_str!("instructions/tailwind_ember.toml"),
        Frameworks::Qwik => include_str!("instructions/tailwind_qwik.toml"),
        Frameworks::Angular => include_str!("instructions/tailwind_angular.toml"),
        Frameworks::Marko => include_str!("instructions/tailwind_marko.toml"),
    }
}
