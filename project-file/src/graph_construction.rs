use crate::csv_handling::VideoGame;
use petgraph::graph::{DiGraph, NodeIndex};

pub fn construct_genre_graph(video_games: &[VideoGame]) -> DiGraph<String, ()> {
    // Implementation of constructing genre-based graph
}

pub fn construct_sales_graph(video_games: &[VideoGame]) -> DiGraph<String, f64> {
    // Implementation of constructing sales-based graph
}
