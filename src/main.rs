use rocket::serde::json::{json, Value};
use rocket::http::{Header,Status};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::data::{Data,ToByteUnit};


mod rgenerator;
mod slider;
mod encode;

#[macro_use] extern crate rocket;

const PUZZLE_EPOCH: u32 = 0;
const SHUFFLE_COUNT: u32 = 20;
// necessary to prevent a bad actor from sending a huge solution to the server
const MAX_SOLUTION_SIZE_BYTES:u32 = 1000;


#[post("/generate/<address>")]
fn generate(address: String) -> Value
{
    let epoch = PUZZLE_EPOCH as usize;
    // construct prng using the address as the seed
    let mut rng = rgenerator::construct( &address, PUZZLE_EPOCH );
    // shuffle the daily target board 20 times using the rng
    let shuffled_board = slider::generate_puzzle(   epoch , 
                                                    SHUFFLE_COUNT,
                                                    &mut rng );

    return json!({
        "index": epoch,
        "target_board": slider::fetch_target( epoch ),
        "shuffled_board": shuffled_board
    });
}



#[post("/verify/<address>", data="<solution>")]
async fn verify(address: String, solution: Data<'_>) -> (Status, &'static str) {
    
    let bytes = solution.open( MAX_SOLUTION_SIZE_BYTES.bytes() ).into_bytes().await;

    match bytes {
        Ok(v) => {

            if !v.is_complete() {
                return (Status::BadRequest, "Solution max length exceeded");
            }

            let epoch = PUZZLE_EPOCH as usize;

            /* Shuffled board is not stored in database as it is computationally inexpensive
               to recompute and result is deterministic */
            let mut rng = rgenerator::construct( &address, PUZZLE_EPOCH );
            let mut shuffled_board = slider::generate_puzzle(   epoch ,
                                                                SHUFFLE_COUNT,
                                                                &mut rng );
            
            if slider::verify_puzzle( epoch , &mut shuffled_board, &v.into_inner() ) {
                let signature = encode::get_signature( &address ,epoch);
                return (Status::Accepted, signature );
            } else {
                return (Status::Accepted, "Invalid solution");
            }            
        }
        Err(e) => {
            return (Status::BadRequest, "Error reading solution");
        }
    }
}

/* Cross Origin Resource Sharing handling.
   For development build, permit all requests
   In production build allow puzzlr domain origin only */
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }
    // allow all
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/* Start web server */
#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(CORS)
    .mount("/", routes![generate])
    .mount("/", routes![verify])
}
