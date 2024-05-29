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

    pub fn set_matching_pool(&mut self, fund: f64) {
        if fund <= 0.0 {
            panic!("Setting negative matching pool !");
        }
        self.matching_pool = fund;
    }

    pub fn add_project(&mut self, proj: Project) {
        self.projects_list.push(proj);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_matching_pool() {
        let mut funding_round = FundingRound::new();
        assert_eq!(0.0, funding_round.matching_pool);
        funding_round.set_matching_pool(100.0);
        assert_eq!(100.0, funding_round.matching_pool);
    }

    #[test]
    fn test_add_project() {
        let proj = Project::new(1);
        let mut round = FundingRound::new();
        round.add_project(proj);

        assert_eq!(1, round.projects_list.len());
    }
}