use dlm_reg;

fn main() {

    for reg in dlm_reg::get_all_regs().iter() {
        println!("{}: {}", reg.date, reg.user);
    }
}
