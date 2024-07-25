mod random;

use k256::elliptic_curve::group::GroupEncoding;
use k256::elliptic_curve::ops::Reduce;
use k256::elliptic_curve::sec1::FromEncodedPoint;
use k256::{self, ProjectivePoint, Scalar, U256};
use serde_json::json;
use sha2::{Digest, Sha256};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq)]
pub struct DLogProof {
    t: ProjectivePoint,
    s: Scalar,
}

///Non-interactive Schnorr ZK DLOG Proof scheme with a Fiat-Shamir transformation
impl DLogProof {
    /// Generates proof
    ///
    /// # Parameters
    ///
    /// - `sid`: String identifier.
    /// - `pid`: Protocol identifier as unsigned integer.
    /// - `x`: The secret scalar for which we are proving knowledge.
    /// - `y`: The elliptic curve point, where `y = x * base_point`.
    ///
    /// # Returns
    ///
    /// - `DLogProof`: The discrete logarithm proof containing the commitment `t` and the response `s`.
    pub fn prove(sid: &str, pid: u32, x: Scalar, y: ProjectivePoint) -> DLogProof {
        let r = random::generate_random_number();
        let base_point = k256::ProjectivePoint::GENERATOR;

        let t = base_point * r;
        let c = hash_pointers(sid, pid, [&base_point, &y, &t]);
        let s = r + c * x;

        DLogProof { t, s }
    }

    /// Verifies proof
    ///
    /// # Parameters
    ///
    /// - `sid`: String identifier
    /// - `pid`: Protocol identifier as unsigned integer.
    /// - `y`: The elliptic curve point, where `y = x * base_point`.
    ///
    /// # Returns
    ///
    /// - `bool`: true if verification succeeds, false otherwise
    pub fn verify(&self, sid: &str, pid: u32, y: ProjectivePoint) -> bool {
        let base_point = k256::ProjectivePoint::GENERATOR;

        let c = hash_pointers(sid, pid, [&base_point, &y, &self.t]);
        let lhs = base_point * self.s;
        let rhs = self.t + (y * c);
        lhs == rhs
    }

    /// Serializes to serde_json::Value as universal middle layer for further processing
    ///
    /// # Returns
    ///
    /// - `serde_json::Value`: contains {"t": ... "s": ...} as json object
    pub fn to_dict(&self) -> serde_json::Value {
        json!({
            "t": self.t.to_affine().to_bytes().to_vec(),
            "s": self.s.to_bytes().to_vec(),
        })
    }

    /// Serializes to serde_json::Value as universal middle layer for further processing
    ///
    /// # Parameters
    /// - `json`: Input for deserialization as `serde_json::Value`
    ///
    /// # Returns
    ///
    /// - `serde_json::Value`: contains {"t": ... "s": ...} as json object
    pub fn from_dict(json: serde_json::Value) -> Result<Self, String> {
        let t_bytes = unpack_bytes(&json["t"])?;
        let s_bytes = unpack_bytes(&json["s"])?;

        let recovered_point: Option<_> = ProjectivePoint::from_encoded_point(
            &k256::EncodedPoint::from_bytes(&t_bytes).map_err(|e| e.to_string())?,
        )
        .into();

        let recovered_scalar = <Scalar as Reduce<U256>>::reduce_bytes(s_bytes.as_slice().into());

        Ok(DLogProof {
            t: recovered_point.ok_or("Could not convert recovered_point")?,
            s: recovered_scalar,
        })
    }
}

fn unpack_bytes(json: &serde_json::Value) -> Result<Vec<u8>, String> {
    let t_bytes: Result<Vec<u8>, String> = json
        .as_array()
        .ok_or("Expected an array".to_string())?
        .iter()
        .map(|x| {
            x.as_u64()
                .and_then(|n| u8::try_from(n).ok())
                .ok_or("Invalid number".to_string())
        })
        .collect();
    t_bytes
}

impl ToString for DLogProof {
    fn to_string(&self) -> String {
        self.to_dict().to_string()
    }
}

fn hash_pointers(sid: &str, pid: u32, points: [&ProjectivePoint; 3]) -> Scalar {
    let mut hasher = Sha256::new();
    hasher.update(sid.as_bytes());
    hasher.update(&pid.to_be_bytes());
    hasher.update(&points[0].to_affine().to_bytes());
    hasher.update(&points[1].to_affine().to_bytes());
    hasher.update(&points[2].to_affine().to_bytes());
    let result: &[u8] = &hasher.finalize();

    <Scalar as Reduce<U256>>::reduce_bytes(result.into())
}
