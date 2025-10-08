fn main() {
    // plans the ultimate heist in preparation for GTA VI, picking the best
    // combination of items to put into a bag of capacity k, where each item has
    // some value v and some weight w
    let loot: Vec<(&str, f32, f32)> = [
        ("Art", 300.0, 11.0),
        ("Stamp", 300.0, 0.3),
        ("Diamond", 100.0, 0.6),
        ("Gold", 200.0, 10.0),
        ("Money", 200.0, 1.0),
    ].into();
    
    let mut memo: Vec<f32> = Vec::new();
    init_array(loot.len() as i64, &mut memo);
    let mut capacity: f32 = 5.0;
    dp_top_down((loot.len() as i64) - 1, &mut memo, &loot, &mut capacity);
    println!("Final: {:?}", memo);
}

fn dp_top_down(i: i64, memo: &mut Vec<f32>, loot: &Vec<(&str, f32, f32)>, capacity: &mut f32) -> f32{
    println!("Got {}, Memo {:?}", i, memo);

    if i == 1{
        if loot[0].2 <= capacity{
            memo[0] = loot[0].2;
            return memo[0];
        }
    }

    if memo[i] != -1{
        return memo[i]
    }

}

fn choose_max(a: f32, b: f32) -> f32{
    if a >= b{
        a
    } else {
        b
    }
}

fn init_array(n: i64, array: &mut Vec<i64>){
    for _ in 0..n{
        array.push(-1);
    }
}

