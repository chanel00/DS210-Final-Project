// Compiling necessary crates
extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate petgraph;

// Importing necessary modules
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

mod community_detection {
    use super::*;

    // Defining the VideoGame struct
    #[derive(Debug, Deserialize)]
    pub struct VideoGame {
        #[serde(rename = "Genre")]
        pub genre: String,
        #[serde(rename = "Platform")]
        pub platform: String,
        #[serde(rename = "NA_Sales")]
        pub na_sales: f64,
        #[serde(rename = "EU_Sales")]
        pub eu_sales: f64,
        #[serde(rename = "JP_Sales")]
        pub jp_sales: f64,
    }

    pub fn detect_communities(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Opening the input file
        let input_file = std::fs::File::open(file_path)?;

        // Creating CSV reader
        let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(input_file);

        // Creating adjacency list to store the genre-based graph
        let mut genre_adjacency_list: HashMap<String, Vec<String>> = HashMap::new();
        // Creating adjacency list to store the sales-based graph
        let mut sales_adjacency_list: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        // Iterating over each record in the CSV file
        for result in rdr.deserialize::<VideoGame>() {
            let record = result?;

            // Updating genre-based adjacency list
            genre_adjacency_list
                .entry(record.genre.clone())
                .or_insert_with(Vec::new)
                .push(record.platform.clone());

            // Updating sales-based adjacency list
            let sales_data = record.na_sales + record.eu_sales + record.jp_sales;
            sales_adjacency_list
                .entry(record.genre.clone())
                .or_insert_with(Vec::new)
                .push((record.platform.clone(), sales_data));
        }

        // Applying Label Propagation Algorithm for community detection on genre-based graph
        let genre_communities = label_propagation(&genre_adjacency_list);
        // Applying Label Propagation Algorithm for community detection on sales-based graph
        let sales_communities = label_propagation_sales(&sales_adjacency_list);

        // Printing the communities for the genre-based graph
        println!("Communities based on Genre:");
        for (idx, community) in genre_communities.iter().enumerate() {
            println!("Community {}: {:?}", idx, community);
        }

        // Printing the communities for the sales-based graph
        println!("Communities based on Sales:");
        for (idx, community) in sales_communities.iter().enumerate() {
            println!("Community {}: {:?}", idx, community);
        }

        Ok(()) // Return Ok if everything is successful
    }

    pub fn label_propagation(adjacency_list: &HashMap<String, Vec<String>>) -> Vec<HashSet<String>> {
        let mut communities: Vec<HashSet<String>> = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();

        for node in adjacency_list.keys() {
            if !visited.contains(node) {
                let mut community: HashSet<String> = HashSet::new();
                let mut stack: Vec<String> = Vec::new();
                stack.push(node.clone());

                while let Some(current_node) = stack.pop() {
                    if visited.contains(&current_node) {
                        continue;
                    }
                    community.insert(current_node.clone());
                    visited.insert(current_node.clone());

                    if let Some(neighbors) = adjacency_list.get(&current_node) {
                        for neighbor in neighbors {
                            if !visited.contains(neighbor) {
                                stack.push(neighbor.clone());
                            }
                        }
                    }
                }

                communities.push(community);
            }
        }

        communities
    }

    pub fn label_propagation_sales(adjacency_list: &HashMap<String, Vec<(String, f64)>>) -> Vec<HashSet<String>> {
        let mut communities: Vec<HashSet<String>> = Vec::new();
        let mut visited: HashSet<String> = HashSet::new();

        for node in adjacency_list.keys() {
            if !visited.contains(node) {
                let mut community: HashSet<String> = HashSet::new();
                let mut stack: Vec<String> = Vec::new();
                stack.push(node.clone());

                while let Some(current_node) = stack.pop() {
                    if visited.contains(&current_node) {
                        continue;
                    }
                    community.insert(current_node.clone());
                    visited.insert(current_node.clone());

                    if let Some(neighbors) = adjacency_list.get(&current_node) {
                        for (neighbor, _) in neighbors {
                            if !visited.contains(neighbor) {
                                stack.push(neighbor.clone());
                            }
                        }
                    }
                }

                communities.push(community);
            }
        }

        communities
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_propagation() {
        let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();
        adjacency_list.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        adjacency_list.insert("B".to_string(), vec!["A".to_string()]);
        adjacency_list.insert("C".to_string(), vec!["A".to_string()]);

        let communities = community_detection::label_propagation(&adjacency_list);
        assert_eq!(communities.len(), 1);
        assert!(communities[0].contains("A"));
        assert!(communities[0].contains("B"));
        assert!(communities[0].contains("C"));
    }

    #[test]
    fn test_label_propagation_sales() {
        let mut adjacency_list: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        adjacency_list.insert("A".to_string(), vec![("B".to_string(), 10.0), ("C".to_string(), 20.0)]);
        adjacency_list.insert("B".to_string(), vec![("A".to_string(), 15.0)]);
        adjacency_list.insert("C".to_string(), vec![("A".to_string(), 25.0)]);

        let communities = community_detection::label_propagation_sales(&adjacency_list);
        assert_eq!(communities.len(), 1);
        assert!(communities[0].contains("A"));
        assert!(communities[0].contains("B"));
        assert!(communities[0].contains("C"));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    community_detection::detect_communities("/Users/chanelthorpe/GitHub/DS210-Final-Project/project-file/Video Games Sales/cleaned_vgsales.csv")
}