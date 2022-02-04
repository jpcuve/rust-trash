
trait Speaker {
    fn say(&self);
}

struct A;
struct B;
struct C;

impl Speaker for A {
    fn say(&self) {
        println!("A!");
    }
}

impl Speaker for B {
    fn say(&self) {
        println!("B!");
    }
}

impl Speaker for C {
    fn say(&self) {
        println!("C!");
    }
}

enum Test {  // the big difference is we have here just 3 well defined objects
    D,
    E,
    F,
}

impl Speaker for Test {
    fn say(&self) {
        match self {
            &Test::D => println!("D!"),
            &Test::E => println!("E!"),
            &Test::F => println!("F!"),
        }
    }
}

pub fn inter() {
    println!("Hello, world 2!");
    let aa = vec!(A{}, A{}, A{});
    let bb = vec!(B{}, B{});
    let cc = vec!(C{}, C{}, C{}, C{});
    let mut v: Vec<Box<dyn Speaker>> = vec!();
    for a in aa.iter(){
        v.push(Box::new(A{}));
    }
    for b in bb.iter(){
        v.push(Box::new(B{}));
    }
    for c in cc.iter(){
        v.push(Box::new(C{}));
    }
    for e in v.iter(){
        e.say();
    }
    let mut v2: Vec<Test> = vec!();
    v2.push(Test::F);
    v2.push(Test::F);
    v2.push(Test::E);
    for e in v2.iter(){
        e.say();
    }
}