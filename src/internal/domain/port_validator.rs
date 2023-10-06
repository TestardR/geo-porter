use super::port::Port;

#[derive(Debug, PartialEq)]
pub struct Violation(String);
type ValidationResult = Result<(), Vec<Violation>>;

pub struct PortValidator;

impl PortValidator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn validate(&self, port: Port) -> ValidationResult {
        let mut violations: Vec<Violation> = vec![];

        if port.name() == "" {
            violations.push(Violation("name is required".to_string()));
        }

        if port.city() == "" {
            violations.push(Violation("city is required".to_string()));
        }

        if violations.len() > 0 {
            Err(violations)
        } else {
            Ok(())
        }
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
            "".to_string()
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
            "".to_string()
        );

        let port_validator = PortValidator::new();
        let expected_violations = vec![Violation("name is required".to_string()),Violation("city is required".to_string())];

        assert_eq!(port_validator.validate(port), Err(expected_violations))
    }
}