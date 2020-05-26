pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attr: &[(&str, &str)]) -> Node {
                    attr.iter().for_each(|attr| {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    });
                    self
                }
                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).as_ref().map(|k| k.as_str())
                }
            }
        }

        pub mod edge {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                node1: String,
                node2: String,
                attrs: Vec<Attr>,
            }

            #[derive(Clone, PartialEq, Debug)]
            pub struct Attr(String, String);
            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Edge {
                    Edge {
                        node1: node1.to_string(),
                        node2: node2.to_string(),
                        attrs: vec![],
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Edge {
                    attrs.iter().for_each(|att| {
                        self.attrs.push(Attr(att.0.to_string(), att.1.to_string()))
                    });
                    self
                }
            }
        }
    }

    #[derive(PartialEq, Default)]
    pub struct Graph {
        pub edges: Vec<Edge>,
        pub nodes: Vec<Node>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                edges: vec![],
                nodes: vec![],
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Graph {
            nodes.iter().for_each(|node| self.nodes.push(node.clone()));
            self
        }

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Graph {
            edges.iter().for_each(|edge| self.edges.push(edge.clone()));
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Graph {
            attrs.iter().for_each(|att| {
                self.attrs.insert(att.0.to_string(), att.1.to_string());
            });
            self
        }

        pub fn get_node(self, name: &str) -> Option<Node> {
            self.nodes
                .iter()
                .find(|node| node.name.eq(&name.to_string()))
                .cloned()
        }

        pub fn get_attr(self, name: &str) -> Option<String> {
            self.attrs.get(name).cloned()
        }
    }
}
