pub struct Solution;

impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        // BFS to get single source all point shortest path
        //
        // Notice since each time each node can send arbitrary amount of
        // messages, this makes the work of each node independent.
        // I.e. for each node, calculate when would it idle, and take max.

        let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); patience.len()];
        edges.iter().for_each(|e| {
            // Leetcode assumes valid edges, no duplicated edges,
            // and no self edges,
            // hence no fancy input sanity check here.
            let n0 = e.first().unwrap().clone() as usize;
            let n1 = e.get(1).unwrap().clone() as usize;
            let oob: &'static str = "edge node out of bound";
            adj_list.get_mut(n0).expect(oob).push(n1);
            adj_list.get_mut(n1).expect(oob).push(n0);
        });
        let mut distances: Vec<Option<usize>> = vec![None; patience.len()];
        let mut bfs_queue: std::collections::VecDeque<usize> = vec![].into();
        distances[0] = Some(0);
        bfs_queue.push_front(0);
        while !bfs_queue.is_empty() {
            let current = bfs_queue.pop_back().unwrap();
            adj_list[current].iter().for_each(|&neighbor| {
                if let None = distances[neighbor] {
                    bfs_queue.push_front(neighbor);
                    distances[neighbor] = distances[current].map(|d| d + 1);
                }
            })
        }
        patience
            .iter()
            .skip(1)
            .zip(distances.iter().skip(1))
            .fold(0, |max, (pat, dist)| {
                std::cmp::max(max, {
                    let dist = dist.unwrap();
                    let pat = *pat as usize;
                    let send_time_of_last_msg = (dist * 2 - 1) / pat * pat;
                    send_time_of_last_msg + dist * 2
                })
            }) as i32
            + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let patience = vec![0, 2, 1];
        assert_eq!(Solution::network_becomes_idle(edges, patience), 8);
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        let patience = vec![0, 10, 10];
        assert_eq!(Solution::network_becomes_idle(edges, patience), 3);
    }
}
