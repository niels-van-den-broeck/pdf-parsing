use pdfium_render::prelude::*;

pub fn load_pdfium() -> Pdfium {
    // Load Pdfium from local binary
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())
            .expect("Error loading pdfium"),
    );

    return pdfium;
}

pub fn parse_document<'a>(pdfium: &'a Pdfium, byte_vector: Vec<u8>) -> PdfDocument<'a> {
    // Instantiate Pdfium from either local binary or systen binary.
    let document = pdfium
        .load_pdf_from_byte_vec(byte_vector, Some(""))
        .expect("Error loading file. Is this a pdf file?");

    return document;
}
