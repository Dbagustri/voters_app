#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec, Map
};

// =======================
// STRUCT
// =======================

#[contracttype]
#[derive(Clone, Debug)]
pub struct Poll {
    pub id: u64,
    pub title: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
}

// =======================
// STORAGE KEY (<= 9 CHAR)
// =======================

const POLL: Symbol = symbol_short!("POLL");
const VOTER: Symbol = symbol_short!("VOTER");

// =======================
// CONTRACT
// =======================

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // =======================
    // GET ALL POLLS
    // =======================
    pub fn get_polls(env: Env) -> Vec<Poll> {
        env.storage()
            .instance()
            .get(&POLL)
            .unwrap_or(Vec::new(&env))
    }

    // =======================
    // CREATE POLL
    // =======================
    pub fn create_poll(env: Env, title: String, options: Vec<String>) -> String {
        let mut polls: Vec<Poll> = env.storage()
            .instance()
            .get(&POLL)
            .unwrap_or(Vec::new(&env));

        // validasi minimal 2 opsi
        if options.len() < 2 {
            return String::from_str(&env, "Minimal 2 opsi");
        }

        // inisialisasi vote = 0
        let mut votes = Vec::new(&env);
        for _ in 0..options.len() {
            votes.push_back(0);
        }

        let poll = Poll {
            id: env.prng().gen::<u64>(),
            title,
            options,
            votes,
        };

        polls.push_back(poll);
        env.storage().instance().set(&POLL, &polls);

        String::from_str(&env, "Polling berhasil dibuat")
    }

    // =======================
    // VOTE
    // =======================
    pub fn vote(env: Env, poll_id: u64, option_index: u32, voter: String) -> String {
        let mut polls: Vec<Poll> = env.storage()
            .instance()
            .get(&POLL)
            .unwrap_or(Vec::new(&env));

        let mut voters: Map<String, Vec<u64>> = env.storage()
            .instance()
            .get(&VOTER)
            .unwrap_or(Map::new(&env));

        // cek sudah vote
        if let Some(voted) = voters.get(voter.clone()) {
            for i in 0..voted.len() {
                if voted.get(i).unwrap() == poll_id {
                    return String::from_str(&env, "Sudah vote");
                }
            }
        }

        // cari poll
        for i in 0..polls.len() {
            let mut poll = polls.get(i).unwrap();

            if poll.id == poll_id {

                // validasi index
                if option_index >= poll.options.len() {
                    return String::from_str(&env, "Opsi tidak valid");
                }

                // tambah vote
                let current = poll.votes.get(option_index).unwrap();
                poll.votes.set(option_index, current + 1);

                polls.set(i, poll);

                // simpan voter
                let mut voted_list = voters.get(voter.clone()).unwrap_or(Vec::new(&env));
                voted_list.push_back(poll_id);
                voters.set(voter, voted_list);

                env.storage().instance().set(&POLL, &polls);
                env.storage().instance().set(&VOTER, &voters);

                return String::from_str(&env, "Vote berhasil");
            }
        }

        String::from_str(&env, "Polling tidak ditemukan")
    }

    // =======================
    // GET RESULT
    // =======================
    pub fn get_result(env: Env, poll_id: u64) -> Vec<u32> {
        let polls: Vec<Poll> = env.storage()
            .instance()
            .get(&POLL)
            .unwrap_or(Vec::new(&env));

        for i in 0..polls.len() {
            let poll = polls.get(i).unwrap();
            if poll.id == poll_id {
                return poll.votes;
            }
        }

        Vec::new(&env)
    }
}

mod test;