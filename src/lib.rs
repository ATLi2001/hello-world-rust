use worker::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MyValue {
    pub version_number: i32,
    pub data: Vec<u8>,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {

    console_log!(
        "{} {}, located at: {:?}, within: {}",
        req.method().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );

    let store = env.kv("KV_FROM_RUST")?;
    console_debug!("store created");
    
    // put n key value pairs
    let n = 10;
    for i in 0..n {
        let key: String = format!("testKey{i}");
        let value_data = format!("testValue{i}");
        let value = MyValue {
            version_number: i,
            data: Vec::from(value_data),
        };
        store.put(&key, value)?.execute().await?;
    }

    // call get on them
    for i in 0..n {
        let key: String = format!("testKey{i}");
        let val: MyValue = store.get(&key).json().await?.unwrap();
        console_debug!("version_number: {:?}, data: {:?}", val.version_number, String::from_utf8(val.data).unwrap());
    }
    
    Response::ok("Hello, World!\n")
}
