fn main() {
    // finds the minimum and the sum of a collection of numbers
    let array: Vec<i8> = [9, 6, 2, 8, 3, 10].into();
    let result = analyze(&array);
    println!("{:?}", result);

    // prints the body mass index category (2nd exercise) using pattern matching
    let bmi: String = body_mass(70.0, 1.75);
    println!("{}", bmi);

    // returns the longest prefix of a collection having numbers smaller than n.
    let array: Vec<i32> = [1, 2, 3, 4, 5, 4, 3, 2, 1].into();
    let prefix = prefix(4, &array);
    println!("{:?}", prefix);

    // returns all prime numbers up to a given number n
    let primes: Vec<i32> = prime(50);
    println!("{:?}", primes);

    // sorts a collection of people with names and ages by name
    let mut people: Vec<(&str, i32)> = [
        ("Gummy Bear", 10),
        ("Michael", 25),
        ("Almdudler", 40),
        ("Zeus", 20),
    ].into();
    sort_by_name(&mut people);
    println!("{:?}", people);

    // prints a collection of heroes with names and health points in a nice way by
    // visualizing them below each other with their health points as health bars.
    let heroes: Vec<(&str, i32)> = [
        ("Warrior", 10),
        ("Barbar", 6),
        ("Queen", 2),
        ("King", 15),
    ].into();
    heroes_print(&heroes);
}

// --------------------------------------
// finds the minimum and the sum of a collection of numbers
// --------------------------------------
fn analyze(array: &Vec<i8>) -> (i8, i8) {
    let mut min = array[0];
    let mut sum = array[0];

    for i in 1..array.len() {
        if array[i] < min {
            min = array[i];
        }

        sum += array[i];
    }

    (min, sum)
}

// --------------------------------------
// prints the body mass index category (2nd exercise) using pattern matching
// --------------------------------------
fn body_mass(weight: f32, height: f32) -> String {
    let bmi: f32 = weight / (height * height);

    match bmi {
        number @ bmi if bmi < 18.5 => format!("Underweight with BMI {number}"),
        number @ bmi if bmi >= 18.5 && bmi < 24.9 => format!("Normal weight with BMI {number}"),
        number @ bmi if bmi >= 24.9 && bmi < 29.9 => format!("Overweight weight with BMI {number}"),
        number @ _ => format!("Obese with BMI {number}"),
    }
}

// --------------------------------------
// returns the longest prefix of a collection having numbers smaller than n.
// --------------------------------------
fn prefix(n: i32, array: &Vec<i32>) -> &[i32] {
    let mut i = 0;

    while array[i] < n && i < array.len() {
        i += 1;
    }

    &array[..i]
}
// --------------------------------------
// returns all prime numbers up to a given number n
// --------------------------------------
fn prime(n: i32) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    for i in 2..=n {
        let mut dividable = 0;
        for y in 1..=i {
            if i % y == 0 {
                dividable += 1;
            }
        }

        if dividable == 2 {
            numbers.push(i);
        }
    }

    numbers
}

// --------------------------------------
// sorts a collection of people with names and ages by name
// --------------------------------------
fn sort_by_name(peoples: &mut Vec<(&str, i32)>) {
    for _ in 0..peoples.len() {
        for y in 0..peoples.len() - 1 {
            if peoples[y].0 > peoples[y + 1].0 {
                let temp = peoples[y];
                peoples[y] = peoples[y + 1];
                peoples[y + 1] = temp;
            }
        }
    }
}

// --------------------------------------
// prints a collection of heroes with names and health points in a nice way by
// visualizing them below each other with their health points as health bars.
// --------------------------------------
fn heroes_print(heroes: &Vec<(&str, i32)>) {
    let mut max_health = heroes[0].1;
    let mut hero_name_lenght = heroes[0].0.len();

    for hero in heroes {
        if hero.1 > max_health {
            max_health = hero.1;
        }

        if hero.0.len() > hero_name_lenght {
            hero_name_lenght = hero.0.len();
        }
    }

    for hero in heroes {
        let bar: String = create_bar(&hero.1);

        println!(
            "{hero_name: >name_lenght$} {bar}",
            hero_name = hero.0,
            name_lenght = hero_name_lenght + 1
        );
    }
}

fn create_bar(bar_len: &i32) -> String {
    let mut bar: String = "".to_string();

    for _ in 0..*bar_len {
        bar.push_str("#");
    }

    bar
}
