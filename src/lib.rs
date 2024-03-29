use std::{convert::Infallible, str::FromStr};

#[derive(Debug, PartialEq)]
enum Bracket {
    Round,
    Square,
    Curly,
}

#[derive(Debug, PartialEq)]
enum Type {
    Open(Bracket),
    Close(Bracket),
    Other,
}

impl From<char> for Type {
    fn from(input: char) -> Self {
        match input {
            '[' => Type::Open(Bracket::Square),
            ']' => Type::Close(Bracket::Square),
            '{' => Type::Open(Bracket::Curly),
            '}' => Type::Close(Bracket::Curly),
            '(' => Type::Open(Bracket::Round),
            ')' => Type::Close(Bracket::Round),
            _ => Type::Other,
        }
    }
}

struct BracketSequence(Vec<Type>);

impl FromStr for BracketSequence {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Infallible> {
        Ok(BracketSequence(
            s.chars().map(|c| c.into()).collect::<Vec<Type>>(),
        ))
    }
}

impl BracketSequence {
    pub fn verify(self) -> bool {
        let mut stack: Vec<Bracket> = Vec::new();

        for c in self.0 {
            match c {
                Type::Open(c) => stack.push(c),
                Type::Close(c) => match stack.last() {
                    Some(s) if &c == s => {
                        stack.pop();
                    }
                    _ => return false,
                },
                Type::Other => continue,
            }
        }

        stack.is_empty()
    }
}

/// Check to see if an input text is a Balance set of bracket
///
/// ```rust
/// assert!(balanced_brackets::is_balanced("([{}({}[])])"));
/// ```
pub fn is_balanced(input: &str) -> bool {
    match input.parse::<BracketSequence>() {
        Ok(brackets) => brackets.verify(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_char() {
        assert_eq!(Type::from('['), Type::Open(Bracket::Square));
        assert_eq!(Type::from(']'), Type::Close(Bracket::Square));
        assert_eq!(Type::from('{'), Type::Open(Bracket::Curly));
        assert_eq!(Type::from('}'), Type::Close(Bracket::Curly));
        assert_eq!(Type::from('('), Type::Open(Bracket::Round));
        assert_eq!(Type::from(')'), Type::Close(Bracket::Round));
        assert_eq!(Type::from('a'), Type::Other);
    }

    #[test]
    fn paired_square_brackets() {
        assert!(is_balanced("[]"));
    }
    #[test]
    fn empty_string() {
        assert!(is_balanced(""));
    }
    #[test]
    fn unpaired_brackets() {
        assert!(!is_balanced("[["));
    }
    #[test]
    fn wrong_ordered_brackets() {
        assert!(!is_balanced("}{"));
    }
    #[test]
    fn wrong_closing_bracket() {
        assert!(!is_balanced("{]"));
    }
    #[test]
    fn paired_with_whitespace() {
        assert!(is_balanced("{ }"));
    }
    #[test]
    fn partially_paired_brackets() {
        assert!(!is_balanced("{[])"));
    }
    #[test]
    fn simple_nested_brackets() {
        assert!(is_balanced("{[]}"));
    }
    #[test]
    fn several_paired_brackets() {
        assert!(is_balanced("{}[]"));
    }
    #[test]
    fn paired_and_nested_brackets() {
        assert!(is_balanced("([{}({}[])])"));
    }
    #[test]
    fn unopened_closing_brackets() {
        assert!(!is_balanced("{[)][]}"));
    }
    #[test]
    fn unpaired_and_nested_brackets() {
        assert!(!is_balanced("([{])"));
    }
    #[test]
    fn paired_and_wrong_nested_brackets() {
        assert!(!is_balanced("[({]})"));
    }
    #[test]
    fn paired_and_incomplete_brackets() {
        assert!(!is_balanced("{}["));
    }
    #[test]
    fn too_many_closing_brackets() {
        assert!(!is_balanced("[]]"));
    }
    #[test]
    fn early_incomplete_brackets() {
        assert!(!is_balanced(")()"));
    }
    #[test]
    fn early_mismatched_brackets() {
        assert!(!is_balanced("{)()"));
    }
    #[test]
    fn math_expression() {
        assert!(is_balanced("(((185 + 223.85) * 15) - 543)/2"));
    }
    #[test]
    fn complex_latex_expression() {
        let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                     \\end{array}\\right)";
        assert!(is_balanced(input));
    }

    #[test]
    fn large_input() {
        let input = "(".repeat(100_000) + &")".repeat(100_000);
        assert!(is_balanced(&input));
    }
}
