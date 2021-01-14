use handle_input::read_input_from_file;
use handle_input::normalize_input;
use handle_input::GraphType;

mod handle_input;

#[allow(dead_code)]
pub fn run() {
    let input: String = read_input_from_file();
    let count = count_possible_container_bags(input.clone());
    println!("count = {}", count);
    let graph = normalize_input(input);
    let start_node = "shiny gold";
    let count = count_bags_inside_bags(&graph, start_node, 1).unwrap();
    let count = count - 1;
    println!("part 2 count = {}", count);
}


fn traverse_graph(graph: &GraphType<String>, visited_nodes: &mut Vec<String>, start_node: &str, goal: &str, depth: usize) -> Option<usize> {
    let has_node_been_visited: bool = match visited_nodes.iter().find(|node| node==&start_node) {
        Some(_) => true,
        None => false
    };
    if has_node_been_visited == false {
        let child_nodes = graph.get(start_node);
        visited_nodes.push(start_node.to_string());
        match child_nodes {
            Some(child_nodes) => {
                let child_nodes = child_nodes.iter();
                for node in child_nodes {
                    let (_num_of_bags, bag_color) = node;
                    if &bag_color == &goal {
                        return Some(depth+1);
                    }
                    match traverse_graph(graph, visited_nodes, bag_color, goal, depth+1) {
                        Some(value) => {
                            return Some(value)
                        },
                        None => {}
                    }
                }
            },
            None => {}
        };
    } 
    return None;
}

fn count_possible_container_bags(raw_input: String) -> usize {
    let graph = normalize_input(raw_input);
    let goal = "shiny gold";
    let depth = 0;
    let mut count = 0;
    graph.iter().for_each(|current_node| {
        let (key, _value) = current_node;
        let start_node = key;
        let mut visited_nodes: Vec<String> = vec!();
        let i = traverse_graph(&graph, &mut visited_nodes, start_node, goal, depth);
        match i {
            None => {},
            Some(_) => { count+=1 }
        };
    });
    return count;
}


fn count_bags_inside_bags(graph: &GraphType<String>, start_node: &str, num_of_bags_parent: usize) -> Option<usize> {
    let child_nodes = graph.get(start_node);
    let mut total_child_bag_count: usize = 0;
    match child_nodes {
        Some(child_nodes) => {
            for node in child_nodes {
                let (num_of_bags, bag_color) = node;
                let child_bag_count = count_bags_inside_bags(graph, bag_color, *num_of_bags);
                total_child_bag_count += child_bag_count.unwrap_or(0);
            }
            return Some(total_child_bag_count * num_of_bags_parent + num_of_bags_parent)
        },
        None => {}
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse_graph() {
        let raw_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.";
        let graph = normalize_input(raw_input.to_string());
        let start_node = "light red";
        let goal = "shiny gold";
        let depth = 0;
        let mut visited_nodes: Vec<String> = vec!();
        let actual_output = traverse_graph(&graph, &mut visited_nodes, start_node, goal, depth).unwrap();
        assert_eq!(2, actual_output);
    }

    #[test]
    fn test_count_possible_container_bags() {
        let raw_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.".to_string();
        let count = count_possible_container_bags(raw_input);
        assert_eq!(4, count);
    }

    #[test]
    fn test_count_bags_inside_bags() {
        let raw_input = "shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.".to_string();
        let graph = normalize_input(raw_input);
        let start_node = "shiny gold";
        let count = count_bags_inside_bags(&graph, start_node, 1).unwrap();
        // substract one to remove the shiny gold bag
        let count = count - 1;
        assert_eq!(126, count);

        let raw_input2 = "shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags, 2 dark green bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain no other bag.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.".to_string();
        let graph = normalize_input(raw_input2);
        let count = count_bags_inside_bags(&graph, start_node, 1).unwrap();
        let count = count;
        assert_eq!(43, count);
    }
}

