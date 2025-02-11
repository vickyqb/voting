use ic_cdk::{export_candid, query, update};
use candid::{CandidType, Decode, Encode };
use serde::Deserialize;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{
    storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable,
};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;


#[derive(CandidType, Deserialize, Clone)]
pub struct Candidate {
    name: String,
    votes: u32,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Voter{
    name: String,
    voted: bool,
}

impl Storable for Voter {
    const BOUND: Bound = Bound::Unbounded ;
    

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
        // let serialized = serde_cbor::to_vec(self).expect("Failed to serialize Benefactors");
        // Cow::Owned(serialized)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
        // serde_cbor::from_slice(&bytes).expect("Failed to deserialize Benefactors")
    }

}

impl Storable for Candidate {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

}


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Initialize a `StableBTreeMap` with `MemoryId(0)`.
    static CANDIDATES: RefCell<StableBTreeMap<String, Candidate, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
    // Initialize a `StableBTreeMap` with `MemoryId(0)`.
    static VOTERS: RefCell<StableBTreeMap<String, Voter, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}


#[update]
fn create_candidate(name: String) -> String {
    if name.is_empty() {
        return "Name cannot be empty".to_string();
    }
    let mut result = "creating".to_string();
    CANDIDATES.with(|candidates| {
        let mut candidates = candidates.borrow_mut();
        if !candidates.contains_key(&name) {
            candidates.insert(name.clone(), Candidate {
                name: name.clone(),
                votes: 0,
            });
            result = "created".to_string();
        }else{
            result = "already exists".to_string();
        }
    });
    result
}

#[update]
fn create_voter(name: String) -> Result<String, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    };
    VOTERS.with(|voters| {
        let mut voters = voters.borrow_mut();
        if !voters.contains_key(&name) {
            voters.insert(name.clone(), Voter {
                name: name.clone(),
                voted: false,
            });
            return Ok("created".to_string());
        }else{
            return Err("already exists".to_string());
        }
    });
    return Ok("created".to_string());
}

#[query]
fn get_candidates() -> Vec<Candidate> {
    let mut result = Vec::new();
    CANDIDATES.with(|candidates| {
        let candidates = candidates.borrow();
        for (_, candidate) in candidates.iter() {
            result.push(candidate.clone());
        }
    });
    result
}

#[query]
fn get_voters() -> Vec<Voter> {
    let mut result = Vec::new();
    VOTERS.with(|voters| {
        let voters = voters.borrow();
        for (_, voter) in voters.iter() {
            result.push(voter.clone());
        }
    });
    result
}

#[query]
fn get_winner() -> Candidate {
    let mut result = Candidate {
        name: "".to_string(),
        votes: 0,
    };
    CANDIDATES.with(|candidates| {
        let candidates = candidates.borrow();
        for (_, candidate) in candidates.iter() {
            if candidate.votes > result.votes {
                result = candidate.clone();
            }
        }
    });
    result
}

#[update]
fn vote_to_candidate(voter_name: String, to: String) -> String {
    if voter_name.is_empty() || to.is_empty() {
        return "Both fields required".to_string();
    }
    let mut result = "Voting in process".to_string();
    CANDIDATES.with(|candidates| {
        let mut candidates = candidates.borrow_mut();
        VOTERS.with(|voters| {
            let mut voters = voters.borrow_mut();
            if let Some(mut voter) = voters.get(&voter_name) {
                if voter.voted {
                    result = "Already voted".to_string();
                    return;
                }
                if let Some(mut candidate) = candidates.get(&to) {
                    candidate.votes += 1;
                    candidates.insert(to.clone(), candidate); // Save updated candidate
                    voter.voted = true;
                    voters.insert(voter_name.clone(), voter); // Save updated voter
                    result = "Voted success".to_string();
                } else {
                    result = "Candidate not found".to_string();
                }
            } else {
                result = "Voter not found".to_string();
            }
        });
    });
    result
}
export_candid!();