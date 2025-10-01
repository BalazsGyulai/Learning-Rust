fn main() {
    // plans the ultimate heist in preparation for GTA VI, picking the best
    // combination of items to put into a bag of capacity k, where each item has
    // some value v and some weight w
    let mut loot: Vec<(&str, f32, f32)> = [
        ("Art", 300.0, 11.0),
        ("Stamp", 300.0, 0.3),
        ("Diamond", 100.0, 0.6),
        ("Gold", 200.0, 10.0),
        ("Money", 200.0, 1.0),
    ].into();
    let picked: Vec<(String, f32, f32)> = heist(&mut loot, 2.5);
    println!("{:?}", picked);
}

fn heist(loot: &mut Vec<(&str, f32, f32)>, capacity: f32) -> Vec<(String, f32, f32)> {
    let mut pick: Vec<(String, f32, f32)> = vec![];
    let mut bag_limit: f32 = 0.0;

    sort_loot_by_value(loot); // now the valuable items are in the front
    sort_loot_by_space(loot); // now the valuable but smaller items are in the front

    // we pick those items which are valuable and small
    for i in 0..loot.len() {
        if loot[i].2 <= capacity - bag_limit { // only need to check the space because it will automatically pick the ones that are valuable
            pick.push((loot[i].0.to_string(), loot[i].1, loot[i].2));
            bag_limit += loot[i].2; // store how many items are in the bag
        }
    }

    pick
}

fn sort_loot_by_value(loot: &mut Vec<(&str, f32, f32)>) {
    // sorts the loot by how valuable it is

    for _ in 0..loot.len() {
        for y in 0..loot.len() - 1 {
            if loot[y].1 < loot[y + 1].1 {
                let temp = loot[y];
                loot[y] = loot[y + 1];
                loot[y + 1] = temp;
            }
        }
    }
}

fn sort_loot_by_space(loot: &mut Vec<(&str, f32, f32)>) {
    // sorts the loot by its space, but does not change the order in how valuable an item is

    for _ in 0..loot.len() {
        for y in 0..loot.len() - 1 {
            if loot[y].1 == loot[y + 1].1 && loot[y].2 > loot[y + 1].2 {
                let temp = loot[y];
                loot[y] = loot[y + 1];
                loot[y + 1] = temp;
            }
        }
    }
}
