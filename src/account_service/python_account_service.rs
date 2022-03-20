use std::collections::HashMap;

const PYTHON_ACCOUNT_SERVICE_ADDRESS: &str = "http://127.0.0.1:40052/";

pub async fn user_register_request(email: String) -> Result<bool, Box<dyn std::error::Error>> {
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("email", email);

    let client = reqwest::Client::new();
    let response = client
        .post(PYTHON_ACCOUNT_SERVICE_ADDRESS.to_string() + "user_register_request/")
        .json(&map)
        .send()
        .await?;

    if response.status().is_success() {
        let json_response = response.json::<serde_json::Value>().await?;
        let the_error = &json_response["error"];
        if the_error != &serde_json::Value::Null {
            return Ok(false);
        }
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub async fn user_register_confirm(
    email: String,
    random_string: String,
) -> Result<String, Box<dyn std::error::Error>> {
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

    let json_response = response.json::<serde_json::Value>().await?;

    let the_error = &json_response["error"];
    if the_error == &serde_json::Value::Null {
        let jwt = &json_response["result"]["jwt"];
        return Ok(jwt.as_str().unwrap().to_string());
    } else {
        return Err(json_response["error"].to_string().into());
    }
    // return Err("Unknow error, can't get jwt from /user_register_confirm".into());
}

pub async fn auth_jwt(jwt: String) -> Result<String, Box<dyn std::error::Error>> {
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("jwt", jwt);

    let client = reqwest::Client::new();
    let response = client
        .post(PYTHON_ACCOUNT_SERVICE_ADDRESS.to_string() + "auth_jwt/")
        .json(&map)
        .send()
        .await?;

    let json_response = response.json::<serde_json::Value>().await?;

    let the_error = &json_response["error"];
    if the_error == &serde_json::Value::Null {
        let email = &json_response["email"];
        return Ok(email.as_str().unwrap().to_string());
    } else {
        return Err(json_response["error"].to_string().into());
    }
    // return Err("Unknow error, can't get email from /auth_jwt".into());
}

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
                println!("Error on pre register request: {:?}", err);
            }
        }

        assert!(ok);
    }

    // #[tokio::test]
    // async fn test_user_register_request_confirm() {
    //     let mut ok = false;

    //     match user_register_confirm(String::from("hi.ss.com"), String::from("hh")).await {
    //         Ok(result) => {
    //             println!("register request confirm is sent: {:?}", result);
    //             ok = true;
    //         }
    //         Err(err) => {
    //             println!("Error on register request confirm: {:?}", err);
    //         }
    //     }

    //     assert!(ok);
    // }
}
