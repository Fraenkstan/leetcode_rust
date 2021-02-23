use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::hanota;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test() {
        let mut a = vec![2, 1, 0];
        let mut b = vec![];
        let mut c = vec![];
        println!("{:?}", hanota(&mut a, &mut b, &mut c));
    }
}


pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) -> Vec<HashMap<String, Vec<i32>>> {
    let mut records : Vec<HashMap<String, Vec<i32>>> = vec![];
    move_plate(a.len(), a, b, c, &mut records);
    records
}

fn move_plate(num: usize, a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>, records: &mut Vec<HashMap<String, Vec<i32>>>) {
    if num == 1 {
        let n = a.pop().unwrap();
        c.push(n);
        return;
    }
    move_plate(num - 1, a, c, b, records);
    let n = a.pop().unwrap();
    c.push(n);
    move_plate(num - 1, b, a, c, records);
    let mut record: HashMap<String, Vec<i32>> = HashMap::new();
    record.insert(String::from("a"), a.clone());
    record.insert(String::from("b"), b.clone());
    record.insert(String::from("c"), c.clone());
    records.push(record);
}

