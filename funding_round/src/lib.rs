use project::*;

#[allow(dead_code)]

// Define a struct that represent a funding round
#[derive(Debug)]
pub struct FundingRound {
    projects_list: Vec<Project>,
    matching_pool: f64,
}

impl FundingRound {

    pub fn new() -> FundingRound {
        FundingRound {
            projects_list: Vec::new(),
            matching_pool: 0f64
        }
    }
}