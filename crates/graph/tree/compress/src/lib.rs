//! Explanation

use hld::Hld;

extern crate hld;

#[derive(Debug, Clone)]
pub struct CompressedTree<'a> {
    hld: &'a Hld,
    // TODO: use edge_list
    tree: Vec<Vec<u32>>,
    used_vertices: Vec<usize>,
    pub is_target: Vec<bool>,
    root: Option<usize>,
}

impl<'a> CompressedTree<'a> {
    pub fn new(hld: &'a Hld) -> Self {
        let n = hld.len();
        Self {
            hld,
            tree: vec![vec![]; n],
            used_vertices: vec![],
            is_target: vec![false; n],
            root: None,
        }
    }

    pub fn clear(&mut self) {
        for v in self.used_vertices.drain(..) {
            self.is_target[v] = false;
            self.tree[v].clear();
        }
        self.root = None;
    }

    pub fn compress(&mut self, mut vertices: Vec<usize>) -> bool {
        self.clear();
        if vertices.is_empty() {
            return false;
        }

        // mark vertices as target
        for v in &vertices {
            self.is_target[*v] = true;
        }

        // get intermediate vertices
        vertices.sort_unstable_by_key(|v| self.hld.index(*v));
        let intermediates = vertices
            .windows(2)
            .map(|w| self.hld.lca(w[0], w[1]))
            .collect::<Vec<_>>();
        vertices.extend(intermediates);
        vertices.sort_unstable_by_key(|v| self.hld.index(*v));
        vertices.dedup();
        self.root = Some(vertices[0]);

        // connect edges
        for w in vertices.windows(2) {
            let par = self.hld.lca(w[0], w[1]);
            self.tree[par].push(w[1] as u32)
        }
        self.used_vertices = vertices;

        true
    }

    pub fn dfs<V>(
        &mut self,
        compute: impl Fn(&mut Self, usize, Vec<(usize, V)>) -> V + Copy,
    ) -> Option<V> {
        let Some(root) = self.root else {
			return None;
		};
        Some(self.dfs_inner(root, compute))
    }

    fn dfs_inner<V>(
        &mut self,
        v: usize,
        compute: impl Fn(&mut Self, usize, Vec<(usize, V)>) -> V + Copy,
    ) -> V {
        let mut sons = vec![];
        for u in self.tree[v].clone() {
            let val = self.dfs_inner(u as usize, compute);
            sons.push((u as usize, val));
        }
        compute(self, v, sons)
    }
}
