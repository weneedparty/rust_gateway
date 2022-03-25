use std::collections::HashMap;

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct PythonAccountStructure {
    pub service_address: Arc<Mutex<String>>,
}

impl PythonAccountStructure {
    pub async fn user_register_request(
        &self,
        email: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // This will POST a body of `{"lang":"rust","body":"json"}`
        let mut map = HashMap::new();
        map.insert("email", email);

        let client = reqwest::Client::new();
        let sub_url = "user_register_request/";
        let mut api_address = String::from("");
        match self.service_address.lock() {
            Ok(address) => api_address = address.clone().as_str().to_string() + sub_url,
            Err(_error) => {}
        };
        let response = client.post(api_address).json(&map).send().await?;

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
        &self,
        email: String,
        random_string: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // This will POST a body of `{"lang":"rust","body":"json"}`
        let mut map = HashMap::new();
        map.insert("email", email);
        map.insert("random_string", random_string);

        let client = reqwest::Client::new();
        let sub_url = "user_register_confirm/";
        let mut api_address = String::from("");
        match self.service_address.lock() {
            Ok(address) => api_address = address.clone().as_str().to_string() + sub_url,
            Err(_error) => {}
        };
        let response = client.post(api_address).json(&map).send().await?;

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

    pub async fn auth_jwt(&self, jwt: String) -> Result<String, Box<dyn std::error::Error>> {
        // This will POST a body of `{"lang":"rust","body":"json"}`
        let mut map = HashMap::new();
        map.insert("jwt", jwt);

        let client = reqwest::Client::new();
        let sub_url = "auth_jwt/";
        let mut api_address = String::from("");
        match self.service_address.lock() {
            Ok(address) => api_address = address.clone().as_str().to_string() + sub_url,
            Err(_error) => {}
        };
        let response = client.post(api_address).json(&map).send().await?;

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

    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     #[tokio::test]
    //     async fn test_user_register_request() {
    //         let mut ok = false;

    //         match user_register_request(String::from("hi.ss.com")).await {
    //             Ok(result) => {
    //                 println!("register request is sent: {:?}", result);
    //                 ok = true;
    //             }
    //             Err(err) => {
    //                 println!("Error on pre register request: {:?}", err);
    //             }
    //         }

    //         assert!(ok);
    //     }
    // }
}
