use std::num::NonZeroUsize;
use crate::dom::Document;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(NonZeroUsize);

impl NodeId {
    pub(crate) fn to_index(&self) -> usize {
        self.0.get()
    }
}

pub struct Tree {
    vec: Vec<Node>,
}

pub struct Node {
    parent: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    children: Option<(NodeId, NodeId)>,
}

impl Tree {
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
}

