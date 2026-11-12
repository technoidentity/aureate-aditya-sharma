fn main() {
    println!("Subject: To learn about vectors in Rust.");
    let mut nums: Vec<i32> = Vec::new();
    // Usually rust infers type once you insert something.
    nums.push(10);
    // Using vec!
    let mut nums = vec![10, 20, 30, 40];
    // Repeated value
    let mut nums = vec![0; 5];
    /*
     * The critical difference between array and vector is that the arrays have a fixed size encoded in its type while vector can grow and shrink.
     */
    let mut nums = vec![10, 20, 30, 40, 50];
    println!("{}", nums[0]);
    println!("{}", nums[1]);
    // if you use out of index the program panics.
    // Safer access uses get()
    let mut value = nums.get(3); // The type is Option<&i32>
    match value {
        Some(value) => println!("{}", value),
        None => println!("Invalid index."),
    }
    // Updating elements
    nums[0] = 7;
    println!("{}", nums[0]);

    let mut nums = vec![10, 20];
    nums.push(30);
    nums.push(4000);
    let value = nums.pop();
    println!("{}", value.unwrap_or(0));

    nums.insert(1, 10000);
    println!("{nums:?}");

    let removed = nums.remove(1);

    println!("{removed}");

    let removed = nums.swap_remove(1);
    println!("{removed}");
    println!("{}", nums.len());
    if nums.is_empty() {
        println!("It is empty!");
    } else {
        println!("It is not empty.");
    }

    for num in &mut nums {
        println!("hehe/: {}", *num * 2);
    }

    let mut nums = vec![10, 20, 30];
    for (i, value) in nums.iter().enumerate() {
        println!("index={i}, value={value}");
    }
    nums.extend([4, 5, 6]);
    println!("{nums:?}");
    nums.clear();
    let mut a = vec![1, 2, 3];
    let mut b = vec![4, 5, 6];
    nums.append(&mut a);
    nums.append(&mut b);
    println!("nums: {nums:?}");
    println!("a: {a:?}");
    println!("b: {b:?}");

    let mut nums = Vec::new();
    nums.push(10);
    nums.push(20);
    nums.push(30);
    println!("Length: {}", nums.len());
    println!("Capacity: {}", nums.capacity());

    if nums.contains(&20) {
        println!("Found");
    }

    let found = nums.iter().find(|&&x| x == 20);
    println!("{found:?}");
    let index = nums.iter().position(|&x| x == 20);
    println!("{index:?}");
    nums.retain(|x| x % 2 == 0 && x % 20 == 0);
    println!("{nums:?}");
    nums.clear();
    nums.extend([1, 2, 3, 8, 5, 6, 9, 4, 7]);
    nums.sort();
    nums.reverse();
    println!("{nums:?}");
    nums.sort_by(|a, b| a.cmp(b));
    println!("{nums:?}");

    // Sort by key function for the types that have keys.
    nums.extend([1, 2, 4, 5]);
    println!("{nums:?}");
    nums.sort();
    nums.dedup();
    println!("{nums:?}");

    // Slicing a vector : A vector can be borrowed as a slice
    let nums = vec![10, 20, 30, 40, 50];
    let slice = &nums[1..4];
    println!("{slice:?}");
    println!("{nums:?}");

    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("{doubled:?}");
    let sum: i32 = nums.iter().sum();
    println!("{sum:?}"); //similar: min max
}
