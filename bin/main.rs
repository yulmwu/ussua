use ussua::opcode::Opcode;

fn main() {
    let a = Opcode::try_from(0xFF).unwrap();
    println!("{:?}", a);
}
