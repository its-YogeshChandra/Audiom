use jsonwebtoken::{encode,  Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};


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

    //
}



//verify the jwt token 