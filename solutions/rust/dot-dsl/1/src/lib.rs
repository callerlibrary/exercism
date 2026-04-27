pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    // 定义宏
    macro_rules! impl_with_attrs {
        ($type:ty) => {
            impl $type {
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs
                        .extend(attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())));
                    self
                }
            }
        };
    }

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == node_name)
        }

        pub fn attr(&self, attr_name: &str) -> Option<&str> {
            self.attrs.get(attr_name).map(String::as_str)
        }
    }

    impl_with_attrs!(Graph);

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, Default, PartialEq)]
            pub struct Node {
                pub(crate) name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn attr(&self, attr_name: &str) -> Option<&str> {
                    self.attrs.get(attr_name).map(String::as_str)
                }
            }

            impl_with_attrs!(Node);
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, Default, PartialEq)]
            pub struct Edge {
                start: String,
                end: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    Self {
                        start: start.to_string(),
                        end: end.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn attr(&self, attr_name: &str) -> Option<&str> {
                    self.attrs.get(attr_name).map(String::as_str)
                }
            }

            impl_with_attrs!(Edge);
        }
    }
}
