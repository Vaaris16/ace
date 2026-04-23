use crate::framework::framework::Frameworks;

pub fn unocss(framework: Frameworks) -> &'static str {
    match framework {
        Frameworks::Vanilla => "",
        Frameworks::Vue => "",
        Frameworks::React => "",
        Frameworks::Preact => "",
        Frameworks::Lit => "",
        Frameworks::Svelte => "",
        Frameworks::Solid => "",
        Frameworks::Ember => "",
        Frameworks::Qwik => "",
        Frameworks::Angular => "",
        Frameworks::Marko => "",
    }
}
