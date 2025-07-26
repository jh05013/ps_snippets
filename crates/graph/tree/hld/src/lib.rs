//! Explanation

use edge_list::Graph;

extern crate edge_list;

#[derive(Debug, Clone)]
pub struct Hld {
    /// Parent of `v`.
    /// `TODO: change to u32 and define parent[v] = v?`
    parent: Vec<Option<u32>>,
    /// Size of the subtree rooted at `v`.
    size: Vec<u32>,
    /// Depth of `v` (depth of root is 0).
    depth: Vec<u32>,
    /// The vertex immediately down the heavy path.
    heavy_child: Vec<Option<u32>>,
    /// The top vertex of the heavy path that contains `v`.
    top: Vec<u32>,
    /// In-order DFS index of `v`.
    dfs_index: Vec<u32>,
}

type Range = (usize, usize);
impl Hld {
    pub fn new<T>(tree: &Graph<T>, root: usize) -> Self {
        let n = tree.vertex_count();
        assert!(root < n);

        let mut hld = Hld {
            parent: vec![None; n],
            size: vec![0; n],
            depth: vec![0; n],
            heavy_child: vec![None; n],
            top: vec![0; n],
            dfs_index: vec![0; n],
        };
        hld.dfs_size(tree, root);
        hld.top[root] = root as u32;
        hld.dfs_hld(tree, root, 0);
        hld
    }

    /// `v` = current vertex, `p` = parent
    fn dfs_size<T>(&mut self, tree: &Graph<T>, v: usize) {
        self.size[v] = 1;
        for (u, _) in tree.neighbors(v) {
            if Some(u as u32) == self.parent[v] {
                continue;
            }
            self.parent[u] = Some(v as u32);
            self.depth[u] = self.depth[v] + 1;
            self.dfs_size(tree, u);
            self.size[v] += self.size[u];
            if let Some(other_u) = self.heavy_child[v] {
                if self.size[u] > self.size[other_u as usize] {
                    self.heavy_child[v] = Some(u as u32);
                }
            } else {
                self.heavy_child[v] = Some(u as u32);
            }
        }
    }

    fn dfs_hld<T>(&mut self, tree: &Graph<T>, v: usize, mut idx: u32) -> u32 {
        self.dfs_index[v] = idx;
        // dfs heavy child first
        let Some(first_u) = self.heavy_child[v] else { return idx; };
        self.top[first_u as usize] = self.top[v];
        idx = self.dfs_hld(tree, first_u as usize, idx + 1);
        // dfs rest
        for (u, _) in tree.neighbors(v) {
            if u as u32 == first_u || Some(u as u32) == self.parent[v] {
                continue;
            }
            self.top[u] = if Some(u as u32) == self.heavy_child[v] {
                self.top[v]
            } else {
                u as u32
            };
            idx = self.dfs_hld(tree, u, idx + 1);
        }
        idx
    }

    pub fn len(&self) -> usize {
        self.parent.len()
    }

    pub fn depth(&self, v: usize) -> u32 {
        self.depth[v]
    }

    /// Returns the ETT index of `v`.
    pub fn index(&self, v: usize) -> usize {
        self.dfs_index[v] as usize
    }

    /// Returns the ETT range that corresponds to
    /// the subtree rooted at `v`.
    pub fn subtree(&self, v: usize) -> Range {
        let l = self.dfs_index[v] as usize;
        (l, l + self.size[v] as usize - 1)
    }

    /// Returns the ETT range that corresponds to
    /// the path from `top[v]` to `v`.
    pub fn path_from_top(&self, v: usize) -> Range {
        let top = self.top[v] as usize;
        (self.dfs_index[top] as usize, self.dfs_index[v] as usize)
    }

    /// Returns the LCA of `a` and `b`.
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        loop {
            let ta = self.top[a] as usize;
            let tb = self.top[b] as usize;
            if ta == tb {
                break;
            }
            if self.depth[ta] > self.depth[tb] {
                a = self.parent[ta].unwrap() as usize;
            } else {
                b = self.parent[tb].unwrap() as usize;
            }
        }
        // a and b are now in the same heavy path
        if self.dfs_index[a] > self.dfs_index[b] {
            b
        } else {
            a
        }
    }

    /// Returns:
    /// - LCA of `a` and `b`
    /// - Two lists of ETT ranges from LCA to `a` and `b`;
    ///   the pieces in each list are arranged from bottom to top.
    ///   Each piece represents a downward path (see `path_from_top`).
    pub fn path(&self, mut a: usize, mut b: usize) -> (usize, Vec<Range>, Vec<Range>) {
        let mut a_path = vec![];
        let mut b_path = vec![];
        loop {
            let ta = self.top[a] as usize;
            let tb = self.top[b] as usize;
            if ta == tb {
                break;
            }
            if self.depth[ta] > self.depth[tb] {
                a_path.push(self.path_from_top(a));
                a = self.parent[ta].unwrap() as usize;
            } else {
                b_path.push(self.path_from_top(b));
                b = self.parent[tb].unwrap() as usize;
            }
        }
        // a and b are now in the same heavy path
        let ai = self.dfs_index[a] as usize;
        let bi = self.dfs_index[b] as usize;
        if ai > bi {
            // a is deeper
            a_path.push((bi, ai));
            (b, a_path, b_path)
        } else {
            // b is deeper
            b_path.push((ai, bi));
            (a, a_path, b_path)
        }
    }
}
