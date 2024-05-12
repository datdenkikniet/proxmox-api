use crate::{de::from_str, ser::to_string};

#[test]
pub fn test() {}

#[test]
fn test_struct() {
    use serde_derive::Serialize;
    #[derive(Serialize)]
    struct T {
        pub field1: String,
        pub filed2: String,
    }
    let t = T {
        field1: String::from("1"),
        filed2: String::from("2"),
    };

    let re = to_string(&t).unwrap();
    println!("re: {}", re);
}

#[test]
fn test_struct_inner() {
    use serde_derive::Serialize;
    #[derive(Serialize)]
    struct Inner {
        pub if1: String,
        pub if2: String,
    }
    #[derive(Serialize)]
    struct T {
        pub field1: String,
        pub filed2: String,
        #[serde(flatten)]
        pub inner: Inner,
    }

    let t = T {
        field1: String::from("1"),
        filed2: String::from("2"),
        inner: Inner {
            if1: String::from("i1"),
            if2: String::from("i2"),
        },
    };

    let re = to_string(&t).unwrap();
    println!("re: {}", re);
}

#[test]
fn test_struct_inners() {
    use serde_derive::Serialize;
    use std::collections::BTreeMap;
    #[derive(Serialize)]
    struct Inner {
        pub if1: String,
        pub if2: String,
    }
    #[derive(Serialize)]
    struct T {
        pub field1: String,
        pub filed2: String,
        #[serde(flatten)]
        pub inners: BTreeMap<String, Inner>,
    }

    let mut inners = BTreeMap::new();
    inners.insert(
        "list1".into(),
        Inner {
            if1: String::from("i1"),
            if2: String::from("i2"),
        },
    );
    let t = T {
        field1: String::from("1"),
        filed2: String::from("2"),
        inners,
    };

    let re = to_string(&t).unwrap();
    println!("re: {}", re);
}

use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Test1 {
    pub status: bool,
    pub key1: String,
    pub key2: String,
    pub sub1: SubTest1,
    pub seq: Vec<String>,
    #[serde(flatten)]
    pub map: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubTest1 {
    pub key1: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum EnumTest {
    I,
    II,
}
#[test]
fn test_serialise() {
    let mut t1 = Test1 {
        status: true,
        key1: "value1".into(),
        key2: "value2".into(),
        sub1: SubTest1 {
            key1: "subvalue1".into(),
        },
        seq: Vec::from(["1".into(), "2".into()]),
        map: BTreeMap::new(),
    };
    t1.map.insert("flattenmap1".into(), "mvalue1".into());
    let s = to_string(&t1);
    println!("t1: {}", s.unwrap())
}

#[test]
fn test_deserialize() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub status: bool,
        pub key1: i16,
        pub key2: String,
        pub sub1: SubTest1,
    }

    let s = r#"status=1&key1=13232&key2=value2&sub1=key1%3Dsubvalue134%3A32"#;
    let s: Test1 = from_str(s).unwrap();
    println!("{:?}", s);
}

#[test]
fn test_deserialize_enum() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub status: bool,
        pub key1: i16,
        pub key2: String,
        pub sub1: SubTest1,
        pub e: EnumTest,
        pub arr: Vec<i32>,
    }

    let s = r#"status=1&key1=13232&key2=value2&sub1=key1%3Dsubvalue134%3A32&e=I&arr=1%3B2"#;
    let s: Test1 = from_str(s).unwrap();
    println!("{:?}", s);
}

#[test]
fn test_deserialize_extra() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub status: bool,
        pub key1: i16,
        pub key2: String,
        pub sub1: SubTest1,
        pub e: EnumTest,
        pub arr: Vec<i32>,
    }

    let s = r#"status=1&key1=13232&key2=value2&sub1=key1%3Dsubvalue134%3A32&e=I&arr=1%3B2&WTF=1"#;
    let s: Test1 = from_str(s).unwrap();
    println!("{:?}", s);
}

#[test]
fn test_deserialize_option_none() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub status: bool,
        pub key1: i16,
        pub key2: String,
        pub sub1: SubTest1,
        pub e: EnumTest,
        pub arr: Vec<i32>,
        pub option: Option<i32>
    }

    let s = r#"status=1&key1=13232&key2=value2&sub1=key1%3Dsubvalue134%3A32&e=I&arr=1%3B2&WTF=1"#;
    let s: Test1 = from_str(s).unwrap();
    println!("{:?}", s);
}

#[test]
fn test_deserialize_option_some() {
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Test1 {
        pub status: bool,
        pub key1: i16,
        pub key2: String,
        pub sub1: SubTest1,
        pub e: EnumTest,
        pub arr: Vec<i32>,
        pub option: Option<i32>
    }

    let s = r#"status=1&key1=13232&key2=value2&sub1=key1%3Dsubvalue134%3A32&e=I&arr=1%3B2&WTF=1&option=1"#;
    let s: Test1 = from_str(s).unwrap();
    println!("{:?}", s);
}