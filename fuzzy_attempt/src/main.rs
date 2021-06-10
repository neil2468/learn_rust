mod fuzzy_set;
use fuzzy_set::FuzzySet;

use log::info;
use log4rs;

const ASCIIGRAPH_DIMENSIONS: (u32, u32) = (100, 10);

fn main() {
    println!("Started");
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("==========================================================================");
    info!("Started");
    info!("Local timezone = {}", chrono::Local::now().offset());

    let set_a = FuzzySet::new("A", &[(0, 1.0), (20, 1.0), (40, 0.0)]);
    let set_b = FuzzySet::new(
        "B",
        &[(30, 0.0), (60, 0.25), (70, 1.0), (90, 1.0), (100, 0.0)],
    );

    info!(
        "\n{}",
        set_a.membership_asciigraph(0..100, ASCIIGRAPH_DIMENSIONS)
    );
    info!(
        "\n{}",
        set_b.membership_asciigraph(0..100, ASCIIGRAPH_DIMENSIONS)
    );

    info!("Finished");
    println!("Finished");
}
