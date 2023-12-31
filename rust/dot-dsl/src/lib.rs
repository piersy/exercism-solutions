pub mod graph {
    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    use std::collections::HashMap;
    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node<'a>>,
        pub edges: Vec<graph_items::edge::Edge<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &Vec<Node<'a>>) -> Self {
            self.nodes = nodes.to_vec();
            // self.nodes = *nodes.clone();
            self
        }
        pub fn with_edges(mut self, edges: &Vec<Edge<'a>>) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }
        pub fn get_node(&self, name: &str) -> Option<&'a Node> {
            self.nodes.iter().find(|n| n.v == name)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            // #[derive(Clone)]
            #[derive(Clone, Eq, PartialEq, Debug)]
            pub struct Edge<'a> {
                start: &'a str,
                end: &'a str,
                pub attrs: HashMap<String, String>,
            }

            impl<'a> Edge<'a> {
                pub fn new(start: &'a str, end: &'a str) -> Self {
                    Edge {
                        start,
                        end,
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;
            #[derive(Clone, Eq, PartialEq, Debug)]
            pub struct Node<'a> {
                pub v: &'a str,
                pub attrs: HashMap<String, String>,
            }
            impl<'a> Node<'a> {
                pub fn new(v: &'a str) -> Self {
                    Node {
                        v,
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }
                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|v| v.as_str())
                }
            }
        }
    }
}
