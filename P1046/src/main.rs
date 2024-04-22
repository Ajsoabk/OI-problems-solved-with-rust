fn main() {
    let mut input = Input::new();
    let arr:Vec<i32> = (0..10).map(|_| input.read::<i32>()).collect();
    let height = input.read::<i32>();
    println!("{}",arr.into_iter().filter(|a|{height+30>=*a}).count());
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