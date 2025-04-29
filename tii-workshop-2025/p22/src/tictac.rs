use std::fmt::{self, format};
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]


#[derive(PartialEq, Clone, Copy)]
enum Slot {
    Empty = 0,
    X,
    Y,
}

impl Default for Slot {
    fn default() -> Self { Slot::Empty }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Slot::Empty=>  write!(f, " "),
            Slot::X =>  write!(f, "x"),
            Slot::Y =>  write!(f, "o"),
        }
    }
}


enum TicTacResult {
    WinX,
    WinY,
    WinBoth,
    GameOn,
}

impl fmt::Display for TicTacResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TicTacResult::WinX => write!(f, "X wins"),
            TicTacResult::WinY => write!(f, "Y wins"),
            TicTacResult::WinBoth => write!(f, "Both wins"),
            TicTacResult::GameOn => write!(f, "no one wins"),
        }
    }
}

#[derive(Default)]
struct TicTacField {
    field: [[Slot; 3]; 3],
}

impl fmt::Display for TicTacField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("");
        for row in self.field {
            s += &format!("{}|{}|{}\n", row[0], row[1], row[2]);
        };

        write!(f, "{}", s)
    }
}

impl TicTacField {
    pub fn new() -> TicTacField {
        return TicTacField {
            field: [[Slot::Empty; 3]; 3],
        };
    }
    pub fn analyse(&self) -> TicTacResult {
        let mut winX = false;
        let mut winY = false;

        // first analyse rows
        for row in self.field {
            if row.iter().all(|x| *x == row[0]) {
                match row[0] {
                    Slot::X => winX = true,
                    Slot::Y => winY = true,
                    Slot::Empty => {},
                }
            }
        };


        // next analyse cols
        for col in 0..3{
            let t = self.field[0][col];
            let mut all_same = true;
            for row in 1..3 {
                if t != self.field[row][col] {
                    all_same = false;
                    break;
                }
            };

            if all_same {
                match self.field[0][col] {
                    Slot::X => winX = true,
                    Slot::Y => winY = true,
                    Slot::Empty => {},
                }
            }
        };

        // next analyse diagonals
        let mut as1 = true;
        let mut as2 = true;
        let t1 = self.field[0][0];
        let t2 = self.field[0][2];
        for i in 1..3 {
            if t1 != self.field[i][i] { as1 = false; }
            if t2 != self.field[2-i][2-i] { as2 = false; }
        }

        if as1 {
            match self.field[0][0] {
                Slot::X => winX = true,
                Slot::Y => winY = true,
                Slot::Empty => {},
            }
        }
        
        if as2 {
            match self.field[0][0] {
                Slot::X => winX = true,
                Slot::Y => winY = true,
                Slot::Empty => {},
            }
        }

        if winX && winY { return TicTacResult::WinBoth; }
        if winX { return TicTacResult::WinX; }
        if winY { return TicTacResult::WinY; }
        return TicTacResult::GameOn;
    } 

    fn make_move(&mut self, x: u32, y: u32, player: Slot) -> TicTacResult {
        self.field[x as usize][y as usize] = player;
        return self.analyse()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut f = TicTacField::new();
        let mut t = f.analyse();

        t = f.make_move(0, 0, Slot::X);
        println!("{} {}", f, t);
        t = f.make_move(0, 1, Slot::Y);
        println!("{} {}", f, t);
        t = f.make_move(1, 0, Slot::X);
        println!("{} {}", f, t);
        t = f.make_move(2, 0, Slot::X);
        println!("{} {}", f, t);
    }
}
