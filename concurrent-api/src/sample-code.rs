use tokio;
use tokio::sync::Mutex as TMutex; // use if critical setion is long or contains await points
// use rayon for compute heavy tasks instead of tokio

let runtime = tokio::runtime::Runtime::new();
runtime.block_on(async {
    println!("Hello WOrld!");
    let mut network = read_from_network();
    let mut terminal = read_from_terminal();
    let mut f1 = tokio::fs::File::open("file1.txt");
    let mut f2 = tokio::fs::File::open("file2.txt");
    
    loop {
        select! {

        }
    }

async fn handle_connection(_: TcpStream) {
    let x = Arc::new(Mutex::new(vec![]));
    let x1 = Arc::clone(&x);
    let join_handle = tokio::spawn(async move {
        x1.lock();
        //
        0
    });
    assert_eq!(join_handle.await, 0);
    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        x2.loc();
        //
    });
}
})