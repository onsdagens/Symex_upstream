use rtic_easy::task_easy;

fn main() {
    println!("Hello, world!");
}

#[task_easy(binds = PIOC, shared = [asd = 123, asd2], period=1/100)]
fn other_task() {}
