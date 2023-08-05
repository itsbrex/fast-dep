use std::collections::{HashSet, HashMap};
use std::cell::{RefCell, Ref};
use std::ops::Deref;

use pyo3::prelude::*;

use crate::importlib;

#[pyclass]
pub struct DepNode {
    pub name: String,
    pub spec: importlib::ModuleSpec,
    dependencies: i32,
    // The dependencies by spec.name
    dependents: HashSet<String>
}

impl DepNode {
    pub fn new(spec: importlib::ModuleSpec) -> DepNode {
        DepNode {
            name: spec.name.clone(),
            spec: spec,
            dependencies: 0,
            dependents: HashSet::new()
        }
    }

    fn is_root(&self) -> bool {
        self.dependencies == 0
    }
}

#[pyclass]
pub struct DepGraph {
    pub nodes: HashMap<String, RefCell<DepNode>>,
    root_nodes: HashSet<String>

}

impl DepGraph {
    pub fn new() -> DepGraph {
        DepGraph {
            nodes: HashMap::new(),
            root_nodes: HashSet::new(),
        }
    }

    pub fn add_dependency(&self, from: &str, on: &str) {
        /// This method will take a node which is currently under construction and updated it's dependencies.
        /// 
        /// **NOTE:** It is imperative that the `from` which is taken in and returned from this method is added to the graph at some point.
        println!("Add dependency '{}' -> '{}'", from, on);

        // Make sure we have the `on` node
        assert!(
            self.nodes.contains_key(from),
            "Node does not exist on graph: {}", from
        );
        assert!(
            self.nodes.contains_key(on),
            "Node does not exist on graph: {}", on
        );

        let mut on = self.nodes.get(on).unwrap().borrow_mut();
        on.dependents.insert(from.to_string());

        let mut from = self.nodes.get(from).unwrap().borrow_mut();
        from.dependencies += 1;
    }

    // pub fn add_dependent(&self, node: &str, dependent: &str) {
    //     // Before performing an operation on either, make sure both exist
    //     println!("Adding '{}' as dependent of '{}'.", dependent, node);
    //     assert!(
    //         self.nodes.contains_key(node),
    //         "Node does not exist on graph: {}", node
    //     );
    //     assert!(
    //         self.nodes.contains_key(dependent),
    //         "Node does not exist on graph: {}", dependent
    //     );

    //     let mut node = self.nodes.get(node).unwrap().borrow_mut();
    //     node.dependents.insert(dependent.to_string());

    //     let mut dependent = self.nodes.get(dependent).unwrap().borrow_mut();
    //     dependent.dependencies += 1;
    // }

    pub fn add(&mut self, node: DepNode) -> Ref<DepNode> {
        assert!(!self.nodes.contains_key(&node.name));

        let name = node.name.clone(); // TODO: Better way?

        self.nodes.insert(
            name.clone(),
            RefCell::new(node)
        );
        self.root_nodes.insert(name.clone());

        self.nodes.get(&name).unwrap().borrow()
    }

    // TODO: Read up on the `where` syntax
    pub fn with<F>(self, name: &str, f: F) where F: Fn(&DepNode) {
        let node = self.nodes.get(name).unwrap().borrow();
        // TODO: Not sure I understand the deref part
        f(node.deref())
    }

    pub fn has_node(&self, name: &str) -> bool {
        self.nodes.contains_key(name)
    }
}


#[pymethods]
impl DepGraph {
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn keys(&self) -> HashSet<String> {
        // TODO: Probably expensive, faster to write a conversion for graph.keys() and Keys type?
        self.nodes.iter().map(|(key, _)| key.to_string()).collect()
    }
}