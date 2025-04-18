use arboard::Clipboard;
use image::DynamicImage;
use std::fs;
use std::{io::Write, path::Path};
use webp::Encoder;

fn encode_and_save_webp(dyn_img: &DynamicImage, webp_path: &Path) {
    let encoder = Encoder::from_image(dyn_img).expect("webp编码失败");
    let webp_bytes = encoder.encode(70.0); // 10.0为较高压缩率，画质较低
    // 保存webp
    let mut file = fs::File::create(webp_path).expect("无法创建webp文件");
    file.write_all(&webp_bytes).expect("写入webp文件失败");
}

fn main() {
    let mut clipboard = Clipboard::new().expect("无法访问粘贴板");
    // 1. 优先尝试读取文本（文件路径）
    if let Ok(text) = clipboard.get().file_list() {
        if !text.is_empty() {
            let path = text[0].clone();
            if path.exists() && path.is_file() {
                if let Ok(dyn_img) = image::open(&path) {
                    let webp_path = path.with_extension("webp");
                    encode_and_save_webp(&dyn_img, &webp_path);
                    return;
                } else {
                    println!("文件不是有效图片: {}", path.display());
                    return;
                }
            }
        }
    }
    // 2. 如果不是图片路径，尝试读取图片内容
    if let Ok(img_data) = clipboard.get_image() {
        let width = img_data.width as u32;
        let height = img_data.height as u32;
        let bytes = img_data.bytes.into_owned();
        let dyn_img = DynamicImage::ImageRgba8(
            image::RgbaImage::from_vec(width, height, bytes).expect("图片数据无效"),
        );
        // uuid生成webp文件名
        let uuid = uuid::Uuid::new_v4().to_string();
        let webp_filename = format!("{}.webp", &uuid[..6]);
        let webp_path = Path::new(&webp_filename);
        encode_and_save_webp(&dyn_img, webp_path);
        return;
    }
    println!("粘贴板无图片内容，也无有效图片文件路径");
}
