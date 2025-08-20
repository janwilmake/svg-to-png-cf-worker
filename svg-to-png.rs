// Import the worker crate for Cloudflare Workers functionality
use worker::*;
// Import panic hook for better error messages in the browser console
use console_error_panic_hook::set_once as set_panic_hook;
// Import Deserialize trait to convert JSON data into Rust structs
use serde::Deserialize;

// Define a struct (like a class in other languages) to represent the JSON body of POST requests
// The #[derive(Deserialize)] automatically generates code to convert JSON into this struct
#[derive(Deserialize)]
struct RequestBody {
    url: String, // A field to hold the URL string from the JSON
}

// This is the main entry point for the Cloudflare Worker
// "async" means this function can wait for other operations without blocking
// The function takes: request object, environment variables, and context
#[event(fetch)] // tells the system this function handles HTTP requests
pub async fn main(mut req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    // Log the current timestamp and request path to the console
    console_log!("{} - [{}]", Date::now().to_string(), req.path());

    // Set up better error handling for debugging
    set_panic_hook();

    // Check what HTTP method was used (GET, POST, etc.) and handle accordingly
    match req.method() {
        Method::Get => {
            // For GET requests, extract the path (everything after the first /) as the image URL
            let image_path = req.path()[1..].to_string();
            // Try to render the SVG and handle any errors
            match handle_render(image_path).await {
                Err(err) => {
                    // If there's an error, print it and return a 500 error response
                    println!("error: {:?}", err);
                    Response::error(format!("an unexpected error occurred: {}", err), 500)
                }
                Ok(res) => Ok(res), // If successful, return the response
            }
        }
        Method::Post => {
            // For POST requests, parse the JSON body to get the URL
            let body = req.json::<RequestBody>().await?;
            // Try to render the SVG using the URL from the JSON body
            match handle_render(body.url).await {
                Err(err) => {
                    // Same error handling as GET
                    println!("error: {:?}", err);
                    Response::error(format!("an unexpected error occurred: {}", err), 500)
                }
                Ok(res) => Ok(res),
            }
        }
        // For any other HTTP methods (PUT, DELETE, etc.), return a 405 error
        _ => Response::error("Method not allowed", 405),
    }
}

// This function does the actual work of fetching an SVG and converting it to PNG
// "async" means it can wait for network requests and other slow operations
async fn handle_render(svg_url: String) -> Result<Response> {
    // Create default options for the SVG parser
    let opt = usvg::Options::default();
    // Log the URL we're about to fetch
    console_log!("svgUrl: {}", svg_url);
    
    // Try to parse the string into a proper URL object
    // "map_err" converts any parsing error into a string error message
    let url = Url::parse(&svg_url)
        .map_err(|err| format!("failed to parse URL: {}", err))?;

    // Make an HTTP request to fetch the SVG file
    // "await" means wait for the network request to complete
    let mut res = Fetch::Url(url)
        .send()
        .await
        .map_err(|err| format!("failed to request remote image: {}", err))?;
    
    // Check if the HTTP request was successful (status code 200)
    if res.status_code() != 200 {
        // If not successful, get the error message and return an error response
        let body = res.text().await?;
        return Response::error(
            format!("upstream image returned: {}: {}", res.status_code(), body),
            500,
        );
    }
    
    // Get the SVG file content as bytes (raw data)
    let svg_data = res.bytes().await?;

    // Parse the SVG data into a tree structure that can be rendered
    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref())
        .map_err(|err| format!("failed to decode SVG: {}", err))?;

    // Calculate the size needed for the output image based on the SVG dimensions
    let pixmap_size = rtree.svg_node().size.to_screen_size();
    
    // Create a new blank image canvas with the calculated dimensions
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or("failed to create new pixmap")?;
    
    // Render the SVG onto the image canvas
    // This converts the vector graphics into pixel data
    resvg::render(
        &rtree,                           // The SVG tree to render
        usvg::FitTo::Original,           // Keep original size
        tiny_skia::Transform::default(), // No transformations (rotation, scaling, etc.)
        pixmap.as_mut(),                 // The canvas to draw on
    )
    .ok_or("failed to render PNG")?; // Convert None result to error message

    // Convert the rendered image into PNG format (compressed bytes)
    let out = pixmap
        .encode_png()
        .map_err(|err| format!("failed to encode PNG: {}", err))?;

    // Create HTTP headers to tell the browser this is a PNG image
    let mut headers = Headers::new();
    headers.set("content-type", "image/png").unwrap();
    
    // Return the PNG image data with proper headers
    Ok(Response::from_bytes(out).unwrap().with_headers(headers))
}