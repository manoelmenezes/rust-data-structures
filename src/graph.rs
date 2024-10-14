use std::collections::HashSet;
use std::collections::VecDeque;

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

    Ok(())
}    

struct Graph {
    n: usize,
    nodes: Vec<Vec<bool>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            n,
            nodes: vec![vec![false; n]; n],
        }
    }

    fn validate(&self, u: usize, v: usize) -> Result<(), String> {
        if u >= self.n || v >= self.n {
            return Err(format!("u and v must be smaller than {}", self.n));
        }
        Ok(())
    }

    pub fn are_connected(&self, u: usize, v: usize) -> Result<bool, String> {
        self.validate(u, v)?;
        if self.nodes[u][v] {
            return Ok(true);
        }
        let mut queue = VecDeque::new();
        queue.push_back(u);
        let mut visited = HashSet::new();
        while !queue.is_empty() {
           let current = queue.pop_front().unwrap();
           visited.insert(current);
           if current == v {
               return Ok(true);
           }
           for i in 0..self.n {
               if self.nodes[current][i] && !visited.contains(&i) {
                   queue.push_back(i);
               }
           }
        }
        Ok(false)
    }
    
    pub fn add_edge(&mut self, u: usize, v: usize) -> Result<(), String> {
        self.validate(u, v)?;
        self.nodes[u][v] = true;
        self.nodes[v][u] = true;
        
        Ok(())
    }
}
