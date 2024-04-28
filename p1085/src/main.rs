fn main() {
    Input line = Input::new();
    let mut ans=0,ind;
    for i in 0..7{
        let (a,b)=(read<int>(),read<int>());
        if a+b>ans{
            ans = a+b;
            ind = i;
        }
    }
    println!("{}",if ans>8{ind+1}else{0});
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
