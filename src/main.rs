extern crate crossbeam_channel;
extern crate simulator;

use simulator::TimeEstimation;

use simulator::{FootballSimulator, SimulationContext};

pub use chrono::prelude::NaiveDate;

fn main() {
    let mut simulator = FootballSimulator::new(num_cpus::get());

    println!("generated with {} ms", TimeEstimation::estimate(|| simulator.generate()));

    let mut context = SimulationContext::new(NaiveDate::from_ymd(2020, 11, 15));

    let total_items = simulator.items_count();

    println!("running with {} items", total_items);

    loop {
        println!("simulated with {} ms", TimeEstimation::estimate(|| simulator.simulate(&mut context)));
    }
}
