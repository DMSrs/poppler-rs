extern crate cairo;
extern crate glib;
extern crate glib_sys;
extern crate poppler;

use cairo::{Context, Format, ImageSurface, PdfSurface};
use poppler::{PopplerDocument, PopplerPage};
use std::{fs::File, io::Read};

#[test]
fn test1() {
    let filename = "tests/test.pdf";
    let doc = PopplerDocument::new_from_file(filename, "").unwrap();
    let num_pages = doc.get_n_pages();

    println!("Document has {} page(s)", num_pages);

    let mut surface = PdfSurface::new(420.0, 595.0, "tests/output.pdf").unwrap();
    let ctx = Context::new(&mut surface);

    // FIXME: move iterator to poppler
    for page_num in 0..num_pages {
        let page = doc.get_page(page_num).unwrap();
        let (w, h) = page.get_size();
        println!("page {} has size {}, {}", page_num, w, h);
        surface.set_size(w, h);

        ctx.save();
        page.render(&ctx);

        println!("Text: {:?}", page.get_text().unwrap_or(""));

        ctx.restore();
        ctx.show_page();
    }
    // g_object_unref (page);
    //surface.write_to_png("file.png");
    surface.finish();
}

#[test]
fn test2_from_file() {
    let path = "tests/test.pdf";
    let doc: PopplerDocument = PopplerDocument::new_from_file(path, "upw").unwrap();
    let num_pages = doc.get_n_pages();
    let title = doc.get_title().unwrap();
    let metadata = doc.get_metadata();
    let version_string = doc.get_pdf_version_string();
    let permissions = doc.get_permissions();
    let page: PopplerPage = doc.get_page(0).unwrap();
    let (w, h) = page.get_size();

    println!(
        "Document {} has {} page(s) and is {}x{}",
        title, num_pages, w, h
    );
    println!(
        "Version: {:?}, Permissions: {:x?}",
        version_string, permissions
    );

    assert!(metadata.is_some());
    assert_eq!(version_string, Some("PDF-1.3".to_string()));
    assert_eq!(permissions, 0xff);

    assert_eq!(title, "This is a test PDF file");

    let mut surface = ImageSurface::create(Format::ARgb32, w as i32, h as i32).unwrap();
    let ctx = Context::new(&mut surface);

    ctx.save();
    page.render(&ctx);
    ctx.restore();
    ctx.show_page();

    let mut f: File = File::create("tests/out.png").unwrap();
    surface.write_to_png(&mut f).expect("Unable to write PNG");
}
#[test]
fn test2_from_data() {
    let path = "tests/test.pdf";
    let mut file = File::open(path).unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let doc: PopplerDocument = PopplerDocument::new_from_data(&mut data[..], "upw").unwrap();
    let num_pages = doc.get_n_pages();
    let title = doc.get_title().unwrap();
    let metadata = doc.get_metadata();
    let version_string = doc.get_pdf_version_string();
    let permissions = doc.get_permissions();
    let page: PopplerPage = doc.get_page(0).unwrap();
    let (w, h) = page.get_size();

    println!(
        "Document {} has {} page(s) and is {}x{}",
        title, num_pages, w, h
    );
    println!(
        "Version: {:?}, Permissions: {:x?}",
        version_string, permissions
    );

    assert!(metadata.is_some());
    assert_eq!(version_string, Some("PDF-1.3".to_string()));
    assert_eq!(permissions, 0xff);
}

#[test]
fn test3() {
    let mut data = vec![];

    assert!(PopplerDocument::new_from_data(&mut data[..], "upw").is_err());
}
