use std::collections::HashMap;
/*
 * UnionFind datastructure is essentailly a forrest.
 * Every set is a rooted tree and represented by the root element.
 * In a rooted tree every node has a parent and the parent[root] == root.
 * */
struct UnionFind {
    // A set is uniquely identified by a node for which
    // the condition parent[node] == node holds true.
    parent: Vec<usize>, // parent of each node
    size: Vec<usize>, 
    // size of the set where the node with the particular index is the parent.
}

impl UnionFind {
    /*
     * Creates a new UnionFind structure.
     * */
    fn new(n: usize) -> UnionFind {
        // Nodes are labeled from 1 to n, 0 is just a placeholder.
        let parent = (0..(n + 1)).collect();
        let size = vec![1; n + 1];

        UnionFind { parent, size }
    }

    /*
     * Finds the label fo the set to which the node x belongs to
     * */
    fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }

        x
    }

    /*
     * Finds the union of two sets where the nodes x and y belongs to.
     * Returns true if both nodes belongs to disjoint sets else false.
     * */
    fn union(&mut self, mut x: usize, mut y: usize) -> bool {
        // Find the label of the set where x and y belongs to
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            // Both nodes belong the same set.
            return false;
        }

        // We want to keep the height of the rooted tree as small as possible.
        // So we make the parent of the root with bigger as as the root of 
        // the tree with smaller size.

        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];
        return true;
    }
}

/*
 * We want to remove maximum number of edges as possible.
 * We will maintain two UnionFind structs A and B to represent the tree of
 * A and B. 
 * If A and B are connected with respect to {edges} there is no reason
 * to maintain more tha n-1 edges in A and B.
 * We want to remove less number of common edges because removing a common
 * edge affects both A and B, while removing the edge associated with alice
 * does not affect the connectivity of bob.
 * */

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut a: i32 = 0; // number of edges retained by A
        let mut b: i32 = 0; // number of edges retained by B
        let mut d: i32 = 0; // count of deleted edges
        let mut A = UnionFind::new(n as usize);
        let mut B = UnionFind::new(n as usize);
        let mut a_v = vec![]; // edges exclusive to Alice
        let mut b_v = vec![]; // edges exclusive to Bob
        let m = edges.len();


        edges.iter().for_each(|edge| {
            let v = edge[0];
            if v == 1 {
                a_v.push(edge);
            } else if v == 2 {
                b_v.push(edge);
            } else {
                let _1 = A.union(edge[1] as usize, edge[2] as usize);
                let _2 = B.union(edge[1] as usize, edge[2] as usize);
                if !_1 && !_2 {
                    // if the edge is not needed for both Alice and Bob
                    // then it can be removed
                    d += 1;
                }
                if _1 {
                    // used by Alice
                    a += 1;
                }
                if _2 {
                    // used by Bob
                    b += 1;
                }
            }
        });
        a_v.iter().for_each (|edge| {
            if (A.union(edge[1] as usize, edge[2] as usize)) {
                a += 1;
            } else {
                d += 1;
            }
        });
        b_v.iter().for_each (|edge| {
            if (B.union(edge[1] as usize, edge[2] as usize)) {
                b += 1;
            } else {
                d += 1;
            }
        });
        if (a == n - 1 && b == n - 1) {
            // Forrest of a connected graph with n vertices should be a tree
            // with n-1 edges
            d
        } else {
            -1
        }
    }
}
