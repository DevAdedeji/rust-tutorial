fn main() {
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 54.5,
        height: 60.4,
    };

    println!("{}", rect1.area());

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    impl User {
        fn build_user(email: String, username: String) -> User {
            User {
                active: true,
                username,
                email,
                sign_in_count: 1,
            }
        }
    }

    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // Immutable borrow
    account.check_balance();

    // Mutable borrow
    account.withdraw(50.50);
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
        println!("Balance is: {}", self.balance);
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {}",
            self.owner, self.balance
        );
    }
}
