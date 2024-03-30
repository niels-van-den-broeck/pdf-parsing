# PDF parser

Can extract images and thumbnails from PDF files.

Uses PDFium as it's rendering engine. Binaries for pdfium can be downloaded and extracted from https://github.com/bblanchon/pdfium-binaries?tab=readme-ov-file


TODO:
- Text extraction with positions
- Create separated modules (move logic from main.rs to separate modules)
    - PDF image generator
    - PDF text generator
        - Optionally take in pdf or parse it by itself?
- Make deployable with deployable Docker file
    - download binary in docker file instead of static binary
- Unit Testing?
