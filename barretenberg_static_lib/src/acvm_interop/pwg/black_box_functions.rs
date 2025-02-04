use crate::Barretenberg;

use common::acvm::acir::{circuit::opcodes::BlackBoxFuncCall, native_types::Witness};
use common::acvm::{FieldElement, OpcodeResolutionError};
use common::black_box_functions::BarretenbergShared;
use std::collections::BTreeMap;

// Note that the outputs for things like Sha256 need to be computed
// as they may be used in later arithmetic gates

pub(super) struct BlackBoxFuncCaller;

impl BarretenbergShared for Barretenberg {
    fn new() -> Self {
        Barretenberg::new()
    }

    fn verify_signature(
        &mut self,
        pub_key: [u8; 64],
        sig: [u8; 64],
        message: &[u8],
    ) -> FieldElement {
        self.verify_signature(pub_key, sig, message)
    }

    fn fixed_base(&mut self, input: &FieldElement) -> (FieldElement, FieldElement) {
        self.fixed_base(input)
    }

    fn encrypt(&mut self, inputs: Vec<FieldElement>) -> (FieldElement, FieldElement) {
        self.encrypt(inputs)
    }
}

impl BlackBoxFuncCaller {
    pub(super) fn solve_black_box_func_call(
        initial_witness: &mut BTreeMap<Witness, FieldElement>,
        gadget_call: &BlackBoxFuncCall,
    ) -> Result<(), OpcodeResolutionError> {
        common::black_box_functions::solve_black_box_func_call::<Barretenberg>(
            initial_witness,
            gadget_call,
        )
    }
}
