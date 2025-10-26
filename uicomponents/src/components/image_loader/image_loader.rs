use base64::prelude::*;
use dioxus::prelude::*;
use reqwest::Client;

#[derive(Props, PartialEq, Clone)]
pub struct ImageLoaderProps {
    /// URL of the image to load.
    pub photo_url: String,

    /// Custom CSS classes to apply to the container `<div>` element.
    #[props(default = "".to_string())]
    pub custom_class: String,

    /// Custom inline styles to apply to the container `<div>` element.
    #[props(default = "".to_string())]
    pub custom_style: String,

    /// The children element
    pub children: Element,
}

#[component]
pub fn ImageLoader(props: ImageLoaderProps) -> Element {
    let photo_url = props.photo_url.clone();

    let image_resource = use_resource(move || {
        let photo_url = photo_url.clone(); // capture inside
        async move {
            let client = Client::new();
            let response = client.get(&photo_url).send().await;
            match response {
                Ok(res) if res.status().is_success() => {
                    let bytes = res.bytes().await.map_err(|e| e.to_string())?;
                    let image_data = BASE64_STANDARD.encode(&bytes);
                    let image_type = match photo_url.split('.').last().unwrap_or("") {
                        "png" => "image/png",
                        "jpg" | "jpeg" => "image/jpeg",
                        "gif" => "image/gif",
                        _ => "image/jpeg", // fallback
                    };
                    let image_url = format!("data:{};base64,{}", image_type, image_data);
                    Ok(image_url)
                }
                Ok(res) => Err(format!("Failed to load image: {}", res.status())),
                Err(e) => Err(format!("Failed to load image: {}", e)),
            }
        }
    });

    let background_image_style = match image_resource.read().as_ref() {
        Some(Ok(url)) => format!("background-image: url(\"{}\");", url),
        _ => "".to_string(), // fallback style
    };

    let combined_style = format!("{} {}", background_image_style, props.custom_style);

    rsx! {
        div {
            class: if let Some(container_classes) = &*image_resource.read() {
                match container_classes {
                    Ok(_) => props.custom_class.clone(),
                    Err(_) => "animate-pulse bg-gray-300".to_string(),
                }
            } else {
                format!("{} animate-pulse bg-gray-300", props.custom_class)
            },
            style: "{combined_style} background-size: cover; background-repeat: no-repeat; background-position: center;",
            {props.children}
        }
    }
}
