#![allow(dead_code)]
use std::collections::HashMap;

// Define a struct to represent a contribution
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Contribution {
    from: u32,
    to: u32,
    amount: f64
}

// Define a struct to represent a project
#[derive(Debug)]
pub struct Project {
    id: u32,
    total_contribution: f64,
    sum_rootsquared_contribution: f64,
    matching_amount: f64,
    final_amount: f64,
    contribution_list: HashMap<u32, f64>,
}

impl Project {

    pub fn new(id: u32) -> Project {
        Project {
            id: id, 
            total_contribution: 0.0, 
            sum_rootsquared_contribution: 0.0, 
            matching_amount: 0.0, 
            final_amount: 0.0, 
            contribution_list: HashMap::new()
        }
    }

    // Add a contribution to the contribution vector.
    pub fn add_contribution(&mut self, contribution: Contribution) {
        // TODO: Check if the contributor already participated to this contribution (avoid easy Sybill)
        self.contribution_list.entry(contribution.from).and_modify(|v| {*v += contribution.amount}).or_insert(contribution.amount);
    }

    // Update the projects data with data contained in the vector of contributions.
    pub fn update(&mut self) {
        // reset values in case previous update occured
        self.total_contribution = 0f64;
        self.sum_rootsquared_contribution = 0f64;
        self.matching_amount = 0f64;
        self.final_amount = 0f64;

        for (_id, amount) in self.contribution_list.clone().into_iter() {
            self.total_contribution += amount;
            self.sum_rootsquared_contribution = self.sum_rootsquared_contribution + amount.clone().sqrt();
        }
        // Compute the QF marching amount for the project
        // In classic QF, the allocated amount for each project can be computed independently
        self.matching_amount = self.sum_rootsquared_contribution.powi(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_contribution() {
        let mut project0 = Project::new(0);
        let contrib = Contribution{from: 10, to: 0, amount: 100.0};
        project0.add_contribution(contrib);
        
        assert_eq!(0.0, project0.total_contribution);
        assert_eq!(0.0, project0.sum_rootsquared_contribution);
        assert_eq!(0.0, project0.matching_amount);
        assert_eq!(0.0, project0.final_amount);

        assert_eq!(1, project0.contribution_list.len());
        // Use Some() to get an Option<>, use cloned() to pass from &f64 to f64
        assert_eq!(Some(100f64), project0.contribution_list.get(&contrib.from).cloned());
    }

    #[test]
    fn test_update() {
        let mut project0 = Project::new(0);
        let contributor = 10u32;
        let contrib_amount = 100f64;
        let contrib = Contribution{from: contributor, to: 0, amount: contrib_amount};
        project0.add_contribution(contrib);
        project0.update();

        assert_eq!(contrib_amount, project0.total_contribution);
        // pow(sqrt(),2) = Id
        assert_eq!(contrib_amount.sqrt(), project0.sum_rootsquared_contribution);
        assert_eq!(contrib_amount, project0.matching_amount);
        // following parts are not updated by the fn
        assert_eq!(0.0, project0.final_amount);
    }
}