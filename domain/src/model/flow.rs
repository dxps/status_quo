//!
//! This modules brings in the `Flow` concept and logic around it.
//!

#[derive(Debug)]
pub struct Flow {
    pub name: String,
    pub description: String,
    // to-be-cont'd
}

impl Flow {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Flow;

    #[test]
    fn basics() {
        let name = "someName";
        let desc = "someDesc";
        let step = Flow::new(name.into(), desc.into());
        assert_eq!(step.name, name);
        assert_eq!(step.description, desc);
    }
}
