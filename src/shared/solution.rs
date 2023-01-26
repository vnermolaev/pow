use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Challenge {
    pub value: u64,
    /// Number of leading zeros to meet.
    pub n_leading_zeros: u8,
}

impl Challenge {
    pub fn random(n_leading_zeros: u8) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            value: rng.gen(),
            n_leading_zeros,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Solution {
    challenge: u64,
    nonce: u64,
}

impl Solution {
    const MAX_LEADING_ZEROS: u8 = 32;

    pub fn find(challenge: &Challenge) -> Result<Solution, SolutionError> {
        if challenge.n_leading_zeros > Self::MAX_LEADING_ZEROS {
            return Err(SolutionError::ChallengeTargetIncorrect(
                challenge.n_leading_zeros,
            ));
        }

        let mut solution = Self {
            challenge: challenge.value,
            nonce: 0,
        };

        while matches!(solution.verify(challenge)?, VerificationStatus::Invalid) {
            solution.nonce += 1;
        }

        Ok(solution)
    }

    pub fn verify(&self, challenge: &Challenge) -> Result<VerificationStatus, SolutionError> {
        if challenge.n_leading_zeros > Self::MAX_LEADING_ZEROS {
            return Err(SolutionError::ChallengeTargetIncorrect(
                challenge.n_leading_zeros,
            ));
        }

        if self.challenge != challenge.value {
            return Err(SolutionError::SolutionChallengeMismatch {
                challenge_value: challenge.value,
                solution_value: self.challenge,
            });
        }

        let ser = bincode::serialize(self).expect("Serialization of built-in types must succeed");

        let mut hasher = Sha256::new();
        hasher.update(&ser);
        let hash = hasher.finalize().into_iter().collect::<Vec<_>>();

        Ok(
            if challenge.n_leading_zeros == 0
                || hash
                    .iter()
                    .take(challenge.n_leading_zeros as usize)
                    .all(|b| *b == 0)
            {
                VerificationStatus::Valid(hash)
            } else {
                VerificationStatus::Invalid
            },
        )
    }
}

#[derive(Debug)]
pub enum VerificationStatus {
    Invalid,
    Valid(Vec<u8>),
}

#[derive(Error, Debug)]
pub enum SolutionError {
    #[error("Target number of leading zeros can be at most 32, found: {0}")]
    ChallengeTargetIncorrect(u8),
    #[error("Challenge must be {challenge_value}, solution contains {solution_value}")]
    SolutionChallengeMismatch {
        challenge_value: u64,
        solution_value: u64,
    },
}

#[cfg(test)]
mod test {
    use crate::shared::solution::{Challenge, Solution, SolutionError, VerificationStatus};

    #[test]
    fn prove_verify() {
        let challenge = Challenge::random(2);
        let s = Solution::find(&challenge).unwrap();

        println!("Solution: {s:?}");

        let v = s.verify(&challenge);
        println!("Verification: {v:?}");

        assert!(matches!(v, Ok(VerificationStatus::Valid(_))));
    }

    #[test]
    fn fake_verify() {
        let challenge = Challenge {
            value: 10,
            n_leading_zeros: 2,
        };
        let s = Solution::find(&challenge).unwrap();

        println!("Solution: {s:?}");

        let challenge = Challenge {
            value: 11,
            n_leading_zeros: 1,
        };
        let v = s.verify(&challenge);
        println!("Verification: {v:?}");

        assert!(matches!(
            v,
            Err(SolutionError::SolutionChallengeMismatch { .. })
        ));
    }
}
