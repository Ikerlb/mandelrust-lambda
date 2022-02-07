mod render;
mod encode;

use num::complex::Complex;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json;

use lambda_runtime::{self, Context, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;

use lambda_http::handler;
use lambda_http::Body;
use lambda_http::{Response, IntoResponse, Request};


/*#[derive(Deserialize, Debug)]
struct MyRequest {
    lower_right: String,
    upper_left: String,
}*/
#[derive(Deserialize, Debug)]
struct MyRequest {
    zoom_level: String,
}



fn from_pair<T: FromStr>(s: &str) -> Option<(T, T)>{
    let i = s.find(",")?;
    match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
        (Ok(fst), Ok(snd)) => Some((fst, snd)),
        (_, _) => None
    }
}

fn parse_complex<T: FromStr>(s: &str) -> Option<Complex<T>>{
    match from_pair(s) {
        Some((fst, snd)) => Some(Complex::new(fst, snd)),
        None => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    // can be replaced with any other method of initializing `log`
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_utc_timestamps()
        .init()
        .unwrap();

    let func = handler(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_handler(mut req: Request, _c: Context) -> Result<Response<Vec<u8>>, Error> {
    let payload = req.body_mut();
    match payload {
        Body::Text(txt) => {
            let rq: MyRequest = serde_json::from_str(txt)
                .unwrap();

            let E = core::f64::consts::E;

            let zoom_level = f64::from_str(rq.zoom_level).unwrap();
            let prop_zoom = 1 / zoom_level; 
               
            let center_point = Complex::new(-E/7.0, -E/20);
             

            let sc1 = rq.lower_right; 
            let sc2 = rq.upper_left; 
            
            let c1 = parse_complex(&sc1).expect("Upper left complex point pair has an invalid format.");
            let bounds = (c1, c2);

            let mut pixels: [u8; 1280usize * 720usize] = [0; 1280usize * 720usize];

            let img_bounds = (1280usize, 720usize);
            render::render(&mut pixels, img_bounds, bounds); 
            let buf = encode::convert_to_png(&pixels, img_bounds).unwrap();
                 
            Ok(Response::builder()
               .status(200)
               .header("Access-Control-Allow-Origin","*")
               .header("Access-Control-Allow-Methods", "GET,PUT,POST,DELETE")
               .header("Access-Control-Allow-Headers", "Content-Type")
               .body(buf)
               .unwrap())
        },
        _ => unimplemented!(),
    }
}
