use uuid::Uuid;
//should match the db schema in the architecture.md


#[derive(Debug)]
pub struct UserMainDbResult {
    pub id            : Uuid, 
    pub email         : String,
    pub name          : String,
    pub avatar_url    : Option<String>,
    pub password_hash : Option<String>,
    pub provider      : String,
    pub created_at    : time::OffsetDateTime,
    pub updated_at    : time::OffsetDateTime
}

pub struct NewUser{
   pub email: String,
   pub name: String,
   pub password_hash: String,
}

//can we do something like only one of them can we filled when called in another part in the system 

pub enum UpdateUser{
    Email(String),
    Name(String),
    PasswordHash(String),
}


pub enum GetUser{
    Email(String),
    Id(Uuid),
}

//not using the impl 
//going with simple functions system 

pub async fn create_user(user:NewUser, pool:&sqlx::PgPool) -> Result<UserMainDbResult,sqlx::Error>{
    //call the create function using sqlx 
    let result = sqlx::query_as!(UserMainDbResult,"INSERT INTO users (email , name , password_hash) VALUES ($1 , $2 , $3) RETURNING *",
    user.email,
    user.name,
    user.password_hash
)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn update_user_data(user_id:Uuid, user_data:UpdateUser,pool:&sqlx::PgPool) -> Result<UserMainDbResult,sqlx::Error>{
//check which field is present 
match user_data {
    UpdateUser::Email(email) => {
           let result = sqlx::query_as!(UserMainDbResult,"UPDATE users SET email = $1  WHERE id = $2 RETURNING *",
           email,
           user_id
           )
           .fetch_one(pool)
           .await?;
        
        return Ok(result)
           }
    UpdateUser::Name(name) => {
        let result = sqlx::query_as!(UserMainDbResult,"UPDATE users SET name = $1  WHERE id = $2 RETURNING *",
           name,
           user_id
           )
           .fetch_one(pool)
           .await?;
        
        return Ok(result)
    }
    UpdateUser::PasswordHash(password_hash) => {
           let result = sqlx::query_as!(UserMainDbResult,"UPDATE users SET password_hash = $1  WHERE id = $2 RETURNING *",
           password_hash,
           user_id
           )
           .fetch_one(pool)
           .await?;
        
        return Ok(result)
    }
}   

}

pub async fn get_user_data(pool:&sqlx::PgPool, user_data:GetUser )-> Result<UserMainDbResult,sqlx::Error>{
    //fetch the result on the basis of the user_data enum 
    match user_data {
        GetUser::Email(email) => {
            let result = sqlx::query_as!(UserMainDbResult,"SELECT * FROM users WHERE email = $1",email).fetch_one(pool).await?;
            return Ok(result)
        }
        GetUser::Id(id) => {
            let result = sqlx::query_as!(UserMainDbResult,"SELECT * FROM users WHERE id = $1",id).fetch_one(pool).await?;
            return Ok(result)
        }
    }
    
}
    
