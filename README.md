# Rust Data Structures (rds)

## Graph

```
use rds::graph::Graph;

fn main() -> Result<(), String> {
    let mut graph = Graph::new(6);
    graph.add_edge(0, 1)?;
    graph.add_edge(1, 2)?;
    graph.add_edge(2, 3)?;
    graph.add_edge(4, 5)?;

    let c = graph.are_connected(0, 3)?;
    println!("0 and 3 connected: {}", c);   

    let c = graph.are_connected(3, 4)?;
    println!("3 and 4 connected: {}", c);

    Ok(())
}
```

## WeightedGraph

```
use rds::graph::weighted::Graph as WeightedGraph;

fn main() -> Result<(), String> {
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
```

## BST

```
use rds::tree::bst::BST;

fn main() {
    let mut bst = BST::<i32>::new();

    bst.insert(10);
    bst.insert(20);
    bst.insert(5);
    bst.insert(8);
    bst.insert(1);
    bst.insert(15);
    bst.insert(25);

    println!("BST elements {:#?}", bst.as_vec());
}    
```
