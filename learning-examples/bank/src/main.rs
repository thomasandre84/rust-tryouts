#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    }

    fn summary(&self) -> String {
        format!(
            "Account ID: {}, Holder: {}, Balance: {}Cents",
            self.id, self.holder, self.balance
        )
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn summary(&self) -> Vec<String> {
        self.accounts.iter()
        .map(|acc| acc.summary())
        .collect()
    }
}


fn main() {
    let mut bank = Bank::new();

    let mut account1 = Account::new(1, "Alice".to_string());
    account1.deposit(1000);
    bank.add_account(account1);

    let mut account2 = Account::new(2, "Bob".to_string());
    account2.deposit(500);
    bank.add_account(account2);

    for summary in bank.summary() {
        println!("{}", summary);
    }
}
