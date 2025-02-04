use super::Plonk;
use crate::composer::StandardComposer;
use crate::Barretenberg;
use common::acvm::acir::{circuit::Circuit, native_types::Witness};
use common::acvm::FieldElement;
use common::acvm::{Language, ProofSystemCompiler};
use common::barretenberg_structures::Assignments;
use common::proof;
use common::serializer::serialize_circuit;
use std::collections::BTreeMap;

impl ProofSystemCompiler for Plonk {
    fn prove_with_meta(
        &self,
        circuit: Circuit,
        witness_values: BTreeMap<Witness, FieldElement>,
    ) -> Vec<u8> {
        let constraint_system = serialize_circuit(&circuit);

        let mut composer = StandardComposer::new(constraint_system);

        let assignments = proof::flatten_witness_map(&circuit, witness_values);

        composer.create_proof(assignments)
    }

    fn verify_from_cs(
        &self,
        proof: &[u8],
        public_inputs: Vec<FieldElement>,
        circuit: Circuit,
    ) -> bool {
        let constraint_system = serialize_circuit(&circuit);

        let mut composer = StandardComposer::new(constraint_system);

        composer.verify(proof, Assignments::from_vec(public_inputs))
    }

    fn np_language(&self) -> Language {
        Language::PLONKCSat { width: 3 }
    }

    fn get_exact_circuit_size(&self, circuit: &Circuit) -> u32 {
        let constraint_system = serialize_circuit(circuit);

        let mut barretenberg = Barretenberg::new();

        StandardComposer::get_exact_circuit_size(&mut barretenberg, &constraint_system)
    }

    fn black_box_function_supported(&self, opcode: &common::acvm::acir::BlackBoxFunc) -> bool {
        match opcode {
            common::acvm::acir::BlackBoxFunc::AES => false,
            common::acvm::acir::BlackBoxFunc::AND => true,
            common::acvm::acir::BlackBoxFunc::XOR => true,
            common::acvm::acir::BlackBoxFunc::RANGE => true,
            common::acvm::acir::BlackBoxFunc::SHA256 => true,
            common::acvm::acir::BlackBoxFunc::Blake2s => true,
            common::acvm::acir::BlackBoxFunc::MerkleMembership => true,
            common::acvm::acir::BlackBoxFunc::SchnorrVerify => true,
            common::acvm::acir::BlackBoxFunc::Pedersen => true,
            common::acvm::acir::BlackBoxFunc::HashToField128Security => true,
            common::acvm::acir::BlackBoxFunc::EcdsaSecp256k1 => true,
            common::acvm::acir::BlackBoxFunc::FixedBaseScalarMul => true,
            common::acvm::acir::BlackBoxFunc::Keccak256 => false,
        }
    }

    fn preprocess(&self, circuit: &Circuit) -> (Vec<u8>, Vec<u8>) {
        let constraint_system = serialize_circuit(circuit);
        let mut composer = StandardComposer::new(constraint_system);

        let proving_key = composer.compute_proving_key();
        let verification_key = composer.compute_verification_key(&proving_key);

        (proving_key, verification_key)
    }

    fn prove_with_pk(
        &self,
        circuit: &Circuit,
        witness_values: BTreeMap<Witness, FieldElement>,
        proving_key: &[u8],
    ) -> Vec<u8> {
        let constraint_system = serialize_circuit(circuit);

        let mut composer = StandardComposer::new(constraint_system);

        let assignments = proof::flatten_witness_map(circuit, witness_values);

        composer.create_proof_with_pk(assignments, proving_key)
    }

    fn verify_with_vk(
        &self,
        proof: &[u8],
        public_inputs: BTreeMap<Witness, FieldElement>,
        circuit: &Circuit,
        verification_key: &[u8],
    ) -> bool {
        let constraint_system = serialize_circuit(circuit);
        let mut composer = StandardComposer::new(constraint_system);

        // Unlike when proving, we omit any unassigned witnesses.
        // Witness values should be ordered by their index but we skip over any indices without an assignment.
        let flattened_public_inputs = public_inputs.into_values().collect();

        composer.verify_with_vk(
            proof,
            Assignments::from_vec(flattened_public_inputs),
            verification_key,
        )
    }
}
