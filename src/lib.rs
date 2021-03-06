#![allow(dead_code)]
#![feature(integer_atomics)]
use std::sync::atomic::{AtomicI32, Ordering};

struct BankAccount {
    balance: AtomicI32,
}

impl BankAccount {
    pub fn open() -> BankAccount {
        BankAccount { balance: AtomicI32::new(0) }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance.load(Ordering::Relaxed)
    }

    pub fn update_balance(&mut self, amount: i32) {
        self.balance.fetch_add(amount, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use super::BankAccount;
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
        let thread_account = account.clone();
        let mut handles = Vec::new();
        let handle1 = thread::spawn(move || {
            thread_account.lock().unwrap().update_balance(10);
        });

        let thread_account_second = account.clone();
        let handle2 = thread::spawn(move || {
            let mut account = thread_account_second.lock().unwrap();
            account.update_balance(20);
            account.update_balance(-10);
        });

        handles.push(handle1);
        handles.push(handle2);

        for handle in handles {
            let _ = handle.join();
        }

        assert_eq!(account.lock().unwrap().get_balance(), 20);
    }
}
