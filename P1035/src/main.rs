fn main() {
    let mut input = Input::new();
    let k = input.read::<i32>();
    let mut n = 0usize;
    let mut sum = 0f64;
    while sum<=k.into(){
        n+=1;
        sum+=1f64/(n as f64);
    }
    println!("{n}");
}
struct Input{
    input: String,
    s:Vec<String>,
    pos:usize,
}
impl Input{
    fn new()->Self{
        Self{
            input:String::new(),
            s: Vec::new(),
            pos:0usize,
        }
    }
    fn read<T>(& mut self)->T where T:std::str::FromStr, T::Err: std::fmt::Debug{
        if self.pos==self.s.len(){
            std::io::stdin().read_line(&mut self.input).unwrap();
            self.s = self.input.trim().split(' ').map(|s|s.to_string()).collect();
            self.pos=0;
        }
        self.pos+=1;
        self.s[self.pos-1].parse().unwrap()
    }
}