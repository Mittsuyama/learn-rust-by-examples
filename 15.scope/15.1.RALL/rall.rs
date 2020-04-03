fn create_box() {
    let _box1 = Box::new(1i32);
    // _box1 会在这里销毁
}

fn main() {
    let _box2 = Box::new(5i32);
    {
        let _box3 = Box::new(4i32);
        // _box3 在此处销毁
    }
    
    for _ in 0..1000 {
        create_box();
    }
    
    // _box2 在此处销毁
}

