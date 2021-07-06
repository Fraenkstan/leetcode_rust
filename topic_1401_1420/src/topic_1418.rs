use std::collections::{HashSet, HashMap};

pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut names_set: HashSet<&str> = HashSet::new();
    let mut foods_cnt: HashMap<i32, HashMap<&str, i32>> = HashMap::new();
    for i in 0..orders.len() {
        let id = orders[i][1].parse::<i32>().unwrap();
        names_set.insert(&orders[i][2]);
        let map = foods_cnt.entry(id).or_insert(HashMap::new());
        *map.entry(&orders[i][2]).or_insert(0) += 1;
    }

    let mut names: Vec<&str> = names_set.iter().map(|name| *name).collect();
    names.sort();
    let mut ids: Vec<i32> = foods_cnt.iter().map(|(k, _v)| *k).collect();
    ids.sort();

    let mut table: Vec<Vec<String>> = vec![];
    let mut header: Vec<String> = vec![];
    header.push("Table".to_string());
    for name in &names {
        header.push(name.to_string());
    }
    table.push(header);
    for i in 0..foods_cnt.len() {
        let id = ids.get(i).unwrap();
        let cnt = foods_cnt.get_mut(id).unwrap();
        let mut row: Vec<String> = vec![];
        row.push(id.to_string());
        for j in 0..names.len() {
            row.push(cnt.get(names[j]).or_else(|| Some(&0)).unwrap().to_string());
        }
        table.push(row);
    }
    table
}