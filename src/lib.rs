use std::ops::{RangeInclusive};

use nom::{*, bytes::complete::tag, character::complete};

pub fn process_part_2(input: &str) -> usize {
    let total_part_overlapped = input.lines()
        .map(|line| get_section_assignement_ranges(&line))
        .map(|ranges| are_ranges_partially_overlapped(&ranges.0, &ranges.1))
        .filter(|part_overlap| *part_overlap)
        .count();
    total_part_overlapped
}

pub fn process_part_1(input: &str) -> usize {
    let total_overlapped = input.lines()
        .map(|line| get_section_assignement_ranges(line))
        .map(|ranges| are_ranges_fully_overlapped(&ranges.0, &ranges.1))
        .filter(|is_overlapped| *is_overlapped)
        .count();
    total_overlapped
}

fn get_section_assignement_ranges(section: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let assignments: Vec<&str> = section.split(",").take(2).collect();
    let first_assignment = get_assignment_range(assignments[0]).unwrap().1;
    let second_assignment = get_assignment_range(assignments[1]).unwrap().1;
    (first_assignment, second_assignment)
}

fn get_assignment_range(assignment: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = complete::u32(assignment)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = complete::u32(input)?;
    Ok((input, start..=end))
}

fn are_ranges_fully_overlapped(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    (range1.start() <= range2.start() && range1.end() >= range2.end()) ||
    (range2.start() <= range1.start() && range2.end() >= range1.end())
}

fn are_ranges_partially_overlapped(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    range1.contains(&range2.start()) || range1.contains(&range2.end()) ||
    range2.contains(&range1.start()) || range2.contains(&range1.end())
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_process_2() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let total = process_part_2(&input);
        assert_eq!(total, 4);
    }

    #[test]
    fn test_are_ranges_fully_overlapped() {
        let ranges = (0..=4, 1..=2);
        assert!(are_ranges_fully_overlapped(&ranges.0, &ranges.1));

        let ranges = (0..=4, 1..=8);
        assert!(!are_ranges_fully_overlapped(&ranges.0, &ranges.1));

        let ranges = (4..=4, 0..=8);
        assert!(are_ranges_fully_overlapped(&ranges.0, &ranges.1));

        let ranges = (5..=6, 1..=5);
        assert!(!are_ranges_fully_overlapped(&ranges.0, &ranges.1));
    }

    #[test]
    fn test_get_assignment_range() {
        let input = "2-4";
        let range = get_assignment_range(&input).unwrap().1;
        assert_eq!(range, 2..=4);

        let input = "3-8";
        let range = get_assignment_range(&input).unwrap().1;
        assert_eq!(range, 3..=8);
    }

    #[test]
    fn test_get_section_assignment_ranges() {
        let section_input = fs::read_to_string("test-section-input.txt").unwrap();
        let section_ranges = get_section_assignement_ranges(&section_input);
        assert_eq!((2..=4, 6..=8), section_ranges);
    }
    
    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let total = process_part_1(&input);
        assert_eq!(total, 2);
    }
}