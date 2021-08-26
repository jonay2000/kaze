
#[cfg(test)]
mod tests {
    use kaze::*;
    use std::io;
    use std::fs::File;

    #[test]
    fn inverter_def() -> io::Result<()>{
// ANCHOR: inverter
let ctx = Context::new();
let inverter = ctx.module("inverter");

// the second parameter is the bit width (which is 1 here).
let i = inverter.input("i", 1);
inverter.output("o", !i);
// ANCHOR_END: inverter
Ok(())
    }

    #[test]
    fn inverter_code() -> io::Result<()> {
let ctx = Context::new();
let inverter = ctx.module("inverter");

let i = inverter.input("i", 1);
inverter.output("o", !i);

let output_file = File::create("inverter_code.output.rs").expect("couldn't open");

// ANCHOR: inverter_code
sim::generate(inverter, sim::GenerationOptions::default(), output_file)?;
// ANCHOR_END: inverter_code
        Ok(())
    }


    #[test]
    fn inverter_test_module() -> io::Result<()> {
        let ctx = Context::new();
        let inverter = ctx.module("inverter");

        let i = inverter.input("i", 1);
        inverter.output("o", !i);

        let test = ctx.module("test");
        let inverter_instance = test.instance("inverter_instance", "inverter");
        inverter_instance.drive_input("i", test.input("a", 1));
        test.output("o", !inverter_instance.output("o"));


        let output_file = File::create("inverter_test_module.output.rs").expect("couldn't open");

// ANCHOR: test_module
        sim::generate(test, sim::GenerationOptions {
            // tracing: true,
            ..Default::default()
        }, output_file)?;
// ANCHOR_END: test_module
        Ok(())
    }
}