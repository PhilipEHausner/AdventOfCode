#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType, 
    pub name: String,
    pub size: u64,
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct FileTree {
    pub root: Node,
}

impl Node {
    pub fn new(node_type: NodeType, name: &str, size: u64) -> Node {
        Node {
            node_type, name: name.to_string(), size, children: vec![],
        }
    }

    pub fn get_child_node_mut(&mut self, child_name: &str) -> &mut Node {
        for child in &mut self.children {
            if child.name == child_name {
                return child;
            }
        };
        panic!("No child with name {}", child_name);        
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        print!("{} -> ", self.name);
        for child in &self.children {
            print!("{}, ", child.name);
        }
        println!("");
        for child in &self.children {
            child.print();
        }
    }

    #[allow(dead_code)]
    fn print_dir_sizes(&self) {
        if self.node_type == NodeType::Folder { println!("{} ({})", self.name, self.size); };
        for child in &self.children {
            child.print_dir_sizes();
        }
    }

    fn compute_sub_sizes(&mut self) -> u64 {
        let mut size = self.size;
        
        for child in &mut self.children {
            let child_size = child.compute_sub_sizes();
            size += child_size;
        }

        self.size = size;
        self.size
    }

    fn get_sub_100k_dir_sizes_sum(&self) -> u64 {
        if self.node_type == NodeType::File { return 0; }

        let mut size = if self.size < 100000 {self.size} else {0};
        
        for child in &self.children {
            size += child.get_sub_100k_dir_sizes_sum();
        }

        size
    }
}

impl FileTree {
    pub fn append_child(&mut self, path: &Vec<String>, child: Node) {
        let mut node = &mut self.root;
        for p in path {
            if p == "ROOT" {
                continue;
            }
            node = node.get_child_node_mut(p);
        }
        node.children.push(child);
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.root.print();
    }    

    #[allow(dead_code)]
    pub fn print_dir_sizes(&self) {
        self.root.print_dir_sizes();
    }

    pub fn compute_sizes(&mut self) {
        self.root.compute_sub_sizes();
    }

    pub fn get_sub_100k_dir_sizes_sum(&self) -> u64 {
        self.root.get_sub_100k_dir_sizes_sum()
    }
}
