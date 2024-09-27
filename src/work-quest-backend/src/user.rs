use serde::{Deserialize, Serialize};
use candid::{CandidType, Principal};
use ic_cdk::api::call::call;
use ic_cdk::{update, query};

// Define the User struct
#[derive(Serialize, Deserialize, CandidType, Debug)]
struct User {
    id: u32,
    username: String,
    email: String,
    phone_number: String,
}

// Define the Job struct
#[derive(Serialize, Deserialize, CandidType, Debug)]
struct Job {
    id: u32,
    title: String,
    description: String,
}

// Register a new user
#[update]
async fn register_user(username: String, email: String, phone_number: String) -> Result<(u32,), String> {
    let user_id: Result<(u32,), String> = call(
        Principal::management_canister(),
        "register_user",
        (username, email, phone_number),
    )
    .await
    .map_err(|e| format!("Error registering user: {:?}", e)); // Handle errors properly
    user_id
}

// Post a new job
#[update]
async fn post_job(title: String, description: String) -> Result<(u32,), String> {
    let job_id: Result<(u32,), String> = call(
        Principal::management_canister(),
        "post_job",
        (title, description),
    )
    .await
    .map_err(|e| format!("Error posting job: {:?}", e)); // Handle errors properly
    job_id
}

// Get all jobs
#[query]
async fn get_jobs() -> Result<Vec<Job>, String> {
    let jobs: Result<Vec<Job>, String> = call(
        Principal::management_canister(),
        "get_jobs",
        (),
    )
    .await
    .map_err(|e| format!("Error fetching jobs: {:?}", e)); // Handle errors properly
    jobs
}

// Update a user's profile
#[update]
async fn update_user_profile(
    user_id: u32,
    username: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
) -> Result<(), String> {
    let result: Result<(), String> = call(
        Principal::management_canister(),
        "update_user_profile",
        (user_id, username, email, phone_number),
    )
    .await
    .map_err(|e| format!("Error updating profile: {:?}", e));
    result 
}
 