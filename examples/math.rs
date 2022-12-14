use qinetic::prelude::*;

fn main() {
    let m2x3 = Matrix2x3::new(
        Vector3::new(111.0, 222.0, 333.0),
        Vector3::new(444.0, 555.0, 666.0),
    );
    println!("{}", m2x3[..1][..1][0]);
}
