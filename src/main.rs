use std::{env, fs, path::PathBuf};

use crate::display_markdown::DisplayMarkdown;
use parse_boox::parse_boox;

mod display_markdown;
mod model;
mod nom_util;
mod parse_boox;

// Take a path to a Boox file and print it as Markdown
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: highlight-extract <path>");
        return;
    }
    let file = PathBuf::from(&args[1]);
    let data = fs::read_to_string(file).unwrap();

    let boox = parse_boox(&data).unwrap().1;

    println!("{}", DisplayMarkdown(&boox));
}

#[test]
fn boox_test() {
    let data = include_str!("../data/russian_1.txt");

    let boox = parse_boox(data);
    let md = boox.map(|b| DisplayMarkdown(&b.1).to_string());

    assert_eq!(md, Ok("# Чистая архитектура

**Author:** Роберт Сесил Мартин

---

## Highlights

#### Highlight (Page 4, 2023-03-31 03:23:00)

> Посвящается моей любимой супруге, моим четырем замечательным детям и их семьям

### Предисловие

#### Highlight (Page 4, 2023-03-31 03:24:00)

> включая пятерых внуков — радость моей жиз

#### Highlight (Page 5, 2023-03-31 03:38:00)

> Очевидная привлекательность архитектуры — это структура. А структура — это то, что доминирует над парадигмами и суждениями в мире разработки программного обеспечения — компонентами, классами, функциями, модулями, слоями

#### Highlight (Page 5, 2023-03-31 03:38:00)

> невероятные небоскребы-башни Дженга, достигающие облаков, археологические слои, залегающие в горной породе. Структура программного обеспечения не всегда интуитивно очевидна, как структура зданий.
> Здания имеют оче

#### Highlight (Page 5, 2023-03-31 03:39:00)

> назначения и от наличия или отсутствия архитектурных украшений

ga

### Причины неприятностей

#### Highlight (Page 23, 2023-03-31 09:47:00)

> Глава\u{a0}1. Что такое дизайн и архитектура?
> Цель?
> В чем состоит цель таких решений, цель хорошего дизайна программного обеспечения? Главная цель — не что иное, как мое

#### Highlight (Page 23, 2023-03-31 09:47:00)

> Мерой качества дизайна может служить простая мера трудозатрат, необходимых для удовлетворения потребностей клиента. Если трудозатраты невелики и остаются небольшими в течение эксплуатации системы, система имеет хо

".to_owned()));
}
