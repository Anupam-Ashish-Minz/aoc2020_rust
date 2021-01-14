use std::fs;
use std::collections::HashMap;

pub fn read_input_from_file() -> String {
    const FILEPATH: &str = "input/day7.input";
    let input = fs::read_to_string(FILEPATH).unwrap();
    return input;
}

pub type GraphValuesType<T> = Vec<(usize, T)>;
pub type GraphType<T> = HashMap<T, GraphValuesType<T>>;

pub fn normalize_input(input: String) -> GraphType<String> {
    let mut groups: GraphType<String> = HashMap::new();
    input
        .split("\n")
        .for_each(|x| {
            let x = remove_unwanted_chars(x.to_string());
            let (primary_bag_color, secondary_bags) = split_by_keyword(x);
            let secondary_bags = split_by_comma(secondary_bags);
            let secondary_bags: Vec<(usize, String)> = secondary_bags
                .iter()
                .map(|bag| {
                    let (num_of_bags, color_of_bags) = separate_string_and_number(bag.to_string());
                    return (num_of_bags, color_of_bags);
                })
            .collect();
            groups.insert(primary_bag_color, secondary_bags);
        });
    return groups;
}

fn split_by_keyword(input: String) -> (String, String) {
    let keyword = "contain";
    let split_pos = input.find(keyword).unwrap_or_default();
    let a = input[..split_pos].trim().to_string();
    let b = input.get((split_pos+7)..);
    let b = match b {
        Some(x) => x,
        None => {
            ""
        }}.trim().to_string();
    return (a, b)
}

fn split_by_comma(input: String) -> Vec<String> {
    input
        .split(",")
        .map(|x| x.trim().to_string())
        .collect()
}

fn remove_unwanted_chars(input: String) -> String {
    let input = input.replace("bags", "").trim().to_string();
    let input = input.replace("bag", "").trim().to_string();
    let input = input.replace(".", "").trim().to_string();
    return input
}

fn separate_string_and_number(input: String) -> (usize, String) {
    let split_pos = input.find(" ").unwrap_or_default();
    let number = input[..split_pos].trim().parse::<usize>().unwrap_or_default();
    let string_portion = input[split_pos..].trim().to_string();
    return (number, string_portion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separate_string_and_number() {
        let mock_input = "3 bright white bags".to_string();
        let expected_ouput = (3 as usize, "bright white bags".to_string());
        assert_eq!(separate_string_and_number(mock_input), expected_ouput);
    }

    #[test] 
    fn test_split_by_comma() {
        let mock_input = "3 bright white bags, 4 muted yellow bags".to_string();
        let expected_output = vec!["3 bright white bags", "4 muted yellow bags"];
        assert_eq!(split_by_comma(mock_input), expected_output);
    }

    #[test]
    fn test_split_by_keyword() {
        let mock_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string();
        let expected_output = ("light red bags".to_string(), "1 bright white bag, 2 muted yellow bags.".to_string());
        assert_eq!(split_by_keyword(mock_input), expected_output);
    }

    #[test]
    fn test_normalize_input() {
        let raw_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.";
        let parsed_input: GraphType<String> = normalize_input(raw_input.to_string());

        let actual_output0 = parsed_input.get("light red").unwrap();
        let expected_output0 = vec![
            (1 as usize, "bright white".to_string()),
            (2 as usize, "muted yellow".to_string())
        ];

        let actual_output8 = parsed_input.get("faded blue").unwrap();
        let expected_output8 = vec![
            (0 as usize, "other".to_string())
        ];

        assert_eq!(actual_output0, &expected_output0);
        assert_eq!(actual_output8, &expected_output8);
    }
}
