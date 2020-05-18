#[cfg(feature = "script_helper")]
use std::error::Error;

#[cfg(feature = "script_helper")]
use handlebars::Handlebars;

#[cfg(feature = "script_helper")]
#[macro_use]
extern crate serde_json;

#[cfg(feature = "script_helper")]
fn main() -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("tpl", "./examples/script/template.hbs")?;
    handlebars.register_script_helper_file("score", "./examples/script/goals.rhai")?;

    let data = json! {[
        [{
            "name": "Dortmund",
            "goals": ["Haaland", "Guerreiro", "Hazard", "Guerreiro"]
        }, {
            "name": "Schalke",
            "goals": []
        }],
        [{
            "name": "RB Leipzig",
            "goals": ["Poulsen"]
        }, {
            "name": "SC Feriburg",
            "goals": ["Gulde"]
        }]
    ]};
    println!("{}", handlebars.render("tpl", &data)?);
    Ok(())
}

#[cfg(not(feature = "script_helper"))]
fn main() {
    println!("Please enable feature flag script_helper for this example");
}
