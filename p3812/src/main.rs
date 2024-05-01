const N:usize=51;
fn main() {
    let mut line = Input::new();
    let n = line.read::<u64>();
    let mut a=[0u64;N];
    let mut vec:Vec<u64> = Vec::with_capacity(N);
    for _ in 0..n{
        let mut tmp = line.read::<u64>();
        for i in (0..51).rev(){
            if 1u64<<i & tmp !=0{
                if a[i]!=0{
                    tmp ^= a[i];    
                }else{
                    a[i]=tmp;
                    vec.push(tmp);
                    break;
                }
            } 
        }
    }
    
    let mut ans=0u64;
    for i in vec{
        if ans^i > ans{
            ans ^= i;
        }
    }
    println!("{ans}");
    // println!("{}",vec.into_iter().fold(0u64,|status,now| if status^now > status{status^now}else{status}));
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