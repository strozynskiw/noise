use k256::ProjectivePoint;

mod random;
use random::generate_random_number;

use task1::DLogProof;

fn main() {
    let sid = "sid";
    let pid = 1;

    let x = generate_random_number();
    println!("x: {:?}", x);
    let y = ProjectivePoint::GENERATOR * &x;

    let start_proof = std::time::Instant::now();
    let dlog_proof = DLogProof::prove(sid, pid, x, y);
    println!(
        "Proof computation time: {} µs",
        start_proof.elapsed().as_micros()
    );

    let start_verify = std::time::Instant::now();
    let result = dlog_proof.verify(sid, pid, y);
    println!(
        "Verify computation time: {} µs",
        start_verify.elapsed().as_micros()
    );

    if result {
        println!("DLOG proof is correct");
    } else {
        println!("DLOG proof is not correct");
    }
}
