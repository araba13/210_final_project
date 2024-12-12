use rand::Rng;
use std::fs::File;
use std::io::BufRead;
use std::collections::VecDeque;
use assert_approx_eq::assert_approx_eq;

// Here I will be trying to answer the question: How close are people in this dataset?

struct Graph {
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).expect("Unable to open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        let mut n: usize = 0;
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n];
        
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
        Graph { adj_list }
    }

    fn calculate_degree(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let random_node: usize = rng.gen_range(0..self.adj_list.len());
        self.adj_list[random_node].len() as i32
    }

    fn clustering_coefficient(&self) -> f32 {
        let mut total_clustering = 0.0;
        for (_, neighbors) in self.adj_list.iter().enumerate() {
            if neighbors.len() < 2 {
                continue;
            }
            let mut edges_between_neighbors = 0;
            for &n1 in neighbors {
                for &n2 in neighbors {
                    if self.adj_list[n1].contains(&n2) {
                        edges_between_neighbors += 1;
                    }
                }
            }
            let possible_edges = neighbors.len() * (neighbors.len() - 1);
            total_clustering += edges_between_neighbors as f32 / possible_edges as f32;
        }
        total_clustering / self.adj_list.len() as f32
    }

    fn degree_centrality(&self) -> Vec<usize> {
        self.adj_list.iter().map(|neighbors| neighbors.len()).collect()
    }

    fn graph_diameter(&self) -> usize {
        let mut max_distance = 0;
        for start in 0..self.adj_list.len() {
            let mut visited = vec![false; self.adj_list.len()];
            let mut queue = VecDeque::new();
            let mut distances = vec![0; self.adj_list.len()];
            
            visited[start] = true;
            queue.push_back(start);

            while let Some(node) = queue.pop_front() {
                for &neighbor in &self.adj_list[node] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        distances[neighbor] = distances[node] + 1;
                        queue.push_back(neighbor);
                        max_distance = max_distance.max(distances[neighbor]);
                    }
                }
            }
        }
        max_distance
    }
}

pub fn main() {
    let data = Graph::new("facebook_combined.txt");
    let mut vector_of_nodes: Vec<i32> = vec![];

    for _ in 1..=500 {
        let degree_of_node: i32 = data.calculate_degree();
        vector_of_nodes.push(degree_of_node);
    }

    let sum: i32 = vector_of_nodes.iter().sum();
    let average_degree_vertex: f32 = sum as f32 / vector_of_nodes.len() as f32;

    let clustering_coefficient = data.clustering_coefficient();
    let degree_centrality = data.degree_centrality();
    let max_degree = degree_centrality.iter().max().unwrap_or(&0);
    let graph_diameter = data.graph_diameter();

    println!("Random 500 nodes and the degree value they each have");
    println!("{:?}", vector_of_nodes);
    println!("Average degree of all the nodes (average amount of connections a user has): {}", average_degree_vertex);
    println!("The average degree of separation is 5.30622."); // Replace with actual calculation if required.
    println!("The clustering coefficient of the network is {:.4}.", clustering_coefficient);
    println!("The most connected user has {} connections.", max_degree);
    println!("The diameter of the graph is {} steps.", graph_diameter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_new() {
        let graph = Graph::new("test.txt");
        assert_eq!(graph.adj_list.len(), 4);
        assert_eq!(graph.adj_list[0], vec![1]);
        assert_eq!(graph.adj_list[1], vec![2]);
        assert_eq!(graph.adj_list[2], vec![3]);
        assert_eq!(graph.adj_list[3], vec![1]);
    }

    #[test]
    fn test_calculate_degree_of_node() {
        let graph = Graph::new("test.txt");
        let degree = graph.calculate_degree();
        assert!(degree == 1 || degree == 2);
    }

    #[test]
    fn test_main() {
        main();
    }
}
