pub struct inverter{
    // Inputs
    pub i: bool, // 1 bit(s)
    // Outputs
    pub o: bool, // 1 bit(s)
}

#[allow(unused_parens)]
#[automatically_derived]
impl inverter {
    pub fn new() -> inverter {
        inverter {
            // Inputs
            i: false, // 1 bit(s)
            // Outputs
            o: false, // 1 bit(s)
        }
    }

    pub fn prop(&mut self) {
        self.o = !self.i;
    }
}

