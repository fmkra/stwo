use num_traits::One;

use self::air::FibonacciAir;
use self::component::FibonacciComponent;
use crate::core::backend::cpu::CpuCircleEvaluation;
use crate::core::channel::{Blake2sChannel, Channel};
use crate::core::fields::m31::{BaseField, M31};
use crate::core::fields::{FieldExpOps, IntoSlice};
use crate::core::poly::circle::{CanonicCoset, CircleEvaluation};
use crate::core::poly::BitReversedOrder;
use crate::core::prover::{commit_and_prove, commit_and_verify, prove, verify, ProvingError, StarkProof, VerificationError};
use crate::core::vcs::blake2_hash::Blake2sHasher;
use crate::core::vcs::hasher::Hasher;

pub mod air;
mod component;

#[derive(Clone)]
pub struct Fibonacci {
    pub air: FibonacciAir,
}

impl Fibonacci {
    pub fn new(log_size: u32, claim: BaseField) -> Self {
        let component = FibonacciComponent::new(log_size, claim);
        Self {
            air: FibonacciAir::new(component),
        }
    }

    pub fn prove(&self) -> Result<StarkProof, ProvingError> {
        let trace = self.get_trace();
        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[self
            .air
            .component
            .claim])));
        commit_and_prove(&self.air, channel, vec![trace])
    }

    pub fn verify(&self, proof: StarkProof) -> Result<(), VerificationError> {
        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[self
            .air
            .component
            .claim])));
        commit_and_verify(proof, &self.air, channel)
    }

    pub fn get_trace(&self) -> CpuCircleEvaluation<BaseField, BitReversedOrder> {
        let trace_domain = CanonicCoset::new(self.air.component.log_size);
        // let mut trace = Vec::with_capacity(trace_domain.size());

        // let mut a = BaseField::one();
        // let mut b = BaseField::one();
        // for _ in 0..trace_domain.size() {
        //     trace.push(a);
        //     let tmp = a.square() + b.square();
        //     a = b;
        //     b = tmp;
        // }

        // let trace = vec![4, 1, 3, 4, 2, 12, 1, 24];
        // let trace = vec![4, 1, 3, 4, 2, 12, 1, 24];
        let trace = vec![8, 1, 7, 8, 6, 56, 5, 336, 4, 1680, 3, 6720, 2, 20160, 1, 40320];

        let trace = trace.iter().map(|&x| M31::from(x)).collect();
        CircleEvaluation::new_canonical_ordered(trace_domain, trace)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::fields::m31::M31;

    #[test]
    pub fn test_main() {
        let fibonacci = super::Fibonacci::new(4, M31::from(40320));
        let proof = fibonacci.prove();
        assert!(proof.is_ok());

        let valid = fibonacci.verify(proof.unwrap());
        assert!(valid.is_ok());
    }
}
