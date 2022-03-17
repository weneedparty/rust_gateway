use std::collections::HashMap;

const PYTHON_ACCOUNT_SERVICE_ADDRESS: &str = "http://127.0.0.1:40053/";

async fn user_register_request(email: String) -> Result<bool, Box<dyn std::error::Error>> {
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("email", email);

    let client = reqwest::Client::new();
    let response = client
        .post(PYTHON_ACCOUNT_SERVICE_ADDRESS.to_string() + "user_register_request/")
        .json(&map)
        .send()
        .await?;

    if !response.status().is_success() {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

async fn user_register_confirm(
    email: String,
    random_string: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("email", email);
    map.insert("random_string", random_string);

    let client = reqwest::Client::new();
    let response = client
        .post(PYTHON_ACCOUNT_SERVICE_ADDRESS.to_string() + "user_register_confirm/")
        .json(&map)
        .send()
        .await?;

    if !response.status().is_success() {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

use tokio_test;

#[cfg(test)]
mod tests {
    use super::*;

    // async fn setup() {
    //     tokio_test::block_on(async {});
    // }

    #[tokio::test]
    async fn test_user_register_request() {
        let mut ok = false;

        match user_register_request(String::from("hi.ss.com")).await {
            Ok(result) => {
                println!("register request is sent: {:?}", result);
                ok = true;
            }
            Err(err) => {
                println!("Error opening file: {:?}", err);
            }
        }

        assert!(ok);
    }
}

/*
@ app.post("/user_register_confirm/", response_model=models.UserRegisterConfirmOutput)
async def user_register_confirm(input: models.UserRegisterConfirmInput):
    email = input.email
    randomString = input.random_string

    matched = await myAuthClass.check_if_any_info_matchs_in_unverified_pool(email=email, random_string=randomString)
    if (not matched):
        return models.UserRegisterConfirmOutput.parse_obj({"result": None, "error": "No matched info found."})
    await myAuthClass.add_info_to_verified_pool(email=email, random_string=randomString)
    jwt_string = await myAuthClass.get_auth_jwt_string(email=email, random_string=randomString)
    return models.UserRegisterConfirmOutput.parse_obj(
    {
        "result": {
            "jwt": jwt_string,
        },
        "error": None
    })


@ app.post("/get_data/", response_model=models.GetDataOutput)
async def get_data(input: models.GetDataInput):
    user = await myAuthClass.auth_jwt_string(raw_jwt_string=input.jwt)

    if (user is None):
        return models.GetDataOutput.parse_obj({"result": None, "error": "Invalid JWT."})
    return models.GetDataOutput.parse_obj({"result": "Hello, " + user.email + "!", "error": None})

*/
