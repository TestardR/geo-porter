use super::port::Port;

#[derive(Debug, PartialEq)]
pub struct Violations {
    violation: Vec<String>
}

impl Violations {
    pub fn new(violation: Vec<String>) -> Self {
        Self { violation }
    }

    pub fn violation(&self) -> &Vec<String> {
        &self.violation
    }
}

pub struct PortValidator;

impl PortValidator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn validate(&self, port: Port) -> Result<(), Violations> {
        let mut violations= Violations::new(vec![]);

        if port.name() == "" {
            violations.violation.push("name is required".to_string());
        }

        if port.city() == "" {
            violations.violation.push("city is required".to_string());
        }

        if violations.violation().len() > 0 {
            return Err(violations);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::internal::domain::coordinates::Coordinates;
    use crate::internal::domain::port_id::PortId;
    use super::*;

    #[test]
    fn test_can_validate_valid_port() {
        let port = Port::new(
            PortId::new("AEFJR".to_string()),
            "Al Fujayrah".to_string(),
            "Al Fujayrah".to_string(),
            "United Arab Emirates".to_string(),
            vec![],
            vec![],
            Coordinates::new(25.12, 56.33),
            "Ajman".to_string(),
            "Asia/Dubai".to_string/**/(),
            vec!["AEFJR".to_string()],
            "".to_string(),
        );

        let port_validator = PortValidator::new();
        assert_eq!(port_validator.validate(port), Ok(()))
    }

    #[test]
    fn test_can_validate_invalid_port() {
        let port = Port::new(
            PortId::new("AEFJR".to_string()),
            "".to_string(),
            "".to_string(),
            "United Arab Emirates".to_string(),
            vec![],
            vec![],
            Coordinates::new(25.12, 56.33),
            "Ajman".to_string(),
            "Asia/Dubai".to_string(),
            vec!["AEFJR".to_string()],
            "".to_string(),
        );

        let port_validator = PortValidator::new();
        let expected_violations = Violations::new(vec![
            "name is required".to_string(),
            "city is required".to_string()
        ]);
        assert_eq!(port_validator.validate(port), Err(expected_violations))
    }
}