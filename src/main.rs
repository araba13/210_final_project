use rand::Rng;
use std::fs::File;
use std::io::BufRead;
use std::collections::VecDeque;
use std::vec;
use assert_approx_eq::assert_approx_eq;
mod how_close;

// Here I will be computing the average degrees of separation in this Facebook dataset to test the six degrees of separation theory.

// struct Graph
struct Graph {
    adj_list: Vec<Vec<usize>>,
}

// implementation on the struct Graph
impl Graph {
    // adjacency list
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        let mut n: usize = 0;
        let mut adj_list: Vec<Vec<usize>> = vec![];
    
        for (_i, line) in buf_reader.enumerate() {
            let line_str = line.expect("Error reading");
            if _i == 0 {
                n = line_str.parse::<usize>().unwrap();
                adj_list = vec![vec![]; n];
            } else {
                let v: Vec<&str> = line_str.trim().split_whitespace().collect();
                let x = v[0].parse::<usize>().unwrap();
                let y = v[1].parse::<usize>().unwrap();
                adj_list[x].push(y);
            }
        }
        
        // return the constructed Graph instance
        Graph { adj_list }
    }


    fn bfs(&self) -> Option<Vec<Option<usize>>> {
        let start = rand::thread_rng().gen_range(0..self.adj_list.len());
        let end = rand::thread_rng().gen_range(0..self.adj_list.len());

        let mut queue = VecDeque::new();
        let mut visited_vertices = vec![false; self.adj_list.len()];
        let mut order = vec![None; self.adj_list.len()];
        queue.push_back(start);
        visited_vertices[start] = true;
    
        while let Some(current_node) = queue.pop_front() {
            if current_node == end {
                break;
            }
            for &neighbor in &self.adj_list[current_node] {
                if !visited_vertices[neighbor] {
                    visited_vertices[neighbor] = true;
                    order[neighbor] = Some(current_node);
                    queue.push_back(neighbor);
                }
            }
        }
        let mut path = vec![];
        let mut end_check = Some(end);
        while let Some(node) = end_check {
            path.push(end_check);
            end_check = order[node];
        }
        path.reverse();
        if path[0] == Some(start) {
            Some(path)
        } else {
            None
        }
    }    

    
    fn average_distance(&self) -> f32 {
        let graph = Graph::new("facebook_combined.txt");
        let mut distances = Vec::new();
    
        for _ in 0..3000 {
            let path = graph.bfs();
            for vertex_path in path.iter() {
                let mut distance = 0;
                for _ in vertex_path.iter() {
                    distance += 1;
                }
                distances.push(distance);
            }
        }
    
        let sum: i32 = distances.iter().sum();
        let average: f32 = (sum as f32) / (distances.len() as f32);
        // println!("average distance between a pair of vertices (distance between users) in Facebook dataset: {}", average);
        return average;
    }

}



fn main() {
    println!();
    let graph = Graph::new("facebook_combined.txt");
    let avg_distance = graph.average_distance();
    println!("Average degrees of separation in Facebook dataset (distance between two users): {}", avg_distance);

    how_close::main();

}


#[test]
fn test_average_distance() {
    let graph = Graph::new("facebook_combined.txt");
    let avg_distance = graph.average_distance();
    assert_approx_eq!(avg_distance, 5.0, 1.0);
}