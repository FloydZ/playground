#![allow(dead_code)]

#[derive(Debug)]
struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

#[derive(Debug)]
struct Bank {
    name: String,
    credit_line: u64,
    debit_interest: u64,
    users: Vec<User>,
}

impl Bank {
    pub fn new() -> Self {
        let users = vec![
            User {name: String::from("a"), credit_line: 100, balance: 100},
            User {name: String::from("b"), credit_line: 1000, balance: -100},
        ];

        Bank { 
            name: String::from("RichBank"),
            credit_line: 100,
            debit_interest: 0,
            users,
        }
    }

    pub fn calc_balance(&self) -> (u64, u64) {
        let mut liabilities: u64 = 0;
        let mut assets: u64 = 0;

        for user in self.users.iter() {
            println!("{:?}", user);
        }

        return (liabilities, assets);
    }

    fn find_user(&mut self, src_n: &String) -> Result<&mut User, &'static str> {
        for user in self.users.iter_mut() {
            if *user.name == *src_n {
                return Ok(user);
            }
        }
        Err("kek")
    }

    // lol not really correct
    pub fn transfer(&mut self,
                    src_n: &String,
                    dst_n: &String,
                    amount: u64) -> Result<u64, &'static str>{
        let src_u = self.find_user(src_n).unwrap();
        if src_u.balance as u64 > amount {
            return Err("not enough money")
        }

        src_u.balance -= amount as i64;

        let dst_u = self.find_user(dst_n).unwrap();
        dst_u.balance -= amount as i64;
        Ok(0)
    }

    pub fn accrue_interest(&mut self, src_n: &String) -> Result<i64, &'static str>{
        let i: i64 = self.debit_interest as i64;
        let src_u = self.find_user(src_n).unwrap();
        if src_u.balance < 0 {
            src_u.balance -= src_u.balance * i;
        } else {
            src_u.balance += src_u.balance * i;
        }
        
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let b = Bank::new();
        b.calc_balance();
    }
}
