use std::collections::HashMap;

#[derive(Clone)]
struct Edge {
    u: usize,
    v: usize,
    weight: usize,
}

impl Edge {
    fn new(u: usize, v: usize, weight: usize) -> Self {
        Self {
            u,
            v,
            weight,
        }
    }
}

pub struct Graph {
    n: usize,
    edges: Vec<Edge>,
    nodes: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: Vec::new(),
            nodes: Graph::init_nodes(n),
        }
    }

    fn init_nodes(n: usize) -> HashMap<usize, Vec<Edge>> {
        let mut nodes = HashMap::new();
        for node in 0..n {
            nodes.insert(node, Vec::new());
        }
        nodes
    }

    fn validate(&self, u: usize, v: usize) -> Result<(), String> {
        if u >= self.n || v >= self.n {
            return Err(format!("u and v must be smaller than {}", self.n));
        }
        Ok(())
    }

    pub fn add_edge(&mut self, u: usize, v: usize, weight: usize) -> Result<(), String> {
        self.validate(u, v)?;
        let edge = Edge::new(u, v, weight);
        self.nodes.get_mut(&u).unwrap().push(edge.clone());
        self.nodes.get_mut(&v).unwrap().push(edge.clone());
        self.edges.push(edge);
        Ok(())
    } 

    pub fn shortest_path(&self, from: usize, to: usize) -> Result<Vec<usize>, String> {
        self.validate(from, to)?;
        Ok(Vec::new())
    }

}

