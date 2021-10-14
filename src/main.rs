mod print;

// this is a comment

fn main() {
    let name = "Brian";

    println!("Hello {}!", name);

    print::run();

    ////////////////////////////////

    let inferred = 3.0;
    let explicit: i128 = 999999;
    let ch: char = 's';
    let st = "hey";
    let bo = false;

    let (a, b, c, s) = (1, 2, 3, "wedwdssd");

    println!("{:?}", (inferred, explicit, ch, st, bo));
    println!("{:?}", (a, b, c, s, '🤣'));

    println!("{}", std::i128::MAX);
    println!("{}", std::i8::MAX);
}
