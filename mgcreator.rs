pub mod mg {
    pub struct Graph {
        pub field1: String,
        pub field2: i32,
    }

    pub struct Node {
        pub name: String,
    }

    impl Graph {
        pub fn new(field1: String, field2: i32) -> Self {
            Self { field1, field2 }
        }

        pub fn create_node(id: String) {

        }

        pub fn create_relationship(idA: String, idB: String, transition: String) {

        }
    }

    /* TODO: Create GrammarGraph */
    pub struct GrammarGraph {
        pub graph_title: String,
    }

    impl Graph for GrammarGraph {
        pub fn new() {

        }

        pub fn parse_MG(mg: String) {

        }
    }
}
