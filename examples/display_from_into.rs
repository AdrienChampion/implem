use implem::implem;

pub struct MyStruct {
    s: String,
}
implem! {
    for MyStruct {
        Display {
            |&self, fmt| write!(fmt, "{{ s: `{}` }}", self.s)
        }
        From<String> {
            |s| Self { s }
        }
    }
    impl('a) for MyStruct {
        From<&'a str> {
            |s| Self { s: s.to_string() }
        }
    }
    impl('a) for &'a MyStruct {
        Into<&'a str> {
            |self| &self.s
        }
    }
}

fn main() {
    let mut val = MyStruct::from("cat");
    println!("val: {}", val);
    assert_eq!("{ s: `cat` }", &val.to_string());

    val = "dog".to_string().into();
    println!("val: {}", val);
    assert_eq!("{ s: `dog` }", &val.to_string());

    let s: &str = {
        let val_ref = &val;
        val_ref.into()
    };
    println!("s: {}", s);
    assert_eq!("dog", s);

    println!();
    println!("done");
}
