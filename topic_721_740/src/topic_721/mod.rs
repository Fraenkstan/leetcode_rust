pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut parents = std::collections::HashMap::new();
    let mut mail2account = std::collections::HashMap::new();
    for i in 0..accounts.len() {
        let name = &accounts[i][0];
        for j in 1..accounts[i].len() {
            parents.entry(&accounts[i][j]).or_insert(&accounts[i][j]);
            mail2account.entry(&accounts[i][j]).or_insert(name);
        }
    }

    for i in 0..accounts.len() {
        for j in 2..accounts[i].len() {
            let mut a = &accounts[i][j - 1];
            let mut b = &accounts[i][j];
            while &a != parents.get(a).unwrap() {
                a = parents.get(a).unwrap();
            }
            while &b != parents.get(b).unwrap() {
                b = parents.get(b).unwrap();
            }
            if a == b {
                continue;
            }
            *parents.get_mut(b).unwrap() = a;
        }
    }

    let mut sets = std::collections::HashMap::new();
    for (&k, &_v) in parents.iter() {
        let mut root = k;
        while &root != parents.get(root).unwrap() {
            root = parents.get(root).unwrap();
        }
        sets.entry(root).or_insert(Vec::new()).push(k);
    }

    let mut answer = Vec::new();
    sets.into_iter().for_each(|(head, set)| {
        let mut row = vec![mail2account.get(head).unwrap().to_string()];
        let mut mails: Vec<String> = set.into_iter().map(|s| s.to_string()).collect();
        mails.sort_unstable();
        row.append(&mut mails);
        answer.push(row);
    });
    answer
}
