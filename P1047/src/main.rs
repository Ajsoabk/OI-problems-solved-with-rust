fn main() {
    let mut input = Input::new();
    let l = input.read::<usize>();
    let m = input.read::<usize>();
    let mut a = [false;10000+5];
    for _ in 0..m{
        let (u,v) = (input.read::<usize>(),input.read::<usize>());
        for x in u..=v{
            a[x]=true;
        }
    }
    println!("{}",(l+1)-a.into_iter().filter(|a|*a).count());
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