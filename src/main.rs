use rds::graph::Graph;
use rds::graph::weighted::Graph as WeightedGraph;
use rds::tree::binary_tree::BinaryTree;
use rds::tree::binary_tree::Node;

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

    let mut bt = BinaryTree::<i32>::new();
    println!("Is tree empty? {}", bt.is_empty());
    
    let root:Node<i32> = Node::complete(
        10,
        Box::new(
            Node::complete(
                20,
                Box::new(Node::leaf(15)),
                Box::new(Node::leaf(25))
            )
        ),
        Box::new(
            Node::complete(
                5,
                Box::new(Node::leaf(1)),
                Box::new(Node::leaf(8))
            )
        ),
    );
    bt.set_root(root);
    println!("Is tree empty? {}", bt.is_empty());

    println!("Contains 10? {}", bt.contains(10));
    println!("Contains 20? {}", bt.contains(20));
    println!("Contains 15? {}", bt.contains(15));
    println!("Contains 25? {}", bt.contains(25));
    println!("Contains 5? {}", bt.contains(5));
    println!("Contains 1? {}", bt.contains(1));
    println!("Contains 8? {}", bt.contains(8));
    println!("Contains 100? {}", bt.contains(100));
    Ok(())
}    

