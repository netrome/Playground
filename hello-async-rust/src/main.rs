async fn learn_song(){
    println!("Learning song");
}

async fn sing(){
    println!("Singing");
}

async fn dance(){
    println!("Dancing!");
}

async fn learn_and_sing(){
    learn_song().await;
    sing().await;
}

async fn async_main(){
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f2, f1);
}

fn main() {
    println!("Hello, world!");
    futures::executor::block_on(async_main());
}
