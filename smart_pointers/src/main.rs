mod custom_smart_pointer;
mod sp_drop_trait;
mod multiple_ownership_rc;
mod box_smart_pointer;

fn main() {
    box_smart_pointer::smart_pointer_with_box();
    box_as_a_reference();
    sp_drop_trait::automatic_drop_at_end_of_scope();
    sp_drop_trait::manual_drop();
    multiple_ownership_rc::rc_use_multiple_reference();
    multiple_ownership_rc::rc_dropping_increasing();
}


fn box_as_a_reference() {
    let x = 5;
    let y = custom_smart_pointer::MyBox::new(x);

    assert_eq!(5, x);
    //assert_eq!(5, *y); out custom box type is missing the Deref trait
    assert_eq!(5,*y)
    // Using * is the same as y.deref()
    // we use it as *(y.deref()) so we don't move
    // the ownership value out of self
}
