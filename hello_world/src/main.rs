mod bank_account;

fn main() {
    let mut account = bank_account::BankAccount::new(100.0);
    println!("Initial balance: {}", account.balance());

    account.deposit(50.0);
    println!("After deposit: {}", account.balance());

    account.withdraw(30.0);
    println!("After withdrawal: {}", account.balance());

    account.withdraw(150.0);  // This should not change the balance
    println!("After trying to over-withdraw: {}", account.balance());
}