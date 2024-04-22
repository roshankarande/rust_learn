fn main() {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    pool.scope(|scope|{
        scope.spawn_broadcast(|_scope, broadcast_context|{
            println!("Hello from broadcast thread {}", broadcast_context.index()); 
        })
    });

    // pool.spawn(|| println!("Hello from pool thread"));

    // pool.scope(|scope| {
    //     for n in 0..20{
    //         scope.spawn(move |_| println!("Scoped spawing!! {n}"));
    //     }
    // });

    // println!("Hello from main thread");

}
