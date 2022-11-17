use headless_chrome::Tab;
use std::fs;
use std::sync::Arc;

use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;

pub fn screenshot(tab: &Arc<Tab>, id: String) {
    let png_data = tab
        .wait_for_element("body")
        .expect("couldn't find body element")
        .capture_screenshot(CaptureScreenshotFormatOption::Png)
        .expect("couldn't take a screenshot");
    fs::write(format!("./screenshots/{}.png", id), &png_data)
        .unwrap_or_else(|_| panic!("couldn't take a screenshot!"));
}
