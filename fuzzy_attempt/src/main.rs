mod fuzzy_set;
use fuzzy_set::Point;
use fuzzy_set::FuzzySet;
mod fuzzy_controller;
use fuzzy_controller::FuzzyController;

use log::info;
use log4rs;

fn main() {
    println!("Started");
    
    // Inititialise logging
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("==========================================================================");
    info!("Started");
    info!("Local timezone = {}", chrono::Local::now().offset());

    // Create FuzzyController
    let mut controller = FuzzyController::new();
    controller.add_set(FuzzySet::new_leading("Very Cold", -30, -20, -10));
    controller.add_set(FuzzySet::new_triangle("Cold", -20, -10, 0));
    controller.add_set(FuzzySet::new_triangle("Cool", -10, 0, 10));
    controller.add_set(FuzzySet::new_triangle("Warm", 0, 10, 20));
    controller.add_set(FuzzySet::new_trailing("Hot", 10, 20, 30));
    
    // Debug
    for graph in controller.membership_asciigraphs() {
        info!("{}", graph);
    }

    info!("Finished");
    println!("Finished");
}
