enum T<'a> {
    Int(&'a mut u32),
    Bool(&'a mut bool)
}

fn f1(d: &mut (u32, bool)) -> T {
    if d.1 {
        T::Int(&mut d.0)
    } else {
        T::Bool(&mut d.1)
    }
}

fn f11(d: &mut (u32, u32), f: bool) -> (&mut u32) {
    if f {
        &mut d.0
    } else {
        &mut d.1
    }
}

fn f4(d: &[u32]) -> (&[u32], &[u32], &[u32], &[u32]){
    let l = d.len() / 4;
    return (
        d.get(0..l).unwrap(),
        d.get(l..2*l).unwrap(),
        d.get(2*l..3*l).unwrap(),
        d.get(3*l..d.len()).unwrap()
    )
}
