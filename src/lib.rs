use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {

    console_log!(
        "{} {}, located at: {:?}, within: {}",
        req.method().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );

    let store = env.kv("KV_FROM_RUST")?;
    console_debug!("store created");
    store.put("testKey", "testValue")?.execute().await?;

    let uri = "https://google.com";
    let request = Request::new(uri, Method::Get)?;
    console_debug!("{:?}", request);

    let fetch = Fetch::Request(request);
    
    let mut response = fetch.send().await?;
    let json = response.text().await?;
    console_debug!("{:?}", json);

    let val = store.get("testKey").text().await?;
    console_debug!("{:?}", val.unwrap());

    Response::ok("Hello, World!")
}
