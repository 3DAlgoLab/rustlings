use std::io::{Error, ErrorKind};

async fn _my_async_call(url: &str) -> Result<serde_json::Value, Error> {
    let response = reqwest::get(url)
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve response"))?;

    let json_response = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to JSON"))?;

    Ok(json_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_async() {
        // let api_url = "https://cat-fact.herokuapp.com/facts/";
        let api_url = "https://cat-fact.herokuapp.com/facts!/"; //to make an error
        let r = _my_async_call(api_url).await;
        match r {
            Ok(r) => {
                dbg!(r);
            }
            Err(e) => {
                dbg!(e);
                // panic!("Failed to make request!");
            }
        }
    }
}
