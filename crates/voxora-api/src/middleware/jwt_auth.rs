use voxora_core::{verify_token, Claims};
use actix_web::{HttpMessage, HttpRequest, dev::ServiceRequest};



pub fn jwt_middleware(request: ServiceRequest) -> Result<Claims, &'static str>{
    //take the jwt token from the headers of the request
    let headers = request.headers();

    match headers.get("Authorization") {
        Some(header) => {
            //too many unwraps (future me problem)
            let token = header.to_str().unwrap();
            let token = token.strip_prefix("Bearer ").unwrap();

            let claims = match verify_token(token) {
                Ok(claims) => claims,
                Err(e) => {
                    println!("{:?}", e);
                    return Err("token is invalid");
                }
            };
            return Ok(claims)
        }
        None => {
            println!("token not found");
            return Err("token not found");
        }
    };

}