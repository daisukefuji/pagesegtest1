use leptonica_sys::*;
use std::env;
use std::os::raw::c_char;
use std::ptr;

fn main() {
    let main_name = "pagesegtest1";

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Error in {}: Syntax:  pagesegtest1 filein", main_name);
    }
    let filein = &args[1];

    unsafe {
        setLeptDebugOK(1);
        let mut pixs = pixRead(filein.as_ptr() as *const c_char);
        if pixs.is_null() {
            panic!("Error in {}: pixs not made", main_name);
        }
        let mut pixadb = pixaCreate(0);
        let mut pixhm: *mut Pix = ptr::null_mut();
        let mut pixtm: *mut Pix = ptr::null_mut();
        let mut pixtb: *mut Pix = ptr::null_mut();
        pixGetRegionsBinary(pixs, &mut pixhm, &mut pixtm, &mut pixtb, pixadb);
        pixDestroy(&mut pixhm);
        pixDestroy(&mut pixtm);
        pixDestroy(&mut pixtb);
        pixDestroy(&mut pixs);

        /* Display intermediate images in a single image */
        lept_mkdir("lept/pagseg".as_ptr() as *const c_char);
        let mut pixd = pixaDisplayTiledAndScaled(pixadb, 32, 400, 4, 0, 20, 3);
        pixWrite(
            "/tmp/lept/pageseg/debug.png".as_ptr() as *const c_char,
            pixd,
            3, /*IFF_PNG*/
        );
        pixaDestroy(&mut pixadb);
        pixDestroy(&mut pixd);
    }
}
