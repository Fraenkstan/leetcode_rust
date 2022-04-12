// use std::cmp::max;
//
// const DIRECTIONS: [[i32; 2]; 4] = [[0,1], [1,0], [-1,0], [0,-1]];
//
// pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
//
//     let rows = grid.len();
//     let cols = grid[0].len();
//
//     // 第1步：把 grid 中的砖头全部击碎，通常算法问题不能修改输入数据，这一步非必需，可以认为是一种答题规范
//     let mut copy = grid.clone();
//     // 把 copy 中的砖头全部击碎
//     hits.iter().for_each(|node| {
//         copy[node[0] as usize][node[1] as usize] = 0;
//     });
//
//     // 第2步：建图，把砖块和砖块的连接关系输入并查集，size 表示二维网格的大小，也表示虚拟的「屋顶」在并查集中的编号
//     let size = rows * cols;
//     let mut union_find = UnionFind1::new(size + 1);
//     // 将下标为 0 的这一行的砖块与「屋顶」相连
//     for j in 0..cols {
//         if copy[0][j] == 1 {
//             union_find.union(j as i32, size as i32);
//         }
//     }
//     // 其余网格，如果是砖块向上、向左看一下，如果也是砖块，在并查集中进行合并
//     for i in 0..rows {
//         for j in 0..cols {
//             if copy[i][j] == 1 {
//                 if copy[i - 1][j] == 1 {
//                     union_find.union(get_index((i - 1) as i32, j as i32, cols),
//                                      get_index(i as i32, j as i32, cols));
//                 }
//                 if j > 0 && copy[i][j - 1] == 1 {
//                     union_find.union(get_index(i as i32, (j - 1) as i32, cols),
//                                      get_index(i as i32, j as i32, cols));
//                 }
//             }
//         }
//     }
//
//     // 第3步：按照 hits 的逆序，在 copy 中补回砖块，把每一次因为补回砖块而与屋顶相连的砖块的增量记录到 res 数组中
//     let hits_len = hits.len();
//     let mut res = Vec::with_capacity(hits_len);
//     for i in hits_len - 1 .. 0 {
//         let x = hits[i][0] as usize;
//         let y = hits[i][1] as usize;
//
//         // 注意：这里不能用 copy，语义上表示，如果原来在 grid 中，这一块是空白，这一步不会产生任何砖块掉落
//         // 逆向补回的时候，与屋顶相连的砖块数量也肯定不会增加
//         if grid[x as usize][y as usize] == 0 {
//             continue;
//         }
//         // 补回之前与屋顶相连的砖块数
//         let origin = union_find.get_size(size as i32);
//         // 注意：如果补回的这个结点在第 1 行，要告诉并查集它与屋顶相连（逻辑同第 2 步）
//         if x == 0 {
//             union_find.union(y as i32, size as i32);
//         }
//         // 在 4 个方向上看一下，如果相邻的 4 个方向有砖块，合并它们
//         DIRECTIONS.iter().for_each(|direction| {
//             let new_x = x as i32 + direction[0];
//             let new_y = y as i32 + direction[1];
//             if in_area(new_x as i32, new_y as i32, rows as i32, cols) &&
//                 copy[new_x as usize][new_y as usize] == 1 {
//                 union_find.union(get_index(x as i32, y as i32, cols),
//                                  get_index(new_x as i32, new_y as i32, cols));
//             }
//         });
//         // 补回之后与屋顶相连的砖块数
//         let current = union_find.get_size(size as i32);
//         // 减去的 1 是逆向补回的砖块（正向移除的砖块），与 0 比较大小，是因为存在一种情况，添加当前砖块，不会使得与屋顶连接的砖块数更多
//         res[i] = max(0 as i32, (current - origin - 1) as i32);
//         // 真正补上这个砖块
//         copy[x][y] = 1;
//     }
//     res
// }
//
// fn in_area(x: i32, y: i32, rows: i32, cols: usize) -> bool {
//     x >= 0 && x < rows && y >= 0 && y < cols as i32
// }
//
// fn get_index(x: i32, y: i32, cols: usize) -> i32 {
//     x * (cols as i32) + y
// }
//
// pub struct UnionFind1 {
//
//     //当前节点的父节点
//     parents: Vec<i32>,
//
//     //以当前结点为根结点的子树的结点总数
//     size: Vec<i32>
// }
//
// impl UnionFind1 {
//
//     pub fn new(size: usize) -> UnionFind1 {
//         UnionFind1{
//             parents : {
//                 let mut vec = vec![];
//                 for i in 0..size as i32 {
//                     vec.push(i);
//                 }
//                 vec
//             },
//
//             size : {
//                 let mut vec = vec![];
//                 for _i in 0..size {
//                     vec.push(1);
//                 }
//                 vec
//             }
//         }
//     }
//
//     pub fn find(&self, x: i32) -> i32 {
//         let mut root = self.parents[x as usize] as i32;
//         if x != root {
//             root = self.find(root);
//         }
//         root
//     }
//
//     pub fn union(&mut self, x: i32, y:i32) {
//         let rootx = self.find(x);
//         let rooty = self.find(y);
//         if rootx == rooty {
//             return;
//         }
//         self.parents[rootx as usize] = rooty;
//         self.size[rooty as usize] += self.size[rootx as usize];
//     }
//
//     pub fn get_size(&self, x: i32) -> i32 {
//         self.size[self.find(x) as usize]
//     }
// }
