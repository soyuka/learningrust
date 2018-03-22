fn main() {
    // constant type i32, f64
    const TEST: i32 = 0x000000;
    println!("The value of TEST is: {}", TEST);
    // var is mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // char is a unicode carac, multiple bytes
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // same as let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tuple is: ({}, {}, {})", x, y, z);
    println!("The value of tuple is: ({}, {}, {})", tup.0, tup.2, tup.2);

    // expression, defines a new scope
    let y = {
        let a = x - 500; //x is 500 from tuple
        a + 1
    };

    // y is 1
    println!("The value of y is: {}", y);

    println!("The value of another_snake(0, 2) is: {}", another_snake(0, 2));
    println!("The value of another_snake(1, 2) is: {}", another_snake(1, 2));

    let num = if y == 1 {
        5
    } else {
        6
    };

    println!("Num is: {}", num);

    let mut n = 0;

    loop {
        n += 1;
        println!("n is: {}", n);

        if n == 2 {
            break;
        }
    }

    // while n {}
    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("the val is: {}", element / 10);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // test with a struct
    // expression, defines a new scope, trying understand ownership
    // let mut e = 0;
    //
    // {
    //     e = 1; //x is 500 from tuple
    // };
    //
    // println!("Test {}", e);

    let my_str = "First Word of this string";

    println!("First Word of this string: {}", first_word(my_str));

    let mut poney = gimme_a_poney(String::from("PONEY"), String::from("PINK"));

    println!("My poney name is {} and its color is {}", poney.name, poney.color);
    poney.color = String::from("MULTI"); // coz mut
    poney.disable();

    println!("Show poney (needs debug annotation) {:?}", poney);

    let poney_buddy = Poney::new_poney(String::from("PONEY"), String::from("PINK"));

}

#[derive(Debug)]
struct Poney {
    name: String,
    color: String,
    active: bool
}

impl Poney {
    fn disable(&mut self) {
        self.active = !self.active;
    }

    fn new_poney(name: String, color: String) -> Poney {
        Poney { name, color, active: true }
    }
}

fn gimme_a_poney(name: String, color: String) -> Poney {
    Poney {
        name,
        color,
        active: true
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

// use snake
fn another_snake(a: i32, b: i32) -> i32 {
    if a == 0 {
        return a //works with and without semicol
    }

    b //no semicol here
}
