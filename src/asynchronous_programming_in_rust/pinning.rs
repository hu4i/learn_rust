use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
pub struct NoPin {
    a: String,
    b: *const String,
}

impl NoPin {
    pub fn new(txt: &str) -> Self {
        NoPin {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    pub fn init(&mut self) {
        self.b = &self.a;  //pointer that points in self
    }

    pub fn a(&self) -> &str {
        &self.a
    }

    pub fn b(&self) -> &String {
        assert!(!self.b.is_null(), "No no no!");
        unsafe { &*(self.b) }
    }
}

pub struct PinToStack {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl PinToStack {
    pub fn new(txt: &str) -> PinToStack {
        PinToStack {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    pub fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    pub fn a(self: Pin<& Self>) -> &str {
        &self.get_ref().a
    }

    pub fn b(self: Pin<& Self>) -> &String {
        assert!(!self.b.is_null(), "no no no!");
        unsafe { &*self.b }
    }
}

pub struct PinToHeap {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl PinToHeap {
    pub fn new(txt: &str) -> Pin<Box<Self>> {
        let t = PinToHeap { 
            a: String::from(txt), 
            b: std::ptr::null(), 
            _marker: PhantomPinned 
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }

    pub fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    pub fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

#[cfg(test)]
mod test {
    use std::pin::Pin;

    use super::{NoPin, PinToStack, PinToHeap};

    #[test]
    fn test_for_no_pin() {
        let mut test1 = NoPin::new("test1");
        test1.init();
        let mut test2 = NoPin::new("test2");
        test2.init();

        println!("a: {}, b: {}", test1.a(), test1.b());
        std::mem::swap(&mut test1, &mut test2); 
        println!("a: {}, b: {}", test2.a(), test2.b());
    }

    #[test]
    fn test_for_pin_to_stack() {
        let mut test1 = PinToStack::new("test1");
        let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
        PinToStack::init(test1.as_mut());
        let mut test2 = PinToStack::new("test2");
        let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
        PinToStack::init(test2.as_mut());

        println!("a: {}, b: {}", PinToStack::a(test1.as_ref()), PinToStack::b(test1.as_ref()));
        //std::mem::swap(test1.get_mut(), test2.get_mut());
        println!("a: {}, b: {}", PinToStack::a(test2.as_ref()), PinToStack::b(test2.as_ref()));
    }

    #[test]
    fn test_for_dangerous_pining() {
        let mut test1 = PinToStack::new("test1");
        let mut test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
        PinToStack::init(test1_pin.as_mut());

        //drop(test1_pin);
        println!(r#"test1.b points to "test1": {:?}..."#, test1.b);

        let mut test2 = PinToStack::new("test2");
        std::mem::swap(&mut test1, &mut test2);
        println!("... and now it points nowhere: {:?}", test1.b);
    }

    #[test]
    fn test_for_pin_to_heap() {
        let test1 = PinToHeap::new("test1");
        let test2 = PinToHeap::new("test2");
        
        println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
        println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
    }



}