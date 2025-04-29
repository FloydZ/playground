pub fn generate() -> String {
    let t1: &Vec<String> = &vec![
        String::from("A partridge in a pear tree\n"),
        String::from("Two turtle doves,\n"),
        String::from("Three French hens,\n"),
    ];
    let t2: Vec<&str> = vec![
        "first",
        "second",
        "third",
    ];
    let mut r = String::from("");
    for i in 0..3 {
        let s = t2[i];
        r += &format!("On the {s} day of Christmas my true love sent to me\n");
        for j in t1.into_iter().take(i+1).rev() {
            r += &j;
        }
        r += "\n";
    };
    return r;
}
