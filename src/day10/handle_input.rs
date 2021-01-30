use std::fs;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::iter::IntoIterator;

// Input 
#[derive(Debug)]
pub struct Input (
    VecDeque<usize>
);

// private functions
impl Input {
    fn parse_input(input: String) -> VecDeque<usize> {
        let input: Vec<usize> = input
            .split("\n")
            .filter_map(|x| x.trim().parse::<usize>().ok())
            .collect();
        return VecDeque::from(input);
    }
}

// functions that return an instance
#[allow(dead_code)]
impl Input {
    pub fn new() -> Self {
        Input (
            VecDeque::new()
        )
    }
    pub fn from(input: &str) -> Self {
        Input (
            Input::parse_input(input.to_string())
        )
    }
    pub fn read_input_from_file() -> Self {
        const FILEPATH: &str = "input/day10.input";
        let input = fs::read_to_string(FILEPATH).unwrap();
        Input (
            Input::parse_input(input)
        )
    }
}

// return computed results
#[allow(dead_code)]
impl Input {
    pub fn clone(&self) -> VecDeque<usize> {
        return self.0.clone();
    }
}

// mutable functions
impl Input {
    pub fn sort(&mut self) {
        let mut value: Vec<usize> = self.0.iter().map(|x| *x).collect();
        value.sort();
        self.0 = VecDeque::from(value);
    }
}


// conver to other types
#[allow(dead_code)]
impl Input {
    pub fn into_graph(self) -> Graph {
        let mut val: Vec<usize> = self.0.into();
        let mut g = Graph::new();
        g.insert_key(0);
        for j in &val {
            if j>&0 && j<=&3 {
                g.insert_value(0, *j);
            }
        }
        val.sort();
        for i in &val {
            g.insert_key(*i);
            // this algo is trash n^2 insted for 3*n .. plans to improve later
            for j in &val {
                if j>i && *j<=*i+3 {
                    g.insert_value(*i, *j);
                }
            }
        }
        return g;
    }
}

impl IntoIterator for Input {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let v: Vec<usize> = self.0.into();
        v.into_iter()
    }
}




// Graph
#[derive(Debug, PartialEq)]
pub struct Graph (
    HashMap<usize, Vec<usize>>
);

#[allow(dead_code)]
impl Graph {
    pub fn new() -> Self {
        Graph (
            HashMap::new()
        )
    }
}

// key values
impl Graph {
    pub fn insert_key(&mut self, key: usize) {
        self.0.insert(key, vec!());
    }
    pub fn insert_value(&mut self, key: usize, value: usize) {
        let mut val: Vec<usize> = self.0
            .get(&key)
            .expect("Key not found")
            .clone();
        val.push(value);
        self.0.insert(key, val);
    }
    pub fn insert_value_vec(&mut self, key: usize, values: Vec<usize>) {
        self.0.insert(key, values);
    }
    pub fn get_list_of_keys(self) -> Vec<usize> {
        return self.0.keys().map(|x| *x).collect();
    }
    pub fn get_value(self, key: usize) -> Option<Vec<usize>> {
        let val = self.0.get(&key)?;
        return Some(val.clone());
    }
}

// some other methods
#[allow(dead_code)]
impl Graph {
    pub fn clone(self) -> HashMap<usize, Vec<usize>> {
        return self.0.clone();
    }
    pub fn get_children(&self, node: usize) -> Option<Vec<usize>> {
        let children: Vec<usize> = self.0.get(&node)?.iter().map(|x| *x).collect();
        return Some(children);
    }
    pub fn bfs(&self, start: usize, end: usize) -> Option<usize> {
        if start == end {
            return Some(end);
        }
        let children = self.get_children(start).unwrap();
        for next in children {
            return self.bfs(next, end);
        }
        return None;
    }
}

