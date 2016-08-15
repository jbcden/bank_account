#![allow(dead_code)]
#![feature(integer_atomics)]
mod first {
    use std::sync::atomic::{AtomicI32, Ordering};

    type BankAccount = AtomicI32;

    pub fn open() -> BankAccount {
        AtomicI32::new(0)
    }

    pub fn get_balance(account: &BankAccount) -> i32 {
        account.load(Ordering::Relaxed)
    }

    pub fn update_balance(account: &mut BankAccount, amount: i32) {
        account.fetch_add(amount, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use super::first;
    #[test]
    fn get_balance() {
        let account = first::open();
        assert_eq!(first::get_balance(&account), 0);
    }
    #[test]
    fn update_balance() {
        let mut account = first::open();
        first::update_balance(&mut account, 10);
        assert_eq!(first::get_balance(&account), 10);
    }
    #[test]
    fn update_balance_different_thread() {
        let account = Arc::new(Mutex::new(first::open()));
        let thread_account = account.clone();
        let mut handles = Vec::new();
        let handle1 = thread::spawn(move || {
            first::update_balance(&mut thread_account.lock().unwrap(), 10);
        });

        let thread_account_second = account.clone();
        let handle2 = thread::spawn(move || {
            let mut account = thread_account_second.lock().unwrap();
            first::update_balance(&mut account, 20);
            first::update_balance(&mut account, -10);
        });

        handles.push(handle1);
        handles.push(handle2);

        for handle in handles {
            let _ = handle.join();
        }

        assert_eq!(first::get_balance(&account.lock().unwrap()), 20);
    }
}
