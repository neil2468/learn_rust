mod fuzzy_lib;
use fuzzy_lib::*;

#[macro_use]
extern crate log;
use env_logger::Env;



fn main() {
    // Init logging
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    info!("Started");

    let set = FuzzySet::new(
        "B",
        &[(30, 0.0), (60, 0.5), (70, 1.0), (90, 1.0), (100, 0.0)],
    );

    info!("\n{}", set.membership_asciigraph(25..105, (100, 20)));
    

    info!("Finished");
}

