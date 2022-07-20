#[derive(PartialEq, Eq)]
enum Bracket {
    Round,
    Square,
    Curly,
}

enum Type {
    Open(Bracket),
    Close(Bracket),
    Other(char),
}

fn parse(input: char) -> Type {
    match input {
        '[' => Type::Open(Bracket::Square),
        ']' => Type::Close(Bracket::Square),
        '{' => Type::Open(Bracket::Curly),
        '}' => Type::Close(Bracket::Curly),
        '(' => Type::Open(Bracket::Round),
        ')' => Type::Close(Bracket::Round),
        c => Type::Other(c),
    }
}

fn verify(input: Vec<Type>) -> bool {
    let mut stack: Vec<Bracket> = Vec::new();

    for c in input {
        match c {
            Type::Open(c) => stack.push(c),
            Type::Close(c) => match stack.last() {
                Some(s) if &c == s => {
                    stack.pop();
                }
                _ => return false,
            },
            Type::Other(_) => continue,
        }
    }

    stack.is_empty()
}

pub fn is_balanced(input: &str) -> bool {
    verify(input.chars().map(parse).collect::<Vec<Type>>())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
