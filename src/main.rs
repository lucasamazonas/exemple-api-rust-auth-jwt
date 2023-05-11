use jwt_simple::prelude::*;


fn main() {
    let key = HS256Key::generate();

    let claims = Claims::create(Duration::from_hours(2));
    let token = key.authenticate(claims).unwrap();
    let token_is_valid = key.verify_token::<NoCustomClaims>(&token, None);

    println!("{token}");

    match token_is_valid {
        Ok(_) => println!("Token válido"),
        _ => println!("token inválido!")
    }
}
