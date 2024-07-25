use k256::{elliptic_curve::Field, Scalar};

#[allow(dead_code)]
pub(super) fn generate_random_number() -> Scalar {
    let rng = rand::thread_rng();
    Scalar::random(rng)
}
