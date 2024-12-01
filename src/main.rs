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
            holder,
            balance: 0,
        }
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
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
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_holder(holder: String) {
    println!("{:#?}", holder);
}

fn change_account(account: &mut Account) {
    account.balance = 10;
}

// fn make_and_print_account() -> &Account {
//     let account = Account::new(1, String::from("me"));
//     println!("{:#?}", account);
//     &account
// }

fn main() {
    // let avcount_ref = make_and_print_account();
    // println!("{}", avcount_ref.balance);
    let mut bank = Bank::new();
    // let other_bank = bank;

    let mut account = Account::new(1, String::from("me"));
    account.deposit(500);
    account.withdraw(200);

    println!("{:#?}", account.summary());
    // let other_account = account;

    bank.add_account(account);
    // let account_ref = &account;
    // let accounts = bank.accounts;

    // let list_of_accounts = vec![account];
    // println!("{:#?}", list_of_accounts);

    println!("{:#?}", bank);
    // println!("{:#?}", bank.accounts);
    // print_account(account);
    // print_holder(account.holder);
    // print_account(account_ref);
    // print_account(&account);
    // print_account(account);
    // println!("{:#?}", account.holder);
    // change_account(&mut account);
    // println!("{:#?}", account);
}
