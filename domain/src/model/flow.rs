//!
//! This modules brings in the `Flow` concept and logic around it.
//!

use super::Step;

#[derive(Debug)]
pub struct Flow {
    pub name: String,
    pub description: String,
    pub steps: Vec<Step>,
}

impl Flow {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            steps: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Flow;

    #[test]
    fn basics() {
        let name = "someName";
        let desc = "someDesc";
        let flow = Flow::new(name.into(), desc.into());
        assert_eq!(flow.name, name);
        assert_eq!(flow.description, desc);
        assert_eq!(flow.steps.len(), 0)
    }
}
