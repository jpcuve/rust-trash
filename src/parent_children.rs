use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_string_vector(){
    let mut v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    println!("{:?}", v);
    println!("{:?}", v[1]);
    v[1] = "coucou".to_string();
    println!("{:?}", v);
}

#[test]
fn test_optional_string_vector(){
    let mut v = vec![Some("a".to_string()), Some("b".to_string()), Some("c".to_string())];
    println!("{:?}", v);
    v[1] = None;
    println!("{:?}", v);
    let s = v[2].take();
    println!("{:?}", v);
    v[0].replace("coucou".to_string());
    println!("{:?}", v);
}

#[derive(Default, Debug)]
struct Parent {
    children: Vec<Child>
}

#[derive(Default, Debug)]
struct Child {
    parent: Option<Rc<RefCell<Parent>>>
}

#[test]
fn test_family(){
    let mut parent = Rc::new(RefCell::new(Parent::default()));
    // parent.children.push(child);  // this does not work because cannot borrow mutable from Rc; hence RefCell
    let mut p = parent.borrow_mut();
    let mut child = Child::default();
    child.parent = Some(parent.clone());
    p.children.push(child);
    // println!("{:?}", parent);  // recursive calls...
}
