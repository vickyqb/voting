
#[update]
fn create_candidate(name: String) {
    CANDIDATES.with(|candidates| {
        let mut candidates = candidates.borrow_mut();
        candidates.push(Candidate {
            name,
            votes: 0,
        });
    });
}
#[update]
fn create_voter(name: String) {
    VOTERS.with(|voters| {
        let mut voters = voters.borrow_mut();
        voters.push(Voter {
            name,
            voted: false,
        });
    });
}

#[query]
fn get_candidates() -> Vec<Candidate> {
    CANDIDATES.with(|candidates| {
        let candidates = candidates.borrow();
        candidates.clone()
    })
}
#[query]
fn get_voters() -> Vec<Voter> {
    VOTERS.with(|voters| {
        let voters = voters.borrow();
        voters.clone()
    })
}

#[update]
fn vote_to_candidate(voter_name: String, to: String) -> String {
    let mut result = "Voted".to_string();
    CANDIDATES.with(|candidates| {
        let mut candidates = candidates.borrow_mut();
        VOTERS.with(|voters| {
            let mut voters = voters.borrow_mut();
            if let Some(voter) = voters.iter_mut().find(|v| v.name == voter_name) {
                if voter.voted {
                    result = "Already voted".to_string();
                    return;
                }
                if let Some(candidate) = candidates.iter_mut().find(|c| c.name == to) {
                    candidate.votes += 1;
                    voter.voted = true;
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

#[query]
fn get_winner() -> Option<Candidate> {
    CANDIDATES.with(|candidates| {
        let candidates = candidates.borrow();
        candidates.iter().max_by_key(|c| c.votes).cloned()
    })
}

