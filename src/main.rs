mod home {
    pub type Double = f64;
    pub type LongLong = i64;

    pub struct ForeverLove {
        pub love: Double,
    }

    impl ForeverLove {
        pub fn love_rust(&mut self) -> () {
            let mut y = self.love;
            while y > -self.love {
                y -= 0.06 * self.love;
                let mut x = -self.love;
                while x < self.love {
                    x += 0.03 * self.love;
                    let z = x * x + y * y - 0.6 * self.love;
                    print!("{}", if z * z * z - x * x * y * y * y <= 0.0 {
                        '*'
                    } else {
                        ' '
                    })
                }
                println!()
            }
        }
    }
}

fn main() {
    let mut ago: home::LongLong = 0;
    let forever: home::LongLong = 2;

    let mut you = home::ForeverLove {
        love: 2 as f64
    };
    while ago < forever {
        you.love_rust();
        ago += 1;
    }
}
