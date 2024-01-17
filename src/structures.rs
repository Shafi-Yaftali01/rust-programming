// Structure in Rust 

struct BankAccount {
  balance: i32,
  verified: bool

}

fn print_balance(account: BankAccount) {
  println!("{:?}", account.balance);

}

fn print_verified(account: &BankAccount) {
  println!("{:?}", account.verified);
}


fn main() {
  let my_account = BankAccount {
    balance: 333,
    verified: true
  };

  // println!("{:?}", my_account.balance);
  // println!("{:?}", my_account.verified);

  // Borrowing
  print_balance(&my_account);
  print_verified(&my_account);



}
