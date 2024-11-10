use std::num::NonZeroUsize;
use crate::dom::{Document, Element};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(NonZeroUsize);

impl NodeId {
    pub fn to_index(&self) -> usize {
        self.0.get() - 1
    }

    pub fn from_index(i: usize) -> Self {
        NodeId(NonZeroUsize::new(i + 1).unwrap())
    }
}

pub struct Tree {
    vec: Vec<Node>,
}

impl Tree {
    pub fn new(root: Element) -> Self {
        Tree {
            vec: vec![Node::new(root)]
        }
    }

    pub fn get(&self, id: NodeId) -> Option<NodeRef> {
        self.vec.get(id.to_index()).map(|node| NodeRef {
            id,
            node,
            tree: self,
        })
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<NodeMut> {
        if let Some(_exists) = self.vec.get(id.to_index()) {
            Some(NodeMut { id, tree: self })
        } else {
            None
        }
    }

    pub fn root(&self) -> NodeRef {
        self.get(NodeId::from_index(0)).expect("root node missing")
    }

    pub fn root_mut(&mut self) -> NodeMut {
        self.get_mut(NodeId::from_index(0)).expect("root node missing")
    }

    fn node(&self, id: NodeId) -> Result<&Node, &str> {
        if let Some(node) = self.vec.get(id.to_index()) {
            Ok(node)
        } else {
            Err("node not found")
        }
    }

    fn node_mut(&mut self, id: NodeId) -> Result<&mut Node, &str> {
        if let Some(node) = self.vec.get_mut(id.to_index()) {
            Ok(node)
        } else {
            Err("node not found")
        }
    }

    fn orphan(&mut self, element: Element) -> NodeMut {
        let id = NodeId::from_index(self.vec.len());
        self.vec.push(Node::new(element));
        self.get_mut(id).unwrap()
    }
}

pub struct Node {
    parent: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    children: Option<(NodeId, NodeId)>,
    element: Element,
}

impl Node {
    pub fn new(element: Element) -> Self {
        Node {
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            children: None,
            element
        }
    }
}

pub struct NodeRef<'a> {
    id: NodeId,
    tree: &'a Tree,
    node: &'a Node,
}

pub struct NodeMut<'a> {
    id: NodeId,
    tree: &'a mut Tree,
}

impl <'a> Copy for NodeRef<'a> {}
impl <'a> Clone for NodeRef<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl <'a> Eq for NodeRef<'a> {}
impl <'a> PartialEq for NodeRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && std::ptr::eq(self.node, other.node)
            && std::ptr::eq(self.tree, other.tree)
    }
}

impl <'a> NodeRef<'a> {
    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn element(&self) -> &Element {
        &self.node.element
    }

    pub fn tree(&self) -> &'a Tree {
        self.tree
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.node.parent
    }

    pub fn prev_sibling(&self) -> Option<NodeId> {
        self.node.prev_sibling
    }

    pub fn next_sibling(&self) -> Option<NodeId> {
        self.node.next_sibling
    }

    pub fn first_child(&self) -> Option<NodeId> {
        self.node.children.map(|(id, _)| id)
    }

    pub fn last_child(&self) -> Option<NodeId> {
        self.node.children.map(|(_, id)| id)
    }

    pub fn has_siblings(&self) -> bool {
        self.node.prev_sibling.is_some() || self.node.next_sibling.is_some()
    }

    pub fn has_children(&self) -> bool {
        self.node.children.is_some()
    }
}

impl <'a> NodeMut<'a> {
    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn element(&mut self) -> &mut Element {
        &mut self.node().element
    }

    pub fn tree(&mut self) -> &mut Tree {
        self.tree
    }

    fn node(&mut self) -> &mut Node {
        self.tree.node_mut(self.id).unwrap()
    }

    pub fn parent(&mut self) -> Option<NodeId> {
        self.node().parent
    }

    pub fn prev_sibling(&mut self) -> Option<NodeId> {
        self.node().prev_sibling
    }

    pub fn next_sibling(&mut self) -> Option<NodeId> {
        self.node().next_sibling
    }

    pub fn first_child(&mut self) -> Option<NodeId> {
        self.node().children.map(|(id, _)| id)
    }

    pub fn last_child(&mut self) -> Option<NodeId> {
        self.node().children.map(|(_, id)| id)
    }

    pub fn has_siblings(&self) -> bool {
        self.tree.get(self.id).unwrap().has_siblings()
    }

    pub fn has_children(&self) -> bool {
        self.tree.get(self.id).unwrap().has_children()
    }

    pub fn detach(&mut self) {
        let parent_id = match self.node().parent {
            Some(id) => id,
            None => return,
        };
        let prev_sibling_id = self.node().prev_sibling;
        let next_sibling_id = self.node().next_sibling;

        self.node().parent = None;
        self.node().prev_sibling = None;
        self.node().next_sibling = None;

        if let Some(id) = prev_sibling_id {
            self.tree.node_mut(id).unwrap().next_sibling = next_sibling_id;
        }
        if let Some(id) = next_sibling_id {
            self.tree.node_mut(id).unwrap().prev_sibling = prev_sibling_id;
        }

        let parent = self.tree.node_mut(parent_id).unwrap();
        let (first_child_id, last_child_id) = parent.children.unwrap();
        if first_child_id == last_child_id {
            parent.children = None;
        } else if first_child_id == self.id {
            parent.children = Some((next_sibling_id.unwrap(), last_child_id));
        } else if last_child_id == self.id {
            parent.children = Some((first_child_id, prev_sibling_id.unwrap()))
        }
    }

    pub fn append(&mut self, element: Element) -> NodeMut {
        let id = self.tree.orphan(element).id;
        self.append_id(id)
    }

    pub fn append_id(&mut self, new_child_id: NodeId) -> NodeMut {
        let last_child_id = self.last_child();
        {
            let mut new_child = self.tree.get_mut(new_child_id).unwrap();
            new_child.detach();
            new_child.node().prev_sibling = last_child_id;
            new_child.node().parent = Some(self.id);
        }

        if let Some(id) = last_child_id {
            let mut last_child = self.tree.get_mut(id).unwrap();
            last_child.node().next_sibling = Some(new_child_id);
        }

        if let Some((first_child_id, _)) = self.node().children {
            self.node().children = Some((first_child_id, new_child_id));
        } else {
            self.node().children = Some((new_child_id, new_child_id));
        }

       self.tree.get_mut(new_child_id).expect("failed to insert child")
    }
}

