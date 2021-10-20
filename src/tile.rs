use artano::Annotation;
use artano::Canvas;
use artano::Position;
use bytes::BufMut;
use rusttype::Font;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TileError {
  ///It was not possible to laod the keys used for validation of the signature
  #[error("Failed to load JWKS keystore from {0:?}")]
  GenericTileError(String),
}

pub async fn create_tile_image(text: String) -> Result<bytes::buf::Writer<Vec<u8>>, TileError> {
  let font_data: &[u8] =
    include_bytes!("../resources/Dancing_Script/static/DancingScript-Bold.ttf");
  let font: Font<'static> = Font::try_from_bytes(font_data).expect("Map bytes to Font failed");

  let annotation = Annotation {
    position: Position::Middle,
    text: text,
  };

  let image_data: &[u8] = include_bytes!("../resources/tegeltje_small.jpg");
  let mut canvas =
    Canvas::read_from_buffer(image_data).expect("create Canvas from bytes for tegeltje.jpg failed");
  canvas.add_annotation(&annotation, &font, 1.0);
  canvas.render();

  let mut buffer = vec![].writer();
  buffer.get_mut().reserve(604800);

  canvas
    .save_png(&mut buffer)
    .expect("Tried to store the image canvas in the buffer");

  Ok(buffer)
}
