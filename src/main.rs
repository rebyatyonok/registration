use dlm_reg;

fn main() {
    let result = dlm_reg::create_reg("Tue Oct 01 2019", "Ivan");
    match result {
        Ok(_) => println!("Success"),
        Err(e) => println!("{}", e),
    }
}
