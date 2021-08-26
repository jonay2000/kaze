pub struct test{
    // Inputs
    pub a: bool, // 1 bit(s)
    // Outputs
    pub o: bool, // 1 bit(s)
}

#[allow(unused_parens)]
#[automatically_derived]
impl test {
    pub fn new() -> test {
        test {
            // Inputs
            a: false, // 1 bit(s)
            // Outputs
            o: false, // 1 bit(s)
        }
    }

    pub fn prop(&mut self) {
        self.o = !!self.a;
    }
}

