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
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_holder(holder: String) {
    println!("{:#?}", holder);
}

fn main() {
    // let bank = Bank::new();
    // let other_bank = bank;

    let account = Account::new(1, String::from("me"));
    let account_ref = &account;
    // let accounts = bank.accounts;

    // let list_of_accounts = vec![account];
    // println!("{:#?}", list_of_accounts);

    // println!("{:#?}", bank);
    // println!("{:#?}", bank.accounts);
    // print_account(account);
    // print_holder(account.holder);
    print_account(account_ref);
    // print_account(account);
    // println!("{:#?}", account.holder);
    println!("{:#?}", account);
}
