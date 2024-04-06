use warp::Filter;

#[tokio::main]
async fn main() {
    println!("API booting.");

    let root = warp::path::end().map(|| "Welcome to the API?\n");

    let greet = warp::path!("greet" / String).map(|name| format!("Hi, {}!\n", name));

    let any = warp::any().map(|| "404 Not Found, but it's a 200 OK - Teehee!\n");

    let routes = root.or(
        warp::path("v0").and(greet.or(any))
    );

    println!("Preparing to serve routes.");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
