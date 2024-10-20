pub struct Solution;

#[derive(Debug, Default, Clone, Copy)]
enum NodeStatus {
    #[default]
    New,
    Discovered,
    Traversed,
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // leetcode assures that
        // 1. `prerequisites` contains unique entries
        // 2. they are all pairs
        let mut adjacency_list = Vec::from_iter((0..num_courses).map(|_| Vec::<usize>::new()));
        prerequisites.iter().for_each(|pair| {
            adjacency_list[pair[0] as usize].push(pair[1] as usize);
        });
        Self::topological_sort(&adjacency_list).is_ok()
    }

    /// Either topoligical sort the graph, or a list containing a cycle
    fn topological_sort(adj_list: &[Vec<usize>]) -> Result<Vec<usize>, Vec<usize>> {
        use NodeStatus::*;
        if adj_list.is_empty() {
            return Ok(vec![]);
        }
        let mut node_statuses = vec![NodeStatus::default(); adj_list.len()];
        let mut which_neighbor_to_dfs = vec![0; adj_list.len()];
        let mut dfs_stack = Vec::with_capacity(adj_list.len());
        let mut topological_sorted = Vec::with_capacity(adj_list.len());
        if adj_list
            .iter()
            .enumerate()
            .try_for_each(|(node_idx, _)| {
                match node_statuses[node_idx] {
                    Discovered => unreachable!(),
                    New => {
                        node_statuses[node_idx] = Discovered;
                        dfs_stack.push(node_idx)
                    }
                    Traversed => { /* no-op */ }
                }
                'dfs_frontier: while let Some(&node_idx) = dfs_stack.last() {
                    for (next_time_start_with_this_neighbor, &neighbor_idx) in adj_list[node_idx]
                        .iter()
                        .enumerate()
                        .skip(which_neighbor_to_dfs[node_idx])
                    {
                        match node_statuses[neighbor_idx] {
                            Discovered => {
                                dfs_stack.push(neighbor_idx);
                                return Err(());
                            }
                            Traversed => { /* no-op */ }
                            New => {
                                node_statuses[node_idx] = Discovered;
                                which_neighbor_to_dfs[node_idx] =
                                    next_time_start_with_this_neighbor + 1;
                                dfs_stack.push(neighbor_idx);
                                continue 'dfs_frontier;
                            }
                        }
                    }
                    node_statuses[node_idx] = Traversed;
                    topological_sorted.push(dfs_stack.pop().unwrap());
                }
                Ok(())
            })
            .is_ok()
        {
            topological_sorted.reverse();
            Ok(topological_sorted)
        } else {
            Err(dfs_stack)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
        assert!(Solution::can_finish(
            10,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![2, 3],
                vec![2, 4],
                vec![3, 5],
                vec![4, 5],
                vec![4, 6],
                vec![5, 7],
                vec![6, 7],
                vec![6, 8],
                vec![7, 9],
            ]
        ));
        assert!(!Solution::can_finish(
            10,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![2, 3],
                vec![2, 4],
                vec![3, 5],
                vec![4, 5],
                vec![4, 6],
                vec![5, 7],
                vec![6, 7],
                vec![6, 8],
                vec![7, 9],
                vec![9, 0]
            ]
        ));
        assert!(Solution::can_finish(
            5,
            vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]]
        ));
    }
}
