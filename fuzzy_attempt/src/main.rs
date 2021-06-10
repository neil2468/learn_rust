mod fuzzy_set;
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

    // Creat and configure FuzzyController
    let mut controller = FuzzyController::new();
    controller.add_set(FuzzySet::new("A", &[(0, 1.0), (20, 1.0), (40, 0.0)]));
    controller.add_set(FuzzySet::new(
        "B",
        &[(30, 0.0), (60, 0.25), (70, 1.0), (90, 1.0), (100, 0.0)],
    ));
    
    // Debug
    for graph in controller.membership_asciigraphs(0..=100) {
        info!("{}", graph);
    }







    info!("Finished");
    println!("Finished");
}
