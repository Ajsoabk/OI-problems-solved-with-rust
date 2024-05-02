#[derive(Default,Debug)]
struct Equipment{
    i :Vec<f64>,
    c :i64,
}
impl Equipment{
    fn new(i:Vec<f64>)->Self{
        Self{
            i,
            c:0i64,
        }
    }
}
const EPSILON:f64 = 0.00001;
fn main() {
    let mut line = Input::new();
    let n = line.read::<usize>();
    let m = line.read::<usize>();
    let mut equip:Vec<Equipment>=Vec::with_capacity(n+1);
    for _ in 0..n{
        let mut tmp:Vec<f64> = Vec::with_capacity(m);
        for _ in 0..m{
            tmp.push(line.read::<f64>());
        }
        equip.push(Equipment::new(tmp));
    }
    for e in equip.iter_mut(){
        e.c=line.read::<i64>();
    }
    equip.sort_by(|a,b| a.c.cmp(&b.c));
    let mut base:Vec<Option<Equipment>> = vec![];
    for _ in 0..m{
        base.push(None);
    }
    let mut cnt=0i32;
    let mut cost=0i64;
    for mut e in equip{
        for j in 0..m{
            if e.i[j] > EPSILON || e.i[j] < -EPSILON{
                if let Some(b) = &base[j]{
                    let ratio:f64 = e.i[j]/b.i[j];
                    for k in j..m{
                        if b.i[k] < -EPSILON || b.i[k] >EPSILON{
                            e.i[k] -= ratio*b.i[k];
                        }
                    }
                }else{
                    cost += e.c;
                    base[j]=Some(e);
                    cnt += 1;
                    break;
                }
            }
        }
    }
    println!("{cnt} {cost}");
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
