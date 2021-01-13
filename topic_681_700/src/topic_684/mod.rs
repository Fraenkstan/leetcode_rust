use data_structure::union_find::UnionFind;

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut uf = UnionFind::new(n);
    for edge in edges {
        if uf.find(edge[0] as usize - 1) == uf.find(edge[1] as usize - 1){
            return edge;
        }

        uf.union(edge[0] as usize - 1 , edge[1] as usize - 1);
    }
    vec![]
}