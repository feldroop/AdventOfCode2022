pub struct TreeHeightHistory {
    steps_since_larger_or_equal_height: [u32; 10],
}

impl TreeHeightHistory {
    pub fn new() -> Self {
        TreeHeightHistory {
            steps_since_larger_or_equal_height: [0; 10],
        }
    }

    pub fn get_viewing_distance_and_update(&mut self, tree_height: u8) -> u32 {
        assert!(tree_height < 10, "Tree height should be a decimal digit");
        let tree_height = tree_height as usize;

        let viewing_distance = self.steps_since_larger_or_equal_height[tree_height];

        // reset smaller trees
        for steps in &mut self.steps_since_larger_or_equal_height[..=tree_height] {
            *steps = 1;
        }

        // increment larger trees
        for steps in &mut self.steps_since_larger_or_equal_height[(tree_height + 1)..] {
            *steps += 1;
        }

        viewing_distance
    }
}

mod tests {
    #[test]
    fn simple_test() {
        let mut tree_height_history = super::TreeHeightHistory::new();
        let tree_heights = [1, 3, 2, 2, 4, 5, 4, 6, 8, 7, 4, 3, 6];
        let expected_viewing_distances = [0, 1, 1, 1, 4, 5, 1, 7, 8, 1, 1, 1, 3];

        let viewing_distances: Vec<_> = tree_heights
            .into_iter()
            .map(|tree_height| tree_height_history.get_viewing_distance_and_update(tree_height))
            .collect();

        assert_eq!(viewing_distances, expected_viewing_distances);
    }
}
