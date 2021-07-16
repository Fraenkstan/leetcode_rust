
pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if buildings.is_empty() { return vec![]; }

    //横坐标排序    (x,LR,bt)
    let mut b: Vec<(i32, i32, i32)> = vec![];
    for e in buildings.into_iter() {
        b.push((e[0], 0, e[2]));  //中间的 0 表示左边线
        b.push((e[1], 1, e[2]));  //中间的 1 表示右边线
    }
    b.sort();

    let mut z = std::collections::HashMap::new();       //存 关键点
    let mut bt = std::collections::BTreeMap::new();     //当作MultiSet, 还能快速找到最大值
    bt.insert(0, 1);             //地平线

    for &(x, lr, h) in b.iter() {  //从左向右扫描
        if lr == 0 {            //左边线 插入
            bt.entry(h).and_modify(|t| *t += 1).or_insert(1);
        } else {                 //右边线 删除
            *bt.get_mut(&h).unwrap() -= 1;
            if bt[&h] == 0 { bt.remove(&h); }
        }

        //  可能的关键点是(x, maxh)
        //  条件是 插入的或删除的 那个原来的h 要比maxh 大（或等）
        //  如果之前有相同高度的关键点就直接跳过
        let maxh = *bt.keys().rev().next().unwrap();
        if h >= maxh {
            z.insert(x, maxh);
        }
    }
    //排序
    let mut z: Vec<_> = z.into_iter().map(|(x, h)| vec![x, h]).collect();
    z.sort();
    //去重
    let mut pre = -1;
    z.into_iter().filter(|x| {
        if x[1] != pre {
            pre = x[1];
            true
        } else {
            false
        }
    }).collect()
}
