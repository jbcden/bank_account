#![allow(dead_code)]
use std::sync::Arc;
struct BankAccount {
    balance: Arc<i32>,
}

impl BankAccount {
    pub fn open() -> BankAccount {
        BankAccount { balance: Arc::new(0) }
    }

    pub fn get_balance(&self) -> i32 {
        *self.balance
    }

    pub fn update_balance(&mut self, amount: i32) {
        let balance: i32 = *self.balance.clone();
        let new_balance = balance + amount;
        self.balance = Arc::new(new_balance);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use super::BankAccount;
    #[test]
    fn it_sets_the_default_balance() {
        let account = BankAccount::open();
        assert_eq!(account.balance, Arc::new(0));
    }
    #[test]
    fn get_balance() {
        let account = BankAccount::open();
        assert_eq!(account.get_balance(), 0);
    }
    #[test]
    fn update_balance() {
        let mut account = BankAccount::open();
        account.update_balance(10);
        assert_eq!(account.get_balance(), 10);
    }
    #[test]
    fn update_balance_different_thread() {
        let account = Arc::new(Mutex::new(BankAccount::open()));
        let mut account = account.clone();
        thread::spawn(move || {
            account.lock().unwrap().update_balance(10);
        });
        assert_eq!(account.lock().unwrap().get_balance(), 10);
    }
}
