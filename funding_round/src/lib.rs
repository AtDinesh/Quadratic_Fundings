use project::*;
use std::collections::HashMap;

#[allow(dead_code)]

// Define a struct that represent a funding round
#[derive(Debug)]
pub struct FundingRound {
    projects_list: HashMap<u32, Project>,
    matching_pool: f64,
}

impl FundingRound {

    pub fn new() -> FundingRound {
        FundingRound {
            projects_list: HashMap::new(),
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
        if self.projects_list.contains_key(&proj.get_id()) {
            panic!("Trying to insert a project with key already contained in project_list !");
        }
        self.projects_list.insert(proj.get_id(), proj);
    }

    pub fn add_contribution(&mut self, contrib: Contribution) {
        if !self.projects_list.contains_key(&contrib.to) {
            panic!("The project list does not contain the requested key");
        }

        self.projects_list.get_mut(&contrib.to).unwrap().add_contribution(contrib);
    }

    pub fn update_projects(&mut self) {
        for (_id, project) in self.projects_list.iter_mut() {
            project.update();
        }
    }

    // Capital Constrained Quadratic Funding
    // Compute the CQF matching amount for the project.
    // The initial allocated amount is computed independently for each project using QF method
    // If the total allocation is > to matching pool M, then the initial allocation is 'normalized'
    // through multiplying with M/(sum of all initial allocations)
    pub fn compute_cqf_allocation(&mut self) -> HashMap<u32, f64> {
        if self.matching_pool <= 0.0 {
            panic!("Matching pool not set");
        }
        
        let mut projects_final_funding = HashMap::new();
        // initialize the map with qf-based fundings
        for (id, project) in self.projects_list.iter() {
            projects_final_funding.insert(*id, project.get_matching_amount());
        }

        let sum_matching_amount : f64 = self.projects_list.iter().map(|(_id, project)| project.get_matching_amount()).sum();
        if sum_matching_amount <= self.matching_pool {
            return projects_final_funding; // i.e. standard qf can be funded totally
        }

        // Otherwise, the pool is not enough to fund totally the projects
        for (_,fund) in projects_final_funding.iter_mut() {
            *fund = *fund * self.matching_pool.clone()/sum_matching_amount;
        }

        // projects_final_funding.iter_mut().map(|(key, value)| (key.clone(), *value * self.matching_pool.clone()/sum_matching_amount));
        return projects_final_funding;
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

    #[test]
    #[should_panic]
    fn test_invalid_add_project_to_funding_roung() {
        let mut round = FundingRound::new();
        let proj0 = Project::new(0);
        let proj0_bis = Project::new(0);

        assert_eq!(0, round.projects_list.len());
        round.add_project(proj0);
        assert_eq!(1, round.projects_list.len());
        round.add_project(proj0_bis); // try to insert another project with same id 0
    }

    #[test]
    fn test_add_contribution() {
        let mut round = FundingRound::new();
        let proj = Project::new(1);
        round.add_project(proj);

        let contrib = Contribution{from: 0, to: 1, amount: 10.0};
        round.add_contribution(contrib);

        // project should now have 1 contribution
        // Other fields should not be updated yet
        let p : Project = round.projects_list.get(&1).unwrap().clone();
        let contribs: HashMap<u32, f64> = p.get_contribution_list();
        assert_eq!(1, p.get_id());
        assert_eq!(1, contribs.len());
    }

    #[test]
    fn test_update_project() {
        let mut round = FundingRound::new();
        let proj = Project::new(1);
        round.add_project(proj);

        let contrib = Contribution{from: 0, to: 1, amount: 10.0};
        round.add_contribution(contrib);

        // project should now have 1 contribution
        // Other fields should not be updated yet
        let p : Project = round.projects_list.get(&1).unwrap().clone();
        let contribs: HashMap<u32, f64> = p.get_contribution_list();
        assert_eq!(1, p.get_id());
        assert_eq!(1, contribs.len());
        assert_eq!(0.0, p.get_total_contribution());
        assert_eq!(0.0, p.get_sum_rootsquared_contribution());
        assert_eq!(0.0, p.get_matching_amount());
        
        round.update_projects();
        // Now other fields should be updated
        let p_updated : Project = round.projects_list.get(&1).unwrap().clone();
        assert_eq!(contrib.amount, p_updated.get_total_contribution());
        assert_eq!(contrib.amount.clone().sqrt(), p_updated.get_sum_rootsquared_contribution());
    }

    #[test]
    fn test_compute_cqf_allocation_keep_qf() {
        // create the funding round
        let mut round = FundingRound::new();
        round.set_matching_pool(100.0);
        // create projects for the round
        let proj0 = Project::new(0);
        let proj1 = Project::new(1);
        round.add_project(proj0);
        round.add_project(proj1);

        let contrib0 = Contribution{from: 10, to: 0, amount: 5.0};
        let contrib1 = Contribution{from: 10, to: 1, amount: 6.0};

        round.add_contribution(contrib0);
        round.add_contribution(contrib1);
        round.update_projects();

        // project0 has total contribution of 5 -> matching_fund is 5²=25
        // project1 has total contribution of 6 -> matching fund is 6²=36
        // sum of matching fund < matching pool.
        // We shall keep the qf-based matching allocation.

        let matching_alloc : HashMap<u32, f64> = round.compute_cqf_allocation();
        assert_eq!(&round.projects_list.get(&0).unwrap().get_matching_amount(), matching_alloc.get(&0).unwrap());
        assert_eq!(&round.projects_list.get(&1).unwrap().get_matching_amount(), matching_alloc.get(&1).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_compute_cqf_allocation_keep_qf_pool_unset() {
        // create the funding round
        let mut round = FundingRound::new();
        
        // create projects for the round
        let proj0 = Project::new(0);
        round.add_project(proj0);

        let contrib0 = Contribution{from: 10, to: 0, amount: 5.0};

        round.add_contribution(contrib0);
        round.update_projects();

        let _matching_alloc : HashMap<u32, f64> = round.compute_cqf_allocation();
    }
}