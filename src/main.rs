use std::{env, fs, path::PathBuf};

use crate::display_markdown::DisplayMarkdown;
use parse_boox::parse_boox;

mod display_markdown;
mod model;
mod nom_util;
mod parse_boox;
mod parse_boox_v1;
mod parse_boox_v2;

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
fn boox_test_v2() {
    let data = include_str!("../test/data/russian_1.txt");

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

#[test]
fn boox_test_v1() {
    let data = include_str!("../test/data/v1.txt");

    let boox = parse_boox(data);
    let md = boox.map(|b| DisplayMarkdown(&b.1).to_string());

    assert_eq!(md, Ok("# One Up on Wall Street - Peter Lynch & John Rothchild (952)

**Author:** Peter Lynch; John Rothchild

---

## Highlights

#### Highlight (Page 13, 2022-03-07 01:11:00)

> tics to a degree neither side could have imagined in the doldrums of the early 1970s, when I first took the helm at Magellan. At that low point, demoralized investors had to remind themselves that bear markets don’t last forever, and those with patience held on to their stocks and mutual funds for the fifteen years it took the Dow and other averages to regain the prices reached in the mid-1960s. Today it’s worth reminding ourselves that bull markets don’t last forever and that patience is required in both directions.On  of this book I say the breakup of ATT

some very good annotation

#### Highlight (Page 20, 2022-03-07 14:02:00)

> valued at $10 billion may not be worth a dime. As expectations turn to reality, the winners will be more obvious than they are today. Investors who see this will have time to act on their “edge.”

#### Highlight (Page 20, 2022-03-07 14:02:00)

> Microsoft went public in 1986 at 15 cents a share. Three years later you could buy a share for under $1, and from there it advanced eightyfold. (The stock has “split” several times along the way, so original shares never actually sold for 15 cents—for further explanation, see the footnote on .) If you took the Missouri “show me” approach and waited to buy Microsoft until it triumphed with Windows 95, you still made seven times your money. You didn’t have to be a programmer to notice Microsoft everywhere you looked. Except in the Apple orchard, all new computers

#### Highlight (Page 22, 2022-03-07 01:20:00)

> Street Journal and Barron’s, and get a snapshot review of almost any publicly traded company. From there you can access “Zack’s” and get a summary of ratings from all the analysts who follow a particular stock.Again thanks to the Internet, the cost of buying and selling stocks has been drastically reduced for the small investor, the way it was reduced for institutional investors in 1975. On-line trading has pressured traditional brokerage houses to reduce commissions and transaction fees, continuing a trend that began with the birth of the discount broker two decades ago.You may be wondering what’s happened to my investing habits since I left Magellan. Instead of following thousands

".to_owned()));
}
