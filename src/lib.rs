// 1, 2, 3 ...
// 'laks': 'Do you like me?'
// choice: "1 ('Yes')" | "2 ('No')" | "3 ('Kill Laks')"  | "4 ('Do nothing')"
// player: "1"
// goto "CHAPTER '2'" => "branch '1'"

#[derive(Debug)]
pub struct Token {
    pub keyword: String,
    pub value: Vec<String>,
    pub line: usize,
    start: usize,
}

impl Token {
    pub fn new(keyword: String, value: Vec<String>, line: usize, start: usize) -> Token {
        Token {
            keyword,
            value,
            line,
            start
        }
    }
}

pub fn find_branch(line: usize, tokens: std::slice::Iter<'_, Token>, number: usize) -> Vec<&Token> {
    let mut found_b_r_a_n_c_h = Vec::new();
    for n in tokens {
        if n.line >= line {
            if n.keyword == "BRANCH" || n.keyword == "가지" {
                if number.to_string() != n.value[0] {
                    break;
                }
            }
            found_b_r_a_n_c_h.push(n);
        }
    }
    return  found_b_r_a_n_c_h;
}

pub fn find_line(tokens: std::slice::Iter<'_, Token>, choice: usize) -> usize {
    let mut line = 0;
    for x in tokens {
        if (x.keyword == "BRANCH" || x.keyword == "가지") && x.value[0] == choice.to_string() { 
            line = x.line;
        }
    }
    return line;
}