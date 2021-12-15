use qrcode::QrCode;
use qrcode::render::svg;

pub fn generate_qr(url: &str) -> String  {
   let code = QrCode::new(url.as_bytes()).unwrap();
   code.render::<svg::Color>().build()
}
