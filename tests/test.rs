
#[test]
fn test0() {
    let a = [(15, 16), (20, 21)];

    println!("{}", a[0].0);
    println!("{}", a[0].1);
    println!("{}", a[1].0);
    println!("{}", a[1].1);
}

#[test]
fn test1() {
    let mut a = Vec::<usize>::new();
    a.push(5);
    a.push(6);
    a.push(7);
    a.push(8);

    println!("{}", a[0]);
    println!("{}", a[1]);
    println!("{}", a[2]);
    println!("{}", a[3]);
}