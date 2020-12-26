struct Outer<'a> {
    collection: Vec<&'a mut Inner>,
}

struct Inner {
    a: u64,
}

fn main() {
    let mut inner1 = Inner { a: 42 };
    let mut inner2 = Inner { a: 10 };
    {
        let mut outer = Outer {
            collection: vec![&mut inner1, &mut inner2],
        };
        for i in outer.collection.iter_mut() {
            i.a += 10;
        }
    } // free the mutable borrow?
    println!("{}", inner1.a);
}
