use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::cmp::Reverse;

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

    fn to(&self, from: usize) -> usize {
        if from == self.u {
            self.v
        } else {
            self.u
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

    pub fn shortest_path(&self, from: usize) -> Result<ShortestPathResult, String> {
        if from >= self.n {
            return Err(format!("From must be smaller than {}", self.n));
        }
        let mut dist = vec![usize::MAX; self.n];
        dist[from] = 0;
        let mut prev = vec![0; self.n];
        let mut pq = PriorityQueue::new(); 
        for n in 0..self.n {
            if n != from {
                pq.push(n, Reverse(usize::MAX));
            }
        }
        pq.push(from, Reverse(0));
        
        while let Some((n, _)) = pq.pop() {
            for e in self.nodes.get(&n).unwrap() {
                let d = dist[n] + e.weight;
                let to = e.to(n);
                if d < dist[to] {
                    prev[to] = n;
                    dist[to] = d;
                    pq.change_priority(&to, Reverse(d));
                }
            }
        }
        Ok(ShortestPathResult{prev: prev, dist: dist, from})
    }

}

#[derive(Debug)]
pub struct ShortestPathResult {
    pub prev: Vec<usize>,
    pub dist: Vec<usize>,
    pub from: usize,
}

