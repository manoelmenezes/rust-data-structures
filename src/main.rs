use rds::graph::Graph;
use rds::graph::weighted::Graph as WeightedGraph;

fn main() ->Result<(), String> {
    let mut graph = Graph::new(6);
    graph.add_edge(0, 1)?;
    graph.add_edge(1, 2)?;
    graph.add_edge(2, 3)?;
    graph.add_edge(4, 5)?;

    let c = graph.are_connected(0, 3)?;
    println!("0 and 3 connected: {}", c);   

    let c = graph.are_connected(3, 4)?;
    println!("3 and 4 connected: {}", c); 

    let mut graph = WeightedGraph::new(5);
    graph.add_edge(0, 1, 5)?;
    graph.add_edge(0, 2, 2)?;
    graph.add_edge(1, 2, 4)?;
    graph.add_edge(1, 3, 2)?;
    graph.add_edge(2, 4, 1)?;
    graph.add_edge(3, 4, 100)?;
    
    let shortest_path_result = graph.shortest_path(0)?;

    println!("Shortest path result {:#?}", shortest_path_result);
    Ok(())
}    

