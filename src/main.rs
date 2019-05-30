use dlm_reg;

fn main() {
    for _ in 0..6 {
        match dlm_reg::create_reg("Tue Oct 01 2019", "Ivan") {
            Ok(_) => println!("Reg created"),
            Err(e) => println!("{}", e),
        }
    }

    for reg in dlm_reg::get_all_regs().iter() {
        println!("{}: {}", reg.date, reg.user);
    }
}
