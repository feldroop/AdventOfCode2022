use std::fmt::Write;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum RelativeDirectory {
    Child { name: String },
    Parent,
    Root,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FilesystemItem {
    File { name: String, size: usize },
    Directory { name: String },
}

impl FilesystemItem {
    pub fn name(&self) -> &String {
        use FilesystemItem::*;

        match self {
            File { name, size: _ } => name,
            Directory { name } => name,
        }
    }
}

impl Display for FilesystemItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FilesystemItem::*;

        match self {
            File { name, size } => write!(f, "{name} (file, size={size})"),
            Directory { name } => write!(f, "{name} (dir)"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    ChangeDirectory { target: RelativeDirectory },
    List { output_items: Vec<FilesystemItem> },
}

pub struct FilesystemNode {
    pub id: usize,
    pub item: FilesystemItem,
    pub parent_id: Option<usize>,
    pub child_ids: HashMap<String, usize>,
}

// does not support node deletion
pub struct FilesystemTree {
    nodes: Vec<FilesystemNode>,
}

impl FilesystemTree {
    pub fn with_root() -> Self {
        let root = FilesystemNode {
            id: 0,
            item: FilesystemItem::Directory {
                name: String::from("/"),
            },
            parent_id: None,
            child_ids: HashMap::new(),
        };
        FilesystemTree { nodes: vec![root] }
    }

    pub fn root(&self) -> &FilesystemNode {
        &self.nodes[0]
    }

    pub fn nodes(&self) -> &Vec<FilesystemNode> {
        &self.nodes
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn has_child_name(&self, parent_id: usize, child_name: &String) -> bool {
        self.nodes[parent_id].child_ids.contains_key(child_name)
    }
    pub fn insert_child(&mut self, parent_id: usize, child_item: FilesystemItem) {
        let new_id = self.len();

        self.nodes[parent_id]
            .child_ids
            .insert(child_item.name().clone(), new_id);

        let new_node = FilesystemNode {
            id: new_id,
            item: child_item,
            parent_id: Some(parent_id),
            child_ids: HashMap::new(),
        };

        self.nodes.push(new_node);
    }

    pub fn visit_items<F>(&self, mut f: F)
    where
        F: FnMut(&FilesystemItem, usize),
    {
        self.visit_node(&mut f, self.root(), 0);
    }

    fn visit_node<F>(&self, f: &mut F, current_node: &FilesystemNode, depth: usize)
    where
        F: FnMut(&FilesystemItem, usize),
    {
        f(&current_node.item, depth);

        let mut child_ids: Vec<_> = current_node.child_ids.values().copied().collect();
        child_ids.sort_by_key(|&id| self.nodes[id].item.name());

        for child_id in child_ids {
            self.visit_node(f, &self.nodes[child_id], depth + 1);
        }
    }

    #[allow(unused)]
    pub fn format_to_string(&self) -> String {
        let mut s = String::new();

        self.visit_items(|item, depth| {
            let padding = depth * 4;
            writeln!(s, "{:padding$}- {item}", "").unwrap();
        });

        s
    }

    pub fn recursive_directory_sizes(&self) -> HashMap<usize, usize> {
        let mut directory_sizes = HashMap::new();

        self.node_and_subdirectory_sizes(self.root(), &mut directory_sizes);

        directory_sizes
    }

    fn node_and_subdirectory_sizes(
        &self,
        current_node: &FilesystemNode,
        directory_sizes: &mut HashMap<usize, usize>,
    ) -> usize {
        match &current_node.item {
            FilesystemItem::File { name: _, size } => *size,
            FilesystemItem::Directory { name: _ } => {
                let mut total_size = 0;

                for &child_id in current_node.child_ids.values() {
                    let child_node = &self.nodes[child_id];
                    total_size += self.node_and_subdirectory_sizes(child_node, directory_sizes);
                }

                directory_sizes.insert(current_node.id, total_size);

                total_size
            }
        }
    }
}
