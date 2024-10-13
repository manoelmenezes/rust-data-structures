fn main() -> Result<(), String> {
    let mut graph = Graph::new(4);
    graph.add_edge(0, 1)?;
    graph.add_edge(4, 4)?;
    
    Ok(())
}    

struct Graph {
    n: usize,
    nodes: Vec<Vec<i32>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            n,
            nodes: vec![vec![-1; n]; n],
        }
    }
    
    pub fn add_edge(&mut self, u: usize, v: usize) -> Result<(), String> {
        if u >= self.n || v >= self.n {
            return  Err(format!("u and v need to be smaller than {}", self.n));
        }
        self.nodes[u][v] = 1;
        self.nodes[v][u] = 1;
        
        Ok(())
    }
}
