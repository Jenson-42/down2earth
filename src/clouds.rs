/// This part of the program is based on code by Kevin Keegan.
/// The original code and its license can be viewed at https://gist.github.com/krkeegan/64e96290eb6569790d230085016501da
/// 
/// Image Source: SSEC RealEarth, UW-Madison

use std::path::Path;
use serde::Deserialize;
use log::{error, trace, debug};
use reqwest::{blocking::{Client, ClientBuilder, Response}, header::HeaderMap, IntoUrl};

// Cloud stuff
const PRODUCT: &str = "globalir";
const API_URL: &str = "https://re.ssec.wisc.edu/api";

const URL_RETRIES: u32 = 3;

/// Needed to deserialize the JSON date from the clouds API.
#[derive(Deserialize, Debug)]
struct DateResponse {
    globalir: [String; 2]
}

#[derive(Debug)]
pub enum CloudDownloadError {
    // Allowing dead code because these errors are only used for debug logging. 
    #[allow(dead_code)]
    HttpError(reqwest::Error),
    #[allow(dead_code)]
    TileDecodeError(image::ImageError),
    #[allow(dead_code)]
    ImageSaveError(image::ImageError),
}

/// Retry a request more than once before giving up.
fn retry<U: IntoUrl>(url: U, client: Client, retries: u32) -> reqwest::Result<Response> {
    let url = url.into_url().unwrap();

    for i in 0..(retries - 1) {
        trace!("Attempting to connect to '{}', try {}/{retries}", url.as_str(), i+1);
        match client.get(url.clone()).send() {
            Ok(response) => return Ok(response),
            Err(_) => continue,
        }
    }

    trace!("Attempting to connect to '{}', try {retries}/{retries}", url.as_str());
    reqwest::blocking::get(url)
} 

fn log_http_error(http_error: &reqwest::Error) {
    if http_error.status().expect("HTTP Error should have status").as_u16() == 403 {
        error!("There was an authorization error trying to download the cloud map.");
        error!("Check that your API Key is valid, and that you haven't hit your usage limit.");
        debug!("HTTP authorization error: {http_error}");
    }
    error!("There was an unknown error downloading the cloud map.");
    error!("The most likely cause is that you're not connected to the internet.");
    error!("Or it could be that the service is down or discontinued.");
    debug!("HTTP Error: {http_error}");
}

/// Download the cloud map from the API and stitch them together as an image.
pub fn download_clouds<P: AsRef<Path>>(zoom: u32, output: P, api_key: Option<String>) -> Result<(), CloudDownloadError> {
    // Create the HTTP client and add the API key header if necessary.
    let mut headers = HeaderMap::new();
    if let Some(api_key) = api_key {
        headers.insert("RE-Access-Key", api_key.parse().expect("API Key header couldn't be parsed"));
    }
    let client = ClientBuilder::new().default_headers(headers).build().expect("HTTP Client couldn't be created.");

    // Get date info from the API.
    let date_url = format!("{API_URL}/time?products={PRODUCT}");
    debug!("Sending date request to '{date_url}");
    let date_response = match retry(date_url, client.clone(), URL_RETRIES) {
        Ok(response) => response,
        Err(e) => {
            log_http_error(&e);
            return Err(CloudDownloadError::HttpError(e));
        },
    };
    let date_text = date_response.text().unwrap();
    let date: DateResponse = serde_json::from_str(&date_text).unwrap();
    let [date0, date1] = date.globalir;

    let base_url = format!("{API_URL}/image?products={PRODUCT}_{date0}_{date1}&equirectangular=true&z={zoom}");

    let tiles_x = 2_u32.pow(zoom) * 2; 
    let tiles_y = 2_u32.pow(zoom);
    debug!("Will be requesting {tiles_x} by {tiles_y} 256x265px tiles from the cloud API.");

    let mut image = image::RgbImage::new(tiles_x * 256, tiles_y * 256);

    for x in 0..tiles_x {
        for y in 0..tiles_y {
            let tile_url = format!("{base_url}&x={x}&y={y}");
            trace!("Getting tile ({x}, {y}) from '{tile_url}'.");

            let response = match retry(&tile_url, client.clone(), URL_RETRIES) {
                Ok(response) => response,
                Err(e) => {
                    log_http_error(&e);
                    return Err(CloudDownloadError::HttpError(e));
                },
            };

            let bytes = response.bytes().expect("HTTP response didn't convert to bytes");
            trace!("Downloaded {} bytes.", bytes.len());
            
            let tile = match image::load_from_memory(&bytes) {
                Ok(tile) => tile,
                Err(e) => {
                    error!("There was an error downloading the cloud map.");
                    debug!("Byte data for cloud tile ({x}, {y}) couldn't be decoded as an image: {e}");
                    return Err(CloudDownloadError::TileDecodeError(e));
                }
            };

            image::imageops::overlay(&mut image, &tile.to_rgb8(), (x*256) as i64, (y*256) as i64);
        }
    }

    if let Err(error) = image.save(&output) {
        error!("There was an error saving the cloud map.");
        return Err(CloudDownloadError::ImageSaveError(error))
    }
    debug!("Image saved successfully at '{}'", output.as_ref().display());

    Ok(())
}
