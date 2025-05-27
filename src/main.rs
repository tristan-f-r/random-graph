use petgraph::graph::UnGraph;
use clap::Parser;
use rand::seq::SliceRandom;
use itertools::izip;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    vertices: usize,
    #[arg(short, long)]
    edges_per: usize,
}

fn main() {
    let cli = Cli::parse();
    let mut graph = UnGraph::<usize, ()>::new_undirected();

    {
        for v in 1..=cli.vertices {
            graph.add_node(v);
        }

        let mut nodes_first = graph.node_indices().cycle().take(cli.vertices * cli.edges_per).collect::<Vec<_>>();
        nodes_first.shuffle(&mut rand::rng());
        let mut nodes_second = graph.node_indices().cycle().take(cli.vertices * cli.edges_per).collect::<Vec<_>>();
        nodes_second.shuffle(&mut rand::rng());

        for (first_vertex, second_vertex) in izip!(nodes_first, nodes_second) {
            graph.add_edge(first_vertex, second_vertex, ());
        }
    }

    for edge in graph.edge_indices() {
        let (first_vertex, second_vertex) = graph.edge_endpoints(edge).unwrap();
        let first_vertex = graph.node_weight(first_vertex).unwrap();
        let second_vertex = graph.node_weight(second_vertex).unwrap();
        println!("{first_vertex}\t{second_vertex}");
    }
}
