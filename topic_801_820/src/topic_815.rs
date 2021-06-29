use std::collections::HashMap;

pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    let mut table = HashMap::new();
    table.insert(source, 0);
    bfs(&routes, &mut table);
    bfs(&routes, &mut table);
    *table.get(&target).unwrap_or(&-1)
}

fn bfs(routes: &Vec<Vec<i32>>, table: &mut HashMap<i32, i32>) {
    for i in routes.iter() {
        if let Some(&v) = i.iter().filter_map(|e| table.get(e)).min() {
            for j in i.iter() {
                if let Some(x) = table.get_mut(j) {
                    *x = (*x).min(v + 1);
                } else {
                    table.insert(*j, v + 1);
                }
            }
        }
    }
}
