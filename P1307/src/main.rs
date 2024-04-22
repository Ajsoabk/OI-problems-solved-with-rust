fn main() {
    let mut lines = Input::new();
    let mut num = lines.read::<i32>();
    let mut ans = 0i32;
    while num!=0{
        ans=ans*10+num%10;
        num/=10;
    }
    println!("{ans}");
}

struct Input<'a>{
    lines: std::io::Lines<std::io::StdinLock<'a>>,
    s:Vec<String>,
    pos:usize,
}
impl Input<'_>{
    fn new()->Self{
        Self{
            lines:std::io::stdin().lines(),
            s: Vec::new(),
            pos:0usize,
        }
    }
    fn read<T>(& mut self)->T where T:std::str::FromStr, T::Err: std::fmt::Debug{
        if self.pos==self.s.len(){
            self.s = self.lines.next().unwrap().expect("faled to read a line").trim().split(' ').map(|s|s.to_string()).collect();
            self.pos=0;
        }
        self.pos+=1;
        self.s[self.pos-1].parse().unwrap()
    }
}
