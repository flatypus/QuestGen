use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    head: Vec<usize>,
    child: Vec<usize>,
    name: String,
}

pub fn set_up_graph() -> HashMap<usize, Node> {
    let mut graph = HashMap::new();
    graph.insert(
        0,
        Node {
            head: vec![],
            child: vec![1, 2],
            name: "addition and subtraction".to_string(),
        },
    );
    graph.insert(
        1,
        Node {
            head: vec![0],
            child: vec![3, 4],
            name: "multiplication".to_string(),
        },
    );
    graph.insert(
        2,
        Node {
            head: vec![0],
            child: vec![5, 6],
            name: "division".to_string(),
        },
    );
    graph.insert(
        3,
        Node {
            head: vec![1],
            child: vec![],
            name: "exponents".to_string(),
        },
    );
    graph.insert(
        4,
        Node {
            head: vec![1],
            child: vec![],
            name: "square roots".to_string(),
        },
    );
    graph.insert(
        5,
        Node {
            head: vec![2],
            child: vec![],
            name: "fractions".to_string(),
        },
    );
    graph.insert(
        6,
        Node {
            head: vec![2],
            child: vec![],
            name: "percentages".to_string(),
        },
    );
    graph
}
