use std::rc::Rc;
use std::cell::RefCell;

fn sharing_resource_refcell_count() {
    struct FamilyMember {
        tv: Rc<RefCell<TV>>,
    }

    #[derive(Debug)]
    struct TV {
        channel: String,
    }

    fn member_start_watch_tv() -> FamilyMember {
        let tv_is_on = Rc::new(RefCell::new(TV{channel:"BBC".to_string()}));
        FamilyMember {
            tv: tv_is_on, 
        }
    }

    let dad = member_start_watch_tv();
    let mom = FamilyMember { tv: Rc::clone(&dad.tv) };
    println!("TV channel for mom {:?}", mom.tv);

    let mut remote_control = dad.tv.borrow_mut();
    println!("TV channel {:?}", remote_control);

    remote_control.channel = "MTV".to_string();
    println!("TV channel {:?}", remote_control);
    drop(remote_control);
    println!("TV channel for mom {:?}", mom.tv);

    //add yourself as watchin person
    let myself = FamilyMember { tv: Rc::clone(&dad.tv) };
    //and show what channe; you are watching
    println!("TV channel for me {:?}", myself.tv);
    //show how many people are watching the TV
    println!("People are watching {}",Rc::strong_count(&myself.tv));
}

fn main() {
    sharing_resource_refcell_count();
}
