use std::rc::Rc;

#[derive(Debug, Default)]
struct Parent<'a> {
    children: Vec<Child<'a>>,
}

#[derive(Debug)]
struct Child<'a> {
    parent: Rc<&'a Parent<'a>>,
}

impl <'a> Parent<'a> {
    fn add_child(&mut self, child: Child<'a>){
        self.children.push(child);
    }
}

impl <'a> Child<'a> {
    fn new(parent: Rc<&'a Parent<'a>>) -> Self {
        Child {
            parent
        }
    }
}

#[test]
fn test_birth(){
    let mut parent = Parent::default();
    let parent_ref = Rc::new(&parent);
    let child = Child::new(parent_ref.clone());
    parent.add_child(child);
}