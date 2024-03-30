use image::ImageFormat;
use pdfium_render::prelude::*;
use pdfium_render::render_config::PdfRenderConfig;

pub fn get_images_from_pdf(
    document: PdfDocument,
    image_format: &ImageFormat,
    include_thumbnail: Option<bool>,
) {
    let render_config = PdfRenderConfig::new()
        .scale_page_by_factor(2.0)
        .render_annotations(false)
        .use_print_quality(true);
    let render_config_thumbnail = PdfRenderConfig::new()
        .scale_page_by_factor(0.5)
        .render_annotations(false)
        .use_print_quality(true);

    document
        .pages()
        .iter()
        .enumerate()
        .for_each(|(index, page)| {
            println!("Rendering page {}", index + 1);
            page.render_with_config(&render_config)
                .unwrap()
                .as_image()
                .save_with_format(
                    format!(
                        "images/slide-source-{}.{}",
                        index + 1,
                        image_format.extensions_str().first().unwrap()
                    ),
                    *image_format,
                )
                .unwrap();

            if include_thumbnail.unwrap_or(false) {
                page.render_with_config(&render_config_thumbnail)
                    .unwrap()
                    .as_image()
                    .save_with_format(
                        format!(
                            "images/slide-source-{}-thumbnail.{}",
                            index + 1,
                            image_format.extensions_str().first().unwrap()
                        ),
                        *image_format,
                    )
                    .unwrap();
            }
        });
}
