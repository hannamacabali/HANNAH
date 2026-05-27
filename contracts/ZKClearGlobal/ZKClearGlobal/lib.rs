#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, BytesN};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComplianceKey {
    Auditor,             
    SolvencyProof(Address), 
    VerificationStatus(Address), 
    LastAuditTimestamp(Address), 
}

#[contract]
pub struct ZKClearGlobal;

#[contractimpl]
impl ZKClearGlobal {
    pub fn initialize(env: Env, auditor: Address) {
        if env.storage().instance().has(&ComplianceKey::Auditor) {
            panic!("Compliance registry structure already initialised");
        }
        env.storage().instance().set(&ComplianceKey::Auditor, &auditor);
    }

    pub fn commit_solvency_proof(env: Env, fintech: Address, proof_hash: BytesN<32>) {
        fintech.require_auth();

        env.storage().persistent().set(&ComplianceKey::SolvencyProof(fintech.clone()), &proof_hash);
        env.storage().persistent().set(&ComplianceKey::VerificationStatus(fintech.clone()), &false);
    }

    pub fn verify_fintech_solvency(env: Env, fintech: Address) {
        let auditor: Address = env.storage().instance().get(&ComplianceKey::Auditor).unwrap();
        auditor.require_auth();

        let proof_key = ComplianceKey::SolvencyProof(fintech.clone());
        if !env.storage().persistent().has(&proof_key) {
            panic!("No solvency commitment proof located for the designated fintech entity.");
        }

        env.storage().persistent().set(&ComplianceKey::VerificationStatus(fintech.clone()), &true);
        env.storage().persistent().set(&ComplianceKey::LastAuditTimestamp(fintech.clone()), &env.ledger().timestamp());
    }

    pub fn get_compliance_status(env: Env, fintech: Address) -> bool {
        env.storage().persistent().get(&ComplianceKey::VerificationStatus(fintech)).unwrap_or(false)
    }
}
