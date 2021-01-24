use std::fs;
use std::collections::VecDeque;
use std::collections::HashMap;

// Input 
#[derive(Debug)]
pub struct Input {
    value: VecDeque<usize>
}

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
        Input {
            value: VecDeque::new()
        }
    }
    pub fn from(input: &str) -> Self {
        Input {
            value: Input::parse_input(input.to_string())
        }
    }
    pub fn read_input_from_file() -> Self {
        const FILEPATH: &str = "input/day10.input";
        let input = fs::read_to_string(FILEPATH).unwrap();
        Input {
            value: Input::parse_input(input)
        }
    }
}

// return computed results
#[allow(dead_code)]
impl Input {
    pub fn clone(&self) -> VecDeque<usize> {
        return self.value.clone();
    }
}

// mutable functions
impl Input {
    pub fn sort(&mut self) {
        let mut value: Vec<usize> = self.value.iter().map(|x| *x).collect();
        value.sort();
        self.value = VecDeque::from(value);
    }
}


// conver to other types
#[allow(dead_code)]
impl Input {
    pub fn into_graph(self) -> Graph {
        let mut val: Vec<usize> = self.value.into();
        let mut g = Graph::new();
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

// Graph
#[derive(Debug, PartialEq)]
pub struct Graph {
    value: HashMap<usize, Vec<usize>>
}

#[allow(dead_code)]
impl Graph {
    pub fn new() -> Self {
        Graph {
            value: HashMap::new()
        }
    }
    pub fn insert_key(&mut self, key: usize) {
        self.value.insert(key, vec!());
    }
    pub fn insert_value(&mut self, key: usize, value: usize) {
        let mut val: Vec<usize> = self.value
            .get(&key)
            .expect("Key not found")
            .clone();
        val.push(value);
        self.value.insert(key, val);
    }
    pub fn insert_value_vec(&mut self, key: usize, values: Vec<usize>) {
        //let mut values = values;
        //let mut val: Vec<usize> = self.value
        //    .get(&key)
        //    .expect("Key not found")
        //    .clone();
        //val.append(&mut values);
        self.value.insert(key, values);
    }
    pub fn get_list_of_keys(self) -> Vec<usize> {
        return self.value.keys().map(|x| *x).collect();
    }
    pub fn get_value(self, key: usize) -> Option<Vec<usize>> {
        let val = self.value.get(&key)?;
        return Some(val.clone());
    }
    pub fn clone(self) -> HashMap<usize, Vec<usize>> {
        return self.value.clone();
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

    // test Input
    #[test]
    fn test_input() {
        let input = Input::from(RAWINPUT);
        let expected_output = vec!(16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4);
        let expected_output: VecDeque<usize> = VecDeque::from(expected_output);
        assert_eq!(expected_output, input.value);
    }

    // test sort in Input
    #[test]
    fn test_sort() {
        let mut input = Input::from(RAWINPUT);
        input.sort();
        let expcted_output = vec!(1,4,5,6,7,10,11,12,15,16,19);
        let expcted_output: VecDeque<usize> = VecDeque::from(expcted_output);
        assert_eq!(expcted_output, input.value);
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
        let input: Graph = input.into_graph();
        assert_eq!(input, test_graph);
    }
}

