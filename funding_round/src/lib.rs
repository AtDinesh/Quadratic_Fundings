use project::*;

#[allow(dead_code)]

// Define a struct that represent a funding round
#[derive(Debug)]
pub struct FundingRound {
    projects_list: Vec<Project>,
    matching_pool: f64,
}