pub struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    pub fn new(balance: Vec<i64>) -> Self {
        Bank { balance }
    }

    pub fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let len = self.balance.len();
        if account1 as usize > len + 1 || account2 as usize > len + 1 {
            return false;
        }
        if self.balance[account1 as usize - 1] < money {
            return false;
        }
        self.balance[account1 as usize - 1] -= money;
        self.balance[account2 as usize - 1] += money;
        true
    }

    pub fn deposit(&mut self, account: i32, money: i64) -> bool {
        if account as usize > self.balance.len() + 1 {
            return false;
        }
        self.balance[account as usize - 1] += money;
        true
    }

    pub fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if account as usize > self.balance.len() + 1 {
            return false;
        }
        if self.balance[account as usize - 1] < money {
            return false;
        }
        self.balance[account as usize - 1] -= money;
        true
    }
}
