use implem::implem;

pub struct MyStruct1 {
    s: String,
}
implem! {
    for MyStruct1 {
        Display {
            |&self, fmt| write!(fmt, "{{ s: `{}` }}", self.s)
        }
        From<String> {
            |s| Self { s }
        }
        Deref<Target = String> {
            |&self| &self.s,
            |&mut self| &mut self.s,
        }
    }
}

pub struct MyStruct2 {
    s: String,
}
implem! {
    for MyStruct2 {
        Display {
            |&self, fmt| write!(fmt, "{{ s: `{}` }}", self.s)
        }
        From<String> {
            |s| Self { s }
        }
        Deref<Target = String> {
            |&self| &self.s
        }
        DerefMut {
            |&mut self| &mut self.s
        }
    }
}

fn main() {
    let mut val_1 = MyStruct1::from("cat".to_string());
    // `&MyStruct2` coerces to `&String` which coerces to `&str`
    needs_ref_str(&val_1);
    // `&mut MyStruct2` coerces to `&mut String`
    needs_ref_mut_string(&mut val_1);
    println!("val_1: {}", val_1);
    assert_eq!(
        "{ s: `cat | 'needs_ref_mut_string' was here` }",
        val_1.to_string()
    );

    let mut val_2 = MyStruct2::from("cat".to_string());
    // `&MyStruct2` coerces to `&String` which coerces to `&str`
    needs_ref_str(&val_2);
    // `&mut MyStruct2` coerces to `&mut String`
    needs_ref_mut_string(&mut val_2);
    println!("val_2: {}", val_2);
    assert_eq!(
        "{ s: `cat | 'needs_ref_mut_string' was here` }",
        val_2.to_string()
    );

    println!();
    println!("done");
}

fn needs_ref_str(_: &str) {}
fn needs_ref_mut_string(s: &mut String) {
    s.push_str(" | 'needs_ref_mut_string' was here")
}
