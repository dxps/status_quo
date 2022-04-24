//!
//! This modules brings in the `Step` concept and logic around it.
//!

#[derive(Debug)]
pub struct Step {
    pub name: String,
    // to-be-cont'd
}

impl Step {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Step;

    #[test]
    fn basics() {
        let name = "some1";
        let step = Step::new(name.into());
        assert_eq!(step.name, name);
    }
}
