fn main() {
    // plans the ultimate heist in preparation for GTA VI, picking the best
    // combination of items to put into a bag of capacity k, where each item has
    // some value v and some weight w

    let loot: Vec<(&str, f32, f32)> = [
        ("Art", 3.0, 2.0),
        ("Stamp", 4.0, 3.0),
        ("Diamond", 5.0, 4.0),
        ("Gold", 8.0, 5.0),
        ("Money", 10.0, 9.0),
    ].into();
    let loot: Vec<(&str, f32, f32)> = [
        ("Art", 60.0, 10.0),
        ("Stamp", 100.0, 20.0),
        ("Diamond", 120.0, 30.0),

    ].into();

    let capacity: f32 = 50.0;
    // dp_top_down((loot.len() as i64) - 1, &mut memo, &loot, &mut capacity);
    // bottom_up(capacity, &mut memo, &loot);
    // println!("Final: {:?}", memo);
    println!("{:?}", top_down(0, capacity, &loot));
}

fn top_down(
    i: u32,
    capacity: f32,
    loot: &Vec<(&str, f32, f32)>
) -> (f32, f32) {
    if i == (loot.len() as u32) || capacity <= 0.0 {
        return (0.0, 0.0);
    }

    let (value_i, weight_i) = (loot[i as usize].1, loot[i as usize].2);

    let excluded = top_down(i + 1, capacity, loot);

    let included = {
        if can_we_include(loot[i as usize], capacity){
            let (sub_v, sub_w) = top_down(i + 1, capacity - weight_i, loot);
            (value_i + sub_v, weight_i + sub_w)
        } else {
            (0.0, 0.0)
        }
    };

    return choose_max(included, excluded);
}

fn can_we_include(item: (&str, f32, f32), capacity: f32) -> bool{
    item.2 <= capacity
}

fn choose_max(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
    if a.0 >= b.0 { a } else { b }
}

fn init_array(n: i64, array: &mut Vec<(f32, f32)>) {
    for _ in 0..=n + 1 {
        array.push((-1.0, -1.0));
    }
}
