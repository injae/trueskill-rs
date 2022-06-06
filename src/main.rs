use itertools::Itertools;
use trueskill::{Rating, quality::quality};


const MU: f64 = 25.;
const SIGMA: f64 = MU / 3.;
const BETA: f64 = SIGMA / 2.;

fn main() {
    println!("{:?}",(0..10).permutations(2).collect_vec());
    println!("beta={}, {}",BETA, quality(&[vec![Rating::new(25., 5.)], vec![Rating::new(25.,5.)]], None, BETA));
}
