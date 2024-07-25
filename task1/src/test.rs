#[cfg(test)]
mod tests {
    use k256::ProjectivePoint;
    use serde_json::json;

    use crate::{random::generate_random_number, DLogProof};

    #[test]
    fn it_works() {
        let sid = "random_id";
        let pid = 1;
        let x = generate_random_number();
        let y = ProjectivePoint::GENERATOR * &x;
        let dlog_proof = DLogProof::prove(sid, pid, x, y);
        assert!(dlog_proof.verify(sid, pid, y));
    }

    #[test]
    fn wrong_pid() {
        let sid = "random_id";
        let x = generate_random_number();
        let y = ProjectivePoint::GENERATOR * &x;
        let dlog_proof = DLogProof::prove(sid, 1, x, y);
        assert!(!dlog_proof.verify(sid, 2, y));
    }

    #[test]
    fn wrong_sid() {
        let sid1 = "random_id1";
        let sid2 = "random_id2";
        let pid = 1;
        let x = generate_random_number();
        let y = ProjectivePoint::GENERATOR * &x;
        let dlog_proof = DLogProof::prove(sid1, pid, x, y);
        assert!(!dlog_proof.verify(sid2, pid, y));
    }

    #[test]
    fn wrong_y() {
        let sid = "random_id";
        let pid = 1;
        let x1 = generate_random_number();
        let y1 = ProjectivePoint::GENERATOR * &x1;
        let x2 = generate_random_number();
        let y2 = ProjectivePoint::GENERATOR * &x2;
        let dlog_proof = DLogProof::prove(sid, pid, x1, y1);
        assert!(!dlog_proof.verify(sid, pid, y2));
    }

    #[test]
    fn it_serializes() {
        let sid = "random_id";
        let pid = 1;
        let x = generate_random_number();
        let y = ProjectivePoint::GENERATOR * &x;
        let dlog_proof = DLogProof::prove(sid, pid, x, y);
        let string = dlog_proof.to_dict().to_string();
        dbg!(&string);
        assert!(string.contains(r#""s":["#));
        assert!(string.contains(r#""t":["#));
    }

    #[test]
    fn it_deserializes() {
        let json = json!(
            {
                "s":[2,46,87,33,225,85,57,123,117,167,81,153,112,38,231,166,71,46,43,227,16,103,131,232,242,51,222,202,210,153,141,100],
                "t":[2,14,42,235,92,45,145,127,141,202,149,81,252,126,137,128,138,9,47,75,133,16,20,38,190,191,2,108,187,195,66,40,121]
            }
        );
        let instance = DLogProof::from_dict(json.clone());
        assert!(instance.is_ok());
        let instance = instance.unwrap();

        let json_serialized = instance.to_dict();
        assert_eq!(json, json_serialized);
    }
}
