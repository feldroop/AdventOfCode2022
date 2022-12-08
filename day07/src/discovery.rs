use crate::filesystem_model::{Command, FilesystemItem, FilesystemTree, RelativeDirectory};

pub struct FilesystemDiscoverer {
    current_directory_id: usize,
    tree: FilesystemTree,
}

impl FilesystemDiscoverer {
    pub fn start_at_root() -> Self {
        FilesystemDiscoverer {
            current_directory_id: 0,
            tree: FilesystemTree::with_root(),
        }
    }

    pub fn apply_command(&mut self, command: &Command) {
        match command {
            Command::ChangeDirectory { target } => {
                self.change_directory(target);
            }
            Command::List { output_items } => {
                self.update_directory(output_items);
            }
        }
    }

    pub fn finish(self) -> FilesystemTree {
        self.tree
    }

    // return id of new directory
    fn change_directory(&mut self, target: &RelativeDirectory) {
        let nodes = &self.tree.nodes();
        let current_directory = &nodes[self.current_directory_id];

        let next_node = match target {
            RelativeDirectory::Child { name } => {
                let &child_id = current_directory
                    .child_ids
                    .get(name)
                    .expect("Should not call cd on undiscovered children");

                &nodes[child_id]
            }
            RelativeDirectory::Parent => {
                let parent_id = current_directory
                    .parent_id
                    .expect("Should not call cd .. at root");

                &nodes[parent_id]
            }
            RelativeDirectory::Root => self.tree.root(),
        };

        self.current_directory_id = next_node.id;
    }

    fn update_directory(&mut self, list_items: &[FilesystemItem]) {
        for item in list_items {
            if !self
                .tree
                .has_child_name(self.current_directory_id, item.name())
            {
                self.tree
                    .insert_child(self.current_directory_id, item.clone());
            }
        }
    }
}
