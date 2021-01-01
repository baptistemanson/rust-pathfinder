#[derive(Debug)]
struct In<'a> {
    id: &'a str
}

#[derive(Debug)]
struct Out<'a> {
    id: &'a str
}

// Question: can I avoid using 'static here?
// Those lifetimes seem incorrect but it compiles
fn transform<'a>(input: Vec<&'static In>) -> Vec<Out<'a>> {
    let mut out: Vec<Out> = Vec::new();
    for e in input {
        out.push(Out {
            id: e.id.clone() // id is a &str, I clone it
        })
    }
    out
}

fn main() {
    let input = vec![&In {id: "1"}, &In {id: "2"}];
    let output = transform(input);
    println!("{:?}", output);
}
