pub struct DisjointSet {
    connected_component_count : usize,
    indices : Vec<usize>,
}

impl DisjointSet {
    /// Construct a new disjoint set with size elements.
    pub fn new(size : usize) -> Self {
        Self {
            connected_component_count : size,
            indices : (0..size).collect()
        }
    }

    /// Number of connected components.
    pub fn connected_component_count(&self) -> usize {
        self.connected_component_count
    }

    /// Return a vector containing a label for each element such that element in the same connected
    /// components have the same label.
    ///
    /// The labels are in the range 0..[Self::connected_component_count()].
    pub fn connceted_component_labels(&mut self) -> Vec<usize> {
        let mut labels = vec![0; self.indices.len()];

        // 1: Label all root node
        let mut next_label = 0;
        for index in 0..self.indices.len() {
            let root = self.find(index);
            if index == root {
                labels[index] = next_label;
                next_label += 1;
            }
        }

        // 2: Label all non-root node
        for index in 0..self.indices.len() {
            let root = self.find(index);
            if index != root {
                labels[index] = labels[root];
            }
        }

        labels
    }

    /// Disjoint-Set Find Algorithm with Path-Compression.
    pub fn find(&mut self, index : usize) -> usize {
        let parent = self.parent(index);
        if parent == index {
            return index;
        }

        let root = self.find(parent);
        self.set_parent(index, root); // Path-Compression
        root
    }

    /// Disjoint-Set Merge Algorithm.
    pub fn merge(&mut self, index1 : usize, index2 : usize) {
        let index1 = self.find(index1);
        let index2 = self.find(index2);
        if index1 != index2 {
            self.set_parent(index1, index2);
            self.connected_component_count -= 1;
        }
    }

    fn set_parent(&mut self, index : usize, parent : usize) {
        self.indices[index] = parent;
    }

    fn parent(&self, index : usize) -> usize {
        self.indices[index]
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut set = DisjointSet::new(5);

        assert!(set.find(0) == 0);
        assert!(set.find(1) == 1);
        assert!(set.find(2) == 2);
        assert!(set.find(3) == 3);
        assert!(set.find(4) == 4);

        set.merge(0, 1); assert_eq!(set.connected_component_count(), 4); assert_eq!(set.connceted_component_labels(), [0, 0, 1, 2, 3]);
        set.merge(2, 3); assert_eq!(set.connected_component_count(), 3); assert_eq!(set.connceted_component_labels(), [0, 0, 1, 1, 2]);
        set.merge(1, 2); assert_eq!(set.connected_component_count(), 2); assert_eq!(set.connceted_component_labels(), [0, 0, 0, 0, 1]);

        set.merge(0, 1); assert_eq!(set.connected_component_count(), 2); assert_eq!(set.connceted_component_labels(), [0, 0, 0, 0, 1]);
        set.merge(2, 3); assert_eq!(set.connected_component_count(), 2); assert_eq!(set.connceted_component_labels(), [0, 0, 0, 0, 1]);
        set.merge(1, 2); assert_eq!(set.connected_component_count(), 2); assert_eq!(set.connceted_component_labels(), [0, 0, 0, 0, 1]);

        assert_eq!(set.find(0), set.find(2));
        assert_eq!(set.find(1), set.find(3));
        assert_eq!(set.find(2), set.find(1));
        assert_eq!(set.find(3), set.find(0));

        assert_ne!(set.find(0), set.find(4));
        assert_ne!(set.find(1), set.find(4));
        assert_ne!(set.find(2), set.find(4));
        assert_ne!(set.find(3), set.find(4));

        set.merge(2, 4); assert_eq!(set.connected_component_count(), 1); assert_eq!(set.connceted_component_labels(), [0, 0, 0, 0, 0]);
        set.merge(2, 4); assert_eq!(set.connected_component_count(), 1); assert_eq!(set.connceted_component_labels(), [0, 0, 0, 0, 0]);

        assert_eq!(set.find(0), set.find(3));
        assert_eq!(set.find(1), set.find(3));
        assert_eq!(set.find(2), set.find(1));
        assert_eq!(set.find(3), set.find(0));
        assert_eq!(set.find(3), set.find(1));
        assert_eq!(set.find(3), set.find(4));
        assert_eq!(set.find(4), set.find(0));
        assert_eq!(set.find(4), set.find(2));
    }
}
