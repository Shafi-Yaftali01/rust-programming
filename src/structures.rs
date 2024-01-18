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

fn is_verified(account: &BankAccount) -> Result<bool, bool> {
  match account.verified {
    true => Ok(true),
    false => Err(false)
  };
}
fn main() {
  let my_account = BankAccount {
    balance: 333,
    verified: true
  };
  let verification_status = is_verified(&my_account).expect("Unable to unwrap result.");

  // println!("{:?}", my_account.balance);
  // println!("{:?}", my_account.verified);

  // Borrowing
  print_balance(&my_account);
  print_verified(&my_account);
  println!("{:?}", verification_status);


}
