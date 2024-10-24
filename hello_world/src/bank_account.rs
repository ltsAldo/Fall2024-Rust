#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    // Creates a new BankAccount with an initial balance
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    // Deposits money into the account
    // If the amount is negative, the deposit is ignored
    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    // Withdraws money from the account
    // If the amount is greater than the current balance or negative, it does nothing
    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    // Returns the current balance
    pub fn balance(&self) -> f64 {
        self.balance
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_deposit_negative() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);  // Should be ignored
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert_eq!(account.balance(), 60.0);
    }

    #[test]
    fn test_withdraw_over_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(150.0);  // Should be ignored
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_withdraw_negative() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-20.0);  // Should be ignored
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_balance() {
        let account = BankAccount::new(200.0);
        assert_eq!(account.balance(), 200.0);
    }
}