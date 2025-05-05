use std::{marker::PhantomPinned, mem, pin, ptr};



struct SelfRef {
    name: String,
    name_ref: *const String,

    // 标记结构体为 !Unpin
    _pin: PhantomPinned,
}

impl SelfRef {

    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_ref: ptr::null(),
            _pin: PhantomPinned,
        }
    }
    fn init(&mut self) {
        self.name_ref = &raw const self.name;
    }
}

pub fn self_ref_swap() {

    let mut s1 = SelfRef::new("Hello");
    s1.init();

    let mut s2 = SelfRef::new("World");
    s2.init();

    println!("s1: name: {}, name_ref: {}", s1.name, unsafe {&*s1.name_ref});
    println!("s2: name: {}, name_ref: {}", s2.name, unsafe {&*s2.name_ref});

    // 交换内存
    // 但是这样子引用的指向并不会改变，造成数据的不一致
    mem::swap(&mut s1, &mut s2);

    println!("s1: name: {}, name_ref: {}", s1.name, unsafe {&*s1.name_ref});
    println!("s2: name: {}, name_ref: {}", s2.name, unsafe {&*s2.name_ref});

}

pub fn self_ref_pin() {

    let mut s1 = SelfRef::new("Hello");
    s1.init();
    let s1 = pin::pin!(s1);
     
    // 添加 _pin: PhantomPinned 后在编译期就标记结构体为 !Unpin 了
    // 因此无法获取可变引用
    // s1.as_mut().get_mut().init();

    let mut s2 = SelfRef::new("Hello");
    s2.init();
    let s2 = pin::pin!(s2);
     
    // 添加 _pin: PhantomPinned 后在编译期就标记结构体为 !Unpin 了
    // 因此无法获取可变引用
    // s2.as_mut().get_mut().init();
    
    println!("s1: name: {}, name_ref: {}", s1.name, unsafe {&*s1.name_ref});
    println!("s2: name: {}, name_ref: {}", s2.name, unsafe {&*s2.name_ref});

    // 交换内存
    // 无法获取到可变引用，因此无法调用该方法
    // mem::swap( s1.as_mut().get_mut(),  s2.as_mut().get_mut());

    println!("s1: name: {}, name_ref: {}", s1.name, unsafe {&*s1.name_ref});
    println!("s2: name: {}, name_ref: {}", s2.name, unsafe {&*s2.name_ref});
}
