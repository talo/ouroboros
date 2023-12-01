use ouroboros::Ouroboros;

#[test]
fn tuple_to_python() {
    assert_eq!("tuple[bool]", <(bool,)>::python());
    assert_eq!("tuple[bool, int]", <(bool, u32)>::python());
    assert_eq!("tuple[bool, int, int]", <(bool, u32, i32)>::python());
    assert_eq!(
        "tuple[bool, int, int, float]",
        <(bool, u32, i32, f32)>::python()
    );
    assert_eq!(
        "tuple[bool, int, int, float, str]",
        <(bool, u32, i32, f32, String)>::python()
    );
    assert_eq!(
        "tuple[bool, int, int, float, tuple[bool, int, int, float, str]]",
        <(bool, u32, i32, f32, (bool, u32, i32, f32, String))>::python()
    );
}

#[test]
fn tuple_to_cpp() {
    assert_eq!("std::tuple<bool>", <(bool,)>::cpp());
    assert_eq!("std::tuple<bool, uint32_t>", <(bool, u32)>::cpp());
    assert_eq!(
        "std::tuple<bool, uint32_t, int32_t>",
        <(bool, u32, i32)>::cpp()
    );
    assert_eq!(
        "std::tuple<bool, uint32_t, int32_t, float>",
        <(bool, u32, i32, f32)>::cpp()
    );
    assert_eq!(
        "std::tuple<bool, uint32_t, int32_t, float, std::string>",
        <(bool, u32, i32, f32, String)>::cpp()
    );
    assert_eq!(
        "std::tuple<bool, uint32_t, int32_t, float, std::tuple<bool, uint32_t, int32_t, float, std::string>>",
        <(bool, u32, i32, f32, (bool, u32, i32, f32, String))>::cpp()
    );
}

#[test]
fn struct_to_python() {
    #[allow(unused)]
    #[derive(Ouroboros)]
    struct Foo {
        a: bool,
        b: i32,
        c: f32,
        d: String,
        e: Option<bool>,
        f: Vec<i64>,
        g: Option<Vec<f64>>,
        h: Vec<Option<String>>,
        i: (bool, u32, f32, String),
        j: Option<(bool, u64, Option<f64>, Vec<String>)>,
        k: (bool, u32, f32, (bool, u32, f32, String)),
    }

    assert_eq!(
        r#"class Foo:

    a: bool
    b: int
    c: float
    d: str
    e: bool | None
    f: list[int]
    g: list[float] | None
    h: list[str | None]
    i: tuple[bool, int, float, str]
    j: tuple[bool, int, float | None, list[str]] | None
    k: tuple[bool, int, float, tuple[bool, int, float, str]]

   def __init__(self, a: bool, b: int, c: float, d: str, e: bool | None, f: list[int], g: list[float] | None, h: list[str | None], i: tuple[bool, int, float, str], j: tuple[bool, int, float | None, list[str]] | None, k: tuple[bool, int, float, tuple[bool, int, float, str]]):
        self.a = a
        self.b = b
        self.c = c
        self.d = d
        self.e = e
        self.f = f
        self.g = g
        self.h = h
        self.i = i
        self.j = j
        self.k = k"#,
        Foo::python()
    );
}

#[test]
fn struct_to_cpp() {
    #[allow(unused)]
    #[derive(Ouroboros)]
    struct Foo {
        a: bool,
        b: i32,
        c: f32,
        d: String,
        e: Option<bool>,
        f: Vec<i64>,
        g: Option<Vec<f64>>,
        h: Vec<Option<String>>,
        i: (bool, u32, f32, String),
        j: Option<(bool, u64, Option<f64>, Vec<String>)>,
        k: (bool, u32, f32, (bool, u32, f32, String)),
    }

    assert_eq!(
        r#"struct Foo {
  bool a;
  int32_t b;
  float c;
  std::string d;
  std::optional<bool> e;
  std::vector<int64_t> f;
  std::optional<std::vector<double>> g;
  std::vector<std::optional<std::string>> h;
  std::tuple<bool, uint32_t, float, std::string> i;
  std::optional<std::tuple<bool, uint64_t, std::optional<double>, std::vector<std::string>>> j;
  std::tuple<bool, uint32_t, float, std::tuple<bool, uint32_t, float, std::string>> k;
};"#,
        Foo::cpp()
    );
}

#[test]
fn enum_to_cpp() {
    #[allow(unused)]
    #[derive(Ouroboros)]
    enum Foo {
        A,
        B,
        C,
    }

    println!("Foo::cpp(): {}", Foo::cpp());
    println!("Foo::python(): {}", Foo::python());
}