#[allow(dead_code)]
impl Graph {
    pub fn total_number_of_paths(&self, start: usize, end: usize, total: usize, memo: &mut HashMap<usize, usize>) -> usize {
        let mut total = total;
        if start == end {
            total+=1;
        }
        let children = self.get_children(start);
        if let Some(children) = children {
            let children = children;
            for next in children {
                if memo.contains_key(&next) {
                    total += *memo.get(&&next).unwrap();
                } else {
                    total = self.total_number_of_paths(next, end, total, memo);
                    memo.insert(next, total);
                }
            }
        }
        return total;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RAWINPUT: &str = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";

    const RAWINPUT2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    // test Input
    #[test]
    fn test_input() {
        let input = Input::from(RAWINPUT);
        let expected_output = vec!(16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4);
        let expected_output: VecDeque<usize> = VecDeque::from(expected_output);
        assert_eq!(expected_output, input.0);
    }

    // test sort in Input
    #[test]
    fn test_sort() {
        let mut input = Input::from(RAWINPUT);
        input.sort();
        let expcted_output = vec!(1,4,5,6,7,10,11,12,15,16,19);
        let expcted_output: VecDeque<usize> = VecDeque::from(expcted_output);
        assert_eq!(expcted_output, input.0);
    }

    //test graph
    #[test]
    fn test_graph() {
        let mut g: Graph = Graph::new();
        g.insert_key(1);
        g.insert_key(2);
        g.insert_key(3);
        g.insert_key(4);
        g.insert_value(1, 2);
        g.insert_value(1, 3);
        g.insert_value(2, 4);
        g.insert_value(3, 4);
    }

    #[test]
    fn test_graph_conversion() {
        let input = Input::from(RAWINPUT);
        let mut test_graph = Graph::new();
        test_graph.insert_key(0);
        test_graph.insert_key(1);
        test_graph.insert_key(4);
        test_graph.insert_key(5);
        test_graph.insert_key(6);
        test_graph.insert_key(7);
        test_graph.insert_key(10);
        test_graph.insert_key(11);
        test_graph.insert_key(12);
        test_graph.insert_key(15);
        test_graph.insert_key(16);
        test_graph.insert_key(19);

        test_graph.insert_value(0, 1);
        test_graph.insert_value(1, 4);
        test_graph.insert_value_vec(4, vec!(5,6,7));
        test_graph.insert_value_vec(5, vec!(6,7));
        test_graph.insert_value(6, 7);
        test_graph.insert_value(7, 10);
        test_graph.insert_value_vec(10, vec!(11, 12));
        test_graph.insert_value(11, 12);
        test_graph.insert_value(12, 15);
        test_graph.insert_value(15, 16);
        test_graph.insert_value(16, 19);
        let input: Graph = input.into_graph();
        assert_eq!(input, test_graph);
    }

    #[test]
    fn test_total_number_of_paths() {
        let mut test_graph = Graph::new();
        test_graph.insert_key(1);
        test_graph.insert_key(4);
        test_graph.insert_key(5);
        test_graph.insert_key(6);
        test_graph.insert_key(7);
        test_graph.insert_key(10);
        test_graph.insert_key(11);
        test_graph.insert_key(12);
        test_graph.insert_key(15);
        test_graph.insert_key(16);
        test_graph.insert_key(19);

        test_graph.insert_value(1, 4);
        test_graph.insert_value_vec(4, vec!(5,6,7));
        test_graph.insert_value_vec(5, vec!(6,7));
        test_graph.insert_value(6, 7);
        test_graph.insert_value(7, 10);
        test_graph.insert_value_vec(10, vec!(11, 12));
        test_graph.insert_value(11, 12);
        test_graph.insert_value(12, 15);
        test_graph.insert_value(15, 16);
        test_graph.insert_value(16, 19);
        let output_count = test_graph.total_number_of_paths(1, 19, 0, &mut HashMap::new());
        assert_eq!(8, output_count);
    }


    #[test]
    fn test_total_numbers_of_paths2() {
        let input = Input::from(RAWINPUT2);
        let graph: Graph = input.into_graph();
        let output_count = graph.total_number_of_paths(0, 49, 0, &mut HashMap::new());
        assert_eq!(19208, output_count);
    }
}
