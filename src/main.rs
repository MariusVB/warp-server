#[macro_use]
extern crate diesel;
extern crate dotenv;

mod controllers { pub mod employee_controller; }


#[tokio::main]
async fn main() {

    warp::serve(controllers::employee_controller::create_employee_endpoints())
        .run(([127, 0, 0, 1], 3030))
        .await;
}