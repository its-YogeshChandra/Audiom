use jsonwebtoken::{encode,  Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use db_services::get_user_data;

//read the secret from the env file 

//tldr of the jwt token : for understanding purpose only
//convert the user data into a token 
//and decode the values into the user credential 




//create the jwt token
pub struct Claims{
    id: Uuid,
    email: String,
    name: String,
}

fn create_token (){
    //get the values from the db
    let db_result = get_user_data();
    

    //
}



//verify the jwt token 