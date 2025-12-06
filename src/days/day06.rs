use crate::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Column {
    pub numbers: Vec<u64>,
    pub operator: Operator,
}

#[derive(Debug)]
pub struct Input {
    pub columns: Vec<Column>,
}

pub struct Day06;

impl Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        let parsed = parsing1::parse_input(input)
            .expect("Failed to parse input")
            .1;

        let mut sum = 0u64;
        for column in &parsed.columns {
            let result = match column.operator {
                Operator::Add => column.numbers.iter().sum::<u64>(),
                Operator::Multiply => column.numbers.iter().product::<u64>(),
            };
            sum += result;
        }

        format!(
            "Parsed {} columns, grand total {}",
            parsed.columns.len(),
            sum
        )
    }

    fn part2(&self, _input: &str) -> String {
        let parsed = parsing2::parse_input(_input).expect("Failed to parse input");

        let mut sum = 0u64;
        for column in &parsed.columns {
            let result = match column.operator {
                Operator::Add => column.numbers.iter().sum::<u64>(),
                Operator::Multiply => column.numbers.iter().product::<u64>(),
            };
            sum += result;
        }

        format!(
            "Parsed {} columns, grand total {}",
            parsed.columns.len(),
            sum
        )
    }
}

mod parsing1 {
    use super::{Column, Input, Operator};
    use nom::{
        IResult, Parser,
        character::complete::{digit1, one_of, space0, space1},
        combinator::map_res,
        multi::separated_list1,
        sequence::preceded,
    };
    fn parse_number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, |s: &str| s.parse::<u64>()).parse(input)
    }

    fn parse_number_row(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list1(space1, preceded(space0, parse_number)).parse(input)
    }

    fn parse_operator(input: &str) -> IResult<&str, Operator> {
        let (input, op) = preceded(space0, one_of("*+")).parse(input)?;
        let operator = match op {
            '*' => Operator::Multiply,
            '+' => Operator::Add,
            _ => unreachable!(),
        };
        Ok((input, operator))
    }

    fn parse_operator_row(input: &str) -> IResult<&str, Vec<Operator>> {
        separated_list1(space1, parse_operator).parse(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, Input> {
        let lines: Vec<&str> = input.lines().filter(|l| !l.trim().is_empty()).collect();

        // Parse all number rows except the last line
        let mut all_rows: Vec<Vec<u64>> = Vec::new();
        for i in 0..lines.len() - 1 {
            let (_, row) = parse_number_row(lines[i])?;
            all_rows.push(row);
        }

        // Parse operator row (last line)
        let (input, operators) = parse_operator_row(lines[lines.len() - 1])?;

        // Group numbers by column
        let num_cols = operators.len();
        let mut columns = Vec::new();

        for col_idx in 0..num_cols {
            let mut numbers = Vec::new();
            for row in &all_rows {
                if let Some(&num) = row.get(col_idx) {
                    numbers.push(num);
                }
            }
            columns.push(Column {
                numbers,
                operator: operators[col_idx],
            });
        }

        Ok((input, Input { columns }))
    }
}

mod parsing2 {
    use super::*;
    type Lines<'input, 'list> = &'list [&'input str];
    pub fn parse_number(input: Lines, col: usize) -> Option<u64> {
        let digits = input[..input.len() - 1]
            .iter()
            .filter_map(|line| {
                if col >= line.len() {
                    return None;
                }
                let c = line[col..].chars().next()?;
                if c.is_digit(10) { Some(c) } else { None }
            })
            .collect::<Vec<char>>();
        let s: String = digits.into_iter().collect();
        if s.is_empty() {
            None
        } else {
            s.parse::<u64>().ok()
        }
    }
    pub fn parse_operator(input: Lines, col: usize) -> Option<Operator> {
        let line = input.last()?;
        if col >= line.len() {
            return None;
        }
        let c = line[col..].chars().next()?;
        match c {
            '*' => Some(Operator::Multiply),
            '+' => Some(Operator::Add),
            _ => None,
        }
    }
    pub fn parse_column(input: Lines, mut col: usize) -> Option<(usize, Column)> {
        let operator = parse_operator(input, col)?;
        let mut numbers = Vec::new();
        loop {
            let res = parse_number(input, col);
            col += 1;
            match res {
                Some(num) => numbers.push(num),
                None => break,
            }
        }
        Some((col, Column { numbers, operator }))
    }
    pub fn parse_input(input: &str) -> Option<Input> {
        let lines: Vec<&str> = input.lines().collect();
        let line_len = lines.first()?.len();
        let mut columns = Vec::new();
        let mut col = 0;
        while let Some((next_col, column)) = parse_column(&lines, col) {
            columns.push(column);
            col = next_col;
            if col >= line_len {
                break;
            }
        }
        Some(Input { columns })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    #[test]
    fn test_parse2_sample() {
        let lines: Vec<&str> = INPUT.lines().collect();

        assert_eq!(parsing2::parse_number(&lines, 0), Some(1));
        assert_eq!(parsing2::parse_number(&lines, 1), Some(24));
        assert_eq!(parsing2::parse_number(&lines, 2), Some(356));
        assert_eq!(parsing2::parse_number(&lines, 3), None);

        assert_eq!(
            parsing2::parse_operator(&lines, 0),
            Some(Operator::Multiply)
        );
        assert_eq!(
            parsing2::parse_column(&lines, 0),
            Some((
                4,
                Column {
                    numbers: vec![1, 24, 356],
                    operator: Operator::Multiply,
                }
            ))
        );

        let parsed = parsing2::parse_input(INPUT).expect("Failed to parse input");
        assert_eq!(parsed.columns.len(), 4);
        assert_eq!(parsed.columns[2].numbers, vec![32, 581, 175]);
        assert_eq!(parsed.columns[2].operator, Operator::Multiply);
    }

    #[test]
    fn test_parse1_sample() {
        let parsed = parsing1::parse_input(INPUT)
            .expect("Failed to parse input")
            .1;

        assert_eq!(parsed.columns.len(), 4);

        assert_eq!(parsed.columns[0].numbers, vec![123, 45, 6]);
        assert_eq!(parsed.columns[0].operator, Operator::Multiply);

        assert_eq!(parsed.columns[1].numbers, vec![328, 64, 98]);
        assert_eq!(parsed.columns[1].operator, Operator::Add);

        assert_eq!(parsed.columns[2].numbers, vec![51, 387, 215]);
        assert_eq!(parsed.columns[2].operator, Operator::Multiply);

        assert_eq!(parsed.columns[3].numbers, vec![64, 23, 314]);
        assert_eq!(parsed.columns[3].operator, Operator::Add);
    }

    #[test]
    fn test_part1() {
        let solution = Day06;
        let result = solution.part1(INPUT);
        assert!(result.contains("4277556"));
    }

    #[test]
    fn test_part2() {
        let solution = Day06;
        let result = solution.part2(INPUT);
        assert!(result.contains("3263827"));
    }
}
