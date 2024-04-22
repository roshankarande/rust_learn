fn double(n: i32) -> i32 {
    n*2
}

fn main() {
    let mut n = 32;
    n += 1;
    println!("{n}");
    
    println!("Hello, world!!!!");
    
    let arr: [i32; 10] = [0; 10];
    // let arr = vec![1,2,3,4];

    for a in arr{
        println!("{a}");
    }

    // for a in arr{
    //     println!("{a}");
    // }

    println!("{:?}", double(n))

}
