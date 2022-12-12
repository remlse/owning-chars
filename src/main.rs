use self_ref_chars::hello_world_chars;

fn main() {
    for c in hello_world_chars() {
        println!("{}", c);
    }
}
