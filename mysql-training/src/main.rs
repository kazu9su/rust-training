#[macro_use]
extern crate mysql;

use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct Hoge {
    id: i32,
    text: Option<String>,
}

fn main() {
    let pool = my::Pool::new("mysql://root@localhost:3306").unwrap();

    let selected_hoges: Vec<Hoge> =
        pool.prep_exec("select * from hoge.hoge;", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, text) = my::from_row(row);
                Hoge {
                    id: id,
                    text: text,
                }
            }).collect()
        }).unwrap();

    let expected = vec![
        Hoge {id: 1, text: Some("hoge".into())},
        Hoge {id: 2, text: Some("fuga".into())},
    ];
    assert_eq!(expected, selected_hoges);
    println!("Yay!")
}
