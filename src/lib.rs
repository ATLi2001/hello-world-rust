use worker::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MyValue {
    pub version: i32,
    pub value: Vec<u8>,
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
    let n = 1;
    for i in 1..(n+1) {
        let key: String = format!("net-item-{i}");
        let value_data = format!("wrong dummy value");
        let my_value = MyValue {
            version: 1,
            value: Vec::from(value_data),
        };
        store.put(&key, my_value)?.execute().await?;
    }

    // call get on them
    for i in 1..(n+1) {
        let key: String = format!("net-item-{i}");
        let val: MyValue = store.get(&key).json().await?.unwrap();
        console_debug!("version_number: {:?}, data: {:?}", val.version, String::from_utf8(val.value).unwrap());
    }

    // pretend to do some work
    console_log!("doing some work...");
    //let work_time = std::time::Duration::from_millis(300);
    //Delay::from(work_time).await;
    //let uri = "https://google.com";
    //let request = Request::new(uri, Method::Get)?;
    //let fetch = Fetch::Request(request);
    //let mut response = fetch.send().await?;
    //let json = response.text().await?;
    console_log!("work done!");

    Response::ok("Hello, World!\n")
}
