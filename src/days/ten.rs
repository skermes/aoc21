use crate::aoc_error::AocError;

pub const NAME: &str = "Syntax Scoring";

#[derive(Debug)]
enum CodeError {
    Corrupted(usize),
    Incomplete(usize)
}

fn is_open(token: char) -> bool {
    token == '(' || token == '[' || token == '{' || token == '<'
}

fn matching_tokens(open: char, close: char) -> bool {
    match (open, close) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _          => false
    }
}

fn corruption_token_score(token: char) -> usize {
    match token {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _   => 0
    }
}

fn incomplete_token_score(token: char) -> usize {
    match token {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _   => 0
    }
}

fn score_incomplete_stack(stack: Vec<char>) -> usize {
    let mut score = 0;

    for token in stack.iter().rev() {
        score = score * 5 + incomplete_token_score(*token);
    }

    score
}

fn find_error(code: &str) -> CodeError {
    let mut stack = Vec::new();

    for token in code.chars() {
        if is_open(token) {
            stack.push(token);
        } else {
            if let Some(top) = stack.pop() {
                if !matching_tokens(top, token) {
                    return CodeError::Corrupted(corruption_token_score(token));
                }
            } else {
                return CodeError::Incomplete(score_incomplete_stack(stack));
            }
        }
    }

    CodeError::Incomplete(score_incomplete_stack(stack))
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let score: usize = input
        .lines()
        .map(|l| match find_error(l) {
            CodeError::Corrupted(score) => score,
            CodeError::Incomplete(_) => 0
        })
        .sum();

    Ok(score.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut scores: Vec<usize> = input
        .lines()
        .map(|l| match find_error(l) {
            CodeError::Corrupted(_) => 0,
            CodeError::Incomplete(score) => score
        })
        .filter(|&score| score > 0)
        .collect();
    scores.sort();

    let score = scores[scores.len() / 2];

    Ok(score.to_string())
}