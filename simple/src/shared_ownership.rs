use std::rc::Rc;

pub fn rc_clone() {
    let rc_clone;
    {
        let rc = Rc::new(1);
        rc_clone = rc.clone();
// rc gets out of scope here but as a "shared owner", rc_clone
// keeps the underlying data alive.
    }
    println!("{}", rc_clone);
}

pub fn box_clone(){
    let b_ref;
    {
        let b = Box::new(1);
        b_ref = &b;
        // b gets out of scope here and since it is the only owner,
        // the underlying data gets dropped.
    }
    println!("{}", b_ref);
}