const OPEN: [char; 3] = ['{', '[', '('];
const CLOSE: [char; 3] = ['}', ']', ')'];


fn get_closing_bracket(open: &char) -> char {
    let index = OPEN.iter().position(|&r| &r == open).unwrap();
    CLOSE[index]
}

pub fn is_balanced(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in input.chars() {
        if OPEN.contains(&c) {
            stack.push(c);
        } else if CLOSE.contains(&c) {
            if stack.is_empty() || get_closing_bracket(stack.last().unwrap()) != c {
                return false;
            }
            stack.pop();
        }
    }

    stack.is_empty()
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