use std::collections::HashMap;

// Map specific fedora users to Drupal users for the migration.
lazy_static! {
    #[rustfmt::skip]
    static ref EXTENSIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("application/acad", "dwg");
        m.insert("application/arj", "arj");
        m.insert("application/base64", "mm");
        m.insert("application/binhex", "hqx");
        m.insert("application/binhex4", "hqx");
        m.insert("application/book", "book");
        m.insert("application/cdf", "cdf");
        m.insert("application/clariscad", "ccad");
        m.insert("application/commonground", "dp");
        m.insert("application/drafting", "drw");
        m.insert("application/dsptype", "tsp");
        m.insert("application/dxf", "dxf");
        m.insert("application/ecmascript", "js");
        m.insert("application/envoy", "evy");
        m.insert("application/excel", "xls");
        m.insert("application/fractals", "fif");
        m.insert("application/freeloader", "frl");
        m.insert("application/futuresplash", "spl");
        m.insert("application/gnutar", "tgz");
        m.insert("application/groupwise", "vew");
        m.insert("application/hlp", "hlp");
        m.insert("application/hta", "hta");
        m.insert("application/i-deas", "unv");
        m.insert("application/iges", "iges");
        m.insert("application/inf", "inf");
        m.insert("application/java-byte-code", "class");
        m.insert("application/java", "class");
        m.insert("application/javascript", "js");
        m.insert("application/lha", "lha");
        m.insert("application/lzx", "lzx");
        m.insert("application/mac-binary", "bin");
        m.insert("application/mac-binhex", "hqx");
        m.insert("application/mac-binhex40", "hqx");
        m.insert("application/mac-compactpro", "cpt");
        m.insert("application/macbinary", "bin");
        m.insert("application/marc", "mrc");
        m.insert("application/mbedlet", "mbd");
        m.insert("application/mcad", "mcd");
        m.insert("application/mime", "aps");
        m.insert("application/mspowerpoint", "ppt");
        m.insert("application/msword", "doc");
        m.insert("application/mswrite", "wri");
        m.insert("application/netmc", "mcp");
        m.insert("application/octet-stream", "bin");
        m.insert("application/oda", "oda");
        m.insert("application/pdf", "pdf");
        m.insert("application/pkcs-12", "p12");
        m.insert("application/pkcs-crl", "crl");
        m.insert("application/pkcs10", "p10");
        m.insert("application/pkcs7-mime", "p7m");
        m.insert("application/pkcs7-signature", "p7s");
        m.insert("application/pkix-cert", "crt");
        m.insert("application/pkix-crl", "crl");
        m.insert("application/plain", "text");
        m.insert("application/postscript", "ps");
        m.insert("application/powerpoint", "ppt");
        m.insert("application/pro_eng", "part");
        m.insert("application/rdf+xml", "rdf");
        m.insert("application/ringing-tones", "rng");
        m.insert("application/rtf", "rtf");
        m.insert("application/sdp", "sdp");
        m.insert("application/sea", "sea");
        m.insert("application/set", "set");
        m.insert("application/sla", "stl");
        m.insert("application/smil", "smil");
        m.insert("application/solids", "sol");
        m.insert("application/sounder", "sdr");
        m.insert("application/step", "step");
        m.insert("application/streamingmedia", "ssm");
        m.insert("application/toolbook", "tbk");
        m.insert("application/vda", "vda");
        m.insert("application/vnd.google-earth.kml+xml", "kml");
        m.insert("application/vnd.google-earth.kmz", "kmz");
        m.insert("application/vnd.ms-access", "mdb");
        m.insert("application/vnd.ms-excel.addin.macroEnabled.12", "xlam");
        m.insert("application/vnd.ms-excel.sheet.binary.macroEnabled.12","xlsb");
        m.insert("application/vnd.ms-excel.sheet.macroEnabled.12", "xlsm");
        m.insert("application/vnd.ms-excel.template.macroEnabled.12", "xltm");
        m.insert("application/vnd.ms-excel", "xls");
        m.insert("application/vnd.ms-powerpoint.addin.macroEnabled.12", "ppam");
        m.insert("application/vnd.ms-powerpoint.presentation.macroEnabled.12", "pptm");
        m.insert("application/vnd.ms-powerpoint.slideshow.macroEnabled.12", "ppsm");
        m.insert("application/vnd.ms-powerpoint.template.macroEnabled.12", "potm");
        m.insert("application/vnd.ms-powerpoint", "ppt");
        m.insert("application/vnd.ms-word.document.macroEnabled.12", "docm");
        m.insert("application/vnd.ms-word.template.macroEnabled.12", "dotm");
        m.insert("application/vnd.openxmlformats-officedocument.presentationml.presentation", "pptx");
        m.insert("application/vnd.openxmlformats-officedocument.presentationml.slideshow", "ppsx");
        m.insert("application/vnd.openxmlformats-officedocument.presentationml.template", "potx");
        m.insert("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", "xlsx");
        m.insert("application/vnd.openxmlformats-officedocument.spreadsheetml.template", "xltx");
        m.insert("application/vnd.openxmlformats-officedocument.wordprocessingml.document", "docx");
        m.insert("application/vnd.openxmlformats-officedocument.wordprocessingml.template", "dotx");
        m.insert("application/vocaltec-media-desc", "vmd");
        m.insert("application/vocaltec-media-file", "vmf");
        m.insert("application/wordperfect", "wp");
        m.insert("application/x-123", "wk1");
        m.insert("application/x-aim", "aim");
        m.insert("application/x-authorware-bin", "aab");
        m.insert("application/x-authorware-map", "aam");
        m.insert("application/x-authorware-seg", "aas");
        m.insert("application/x-bcpio", "bcpio");
        m.insert("application/x-binary", "bin");
        m.insert("application/x-binhex40", "hqx");
        m.insert("application/x-bsh", "sh");
        m.insert("application/x-bzip", "bz");
        m.insert("application/x-bzip2", "bz2");
        m.insert("application/x-cdf", "cdf");
        m.insert("application/x-cdlink", "vcd");
        m.insert("application/x-chat", "chat");
        m.insert("application/x-cmu-raster", "ras");
        m.insert("application/x-cocoa", "cco");
        m.insert("application/x-compactpro", "cpt");
        m.insert("application/x-compress", "z");
        m.insert("application/x-compressed", "zip");
        m.insert("application/x-cpio", "cpio");
        m.insert("application/x-cpt", "cpt");
        m.insert("application/x-csh", "csh");
        m.insert("application/x-deepv", "deepv");
        m.insert("application/x-director", "dir");
        m.insert("application/x-dvi", "dvi");
        m.insert("application/x-elc", "elc");
        m.insert("application/x-envoy", "env");
        m.insert("application/x-esrehber", "es");
        m.insert("application/x-excel", "xls");
        m.insert("application/x-frame", "mif");
        m.insert("application/x-freelance", "pre");
        m.insert("application/x-gsp", "gsp");
        m.insert("application/x-gss", "gss");
        m.insert("application/x-gtar", "gtar");
        m.insert("application/x-gzip", "gz");
        m.insert("application/x-hdf", "hdf");
        m.insert("application/x-helpfile", "help");
        m.insert("application/x-httpd-imap", "imap");
        m.insert("application/x-ima", "ima");
        m.insert("application/x-internett-signup", "ins");
        m.insert("application/x-inventor", "iv");
        m.insert("application/x-ip2", "ip");
        m.insert("application/x-java-class", "class");
        m.insert("application/x-java-commerce", "jcm");
        m.insert("application/x-javascript", "js");
        m.insert("application/x-koan", "skd");
        m.insert("application/x-ksh", "ksh");
        m.insert("application/x-latex", "latex");
        m.insert("application/x-lha", "lha");
        m.insert("application/x-lisp", "lsp");
        m.insert("application/x-livescreen", "ivy");
        m.insert("application/x-lotus", "wq1");
        m.insert("application/x-lotusscreencam", "scm");
        m.insert("application/x-lzh", "lzh");
        m.insert("application/x-lzx", "lzx");
        m.insert("application/x-mac-binhex40", "hqx");
        m.insert("application/x-macbinary", "bin");
        m.insert("application/x-mathcad", "mcd");
        m.insert("application/x-meme", "mm");
        m.insert("application/x-midi", "midi");
        m.insert("application/x-mif", "mif");
        m.insert("application/x-mix-transfer", "nix");
        m.insert("application/x-mplayer2", "asx");
        m.insert("application/x-msexcel", "xls");
        m.insert("application/x-mspowerpoint", "ppt");
        m.insert("application/x-navi-animation", "ani");
        m.insert("application/x-navidoc", "nvd");
        m.insert("application/x-navimap", "map");
        m.insert("application/x-navistyle", "stl");
        m.insert("application/x-netcdf", "cdf");
        m.insert("application/x-newton-compatible-pkg", "pkg");
        m.insert("application/x-omc", "omc");
        m.insert("application/x-omcdatamaker", "omcd");
        m.insert("application/x-omcregerator", "omcr");
        m.insert("application/x-pagemaker", "pm4");
        m.insert("application/x-pcl", "pcl");
        m.insert("application/x-pixclscript", "plx");
        m.insert("application/x-pkcs10", "p10");
        m.insert("application/x-pkcs12", "p12");
        m.insert("application/x-pkcs7-certificates", "spc");
        m.insert("application/x-pkcs7-certreqresp", "p7r");
        m.insert("application/x-pkcs7-mime", "p7c");
        m.insert("application/x-pkcs7-signature", "p7a");
        m.insert("application/x-pointplus", "css");
        m.insert("application/x-portable-anymap", "pnm");
        m.insert("application/x-project", "mpx");
        m.insert("application/x-qpro", "wb1");
        m.insert("application/x-rtf", "rtf");
        m.insert("application/x-sdp", "sdp");
        m.insert("application/x-sea", "sea");
        m.insert("application/x-seelogo", "sl");
        m.insert("application/x-sh", "sh");
        m.insert("application/x-shar", "shar");
        m.insert("application/x-shockwave-flash", "swf");
        m.insert("application/x-sit", "sit");
        m.insert("application/x-sprite", "sprite");
        m.insert("application/x-stuffit", "sit");
        m.insert("application/x-sv4cpio", "sv4cpio");
        m.insert("application/x-sv4crc", "sv4crc");
        m.insert("application/x-tar", "tar");
        m.insert("application/x-tbook", "tbk");
        m.insert("application/x-tcl", "tcl");
        m.insert("application/x-tex", "tex");
        m.insert("application/x-texinfo", "texinfo");
        m.insert("application/x-troff-man", "man");
        m.insert("application/x-troff-me", "me");
        m.insert("application/x-troff-ms", "ms");
        m.insert("application/x-troff-msvideo", "avi");
        m.insert("application/x-troff", "roff");
        m.insert("application/x-ustar", "ustar");
        m.insert("application/x-visio", "vsd");
        m.insert("application/x-vrml", "vrml");
        m.insert("application/x-wais-source", "wsrc");
        m.insert("application/x-winhelp", "hlp");
        m.insert("application/x-wintalk", "wtk");
        m.insert("application/x-world", "wrl");
        m.insert("application/x-wpwin", "wpd");
        m.insert("application/x-wri", "wri");
        m.insert("application/x-x509-ca-cert", "crt");
        m.insert("application/x-x509-user-cert", "crt");
        m.insert("application/x-zip-compressed", "zip");
        m.insert("application/xml", "xml");
        m.insert("application/zip", "zip");
        m.insert("audio/aac", "aac");
        m.insert("audio/aiff", "aiff");
        m.insert("audio/basic", "snd");
        m.insert("audio/it", "it");
        m.insert("audio/make", "funk");
        m.insert("audio/mid", "rmi");
        m.insert("audio/midi", "midi");
        m.insert("audio/mod", "mod");
        m.insert("audio/mpeg", "mp3");
        m.insert("audio/mpeg3", "mp3");
        m.insert("audio/nspaudio", "lma");
        m.insert("audio/s3m", "s3m");
        m.insert("audio/tsp-audio", "tsi");
        m.insert("audio/tsplayer", "tsp");
        m.insert("audio/voc", "voc");
        m.insert("audio/voxware", "vox");
        m.insert("audio/wav", "wav");
        m.insert("audio/x-adpcm", "snd");
        m.insert("audio/x-aiff", "aiff");
        m.insert("audio/x-au", "au");
        m.insert("audio/x-gsm", "gsm");
        m.insert("audio/x-jam", "jam");
        m.insert("audio/x-liveaudio", "lam");
        m.insert("audio/x-mid", "midi");
        m.insert("audio/x-midi", "midi");
        m.insert("audio/x-mod", "mod");
        m.insert("audio/x-mpeg-3", "mp3");
        m.insert("audio/x-mpeg", "mp2");
        m.insert("audio/x-mpequrl", "m3u");
        m.insert("audio/x-nspaudio", "lma");
        m.insert("audio/x-pn-realaudio-plugin", "rpm");
        m.insert("audio/x-pn-realaudio", "ra");
        m.insert("audio/x-psid", "sid");
        m.insert("audio/x-realaudio", "ra");
        m.insert("audio/x-twinvq-plugin", "vqe");
        m.insert("audio/x-twinvq", "vqf");
        m.insert("audio/x-voc", "voc");
        m.insert("audio/x-wav", "wav");
        m.insert("audio/xm", "xm");
        m.insert("chemical/x-pdb", "pdb");
        m.insert("drawing/x-dwf (old)", "dwf");
        m.insert("i-world/i-vrml", "ivr");
        m.insert("image/bmp", "bmp");
        m.insert("image/cmu-raster", "rast");
        m.insert("image/fif", "fif");
        m.insert("image/florian", "flo");
        m.insert("image/g3fax", "g3");
        m.insert("image/gif", "gif");
        m.insert("image/ief", "ief");
        m.insert("image/jp2", "jp2");
        m.insert("image/jpeg", "jpg");
        m.insert("image/jpg", "jpg");
        m.insert("image/jutvision", "jut");
        m.insert("image/naplps", "naplps");
        m.insert("image/pict", "pict");
        m.insert("image/pjpeg", "jpg");
        m.insert("image/png", "png");
        m.insert("image/tif", "tiff");
        m.insert("image/tiff", "tiff");
        m.insert("image/vasa", "mcf");
        m.insert("image/x-cmu-raster", "ras");
        m.insert("image/x-dwg", "dwg");
        m.insert("image/x-icon", "ico");
        m.insert("image/x-jg", "art");
        m.insert("image/x-jps", "jps");
        m.insert("image/x-niff", "niff");
        m.insert("image/x-pcx", "pcx");
        m.insert("image/x-pict", "pct");
        m.insert("image/x-portable-anymap", "pnm");
        m.insert("image/x-portable-bitmap", "pbm");
        m.insert("image/x-portable-graymap", "pgm");
        m.insert("image/x-portable-greymap", "pgm");
        m.insert("image/x-portable-pixmap", "ppm");
        m.insert("image/x-quicktime", "qif");
        m.insert("image/x-rgb", "rgb");
        m.insert("image/x-tiff", "tiff");
        m.insert("image/x-windows-bmp", "bmp");
        m.insert("image/x-xbitmap", "xbm");
        m.insert("image/x-xbm", "xbm");
        m.insert("image/x-xpixmap", "xpm");
        m.insert("image/x-xwd", "xwd");
        m.insert("image/x-xwindowdump", "xwd");
        m.insert("image/xbm", "xbm");
        m.insert("image/xpm", "xpm");
        m.insert("message/rfc822", "mime");
        m.insert("model/iges", "iges");
        m.insert("model/vrml", "vrml");
        m.insert("model/x-pov", "pov");
        m.insert("multipart/x-gzip", "gzip");
        m.insert("multipart/x-ustar", "ustar");
        m.insert("multipart/x-zip", "zip");
        m.insert("music/crescendo", "midi");
        m.insert("music/x-karaoke", "kar");
        m.insert("paleovu/x-pv", "pvu");
        m.insert("text/asp", "asp");
        m.insert("text/css", "css");
        m.insert("text/csv", "csv");
        m.insert("text/ecmascript", "js");
        m.insert("text/html", "html");
        m.insert("text/javascript", "js");
        m.insert("text/mcf", "mcf");
        m.insert("text/pascal", "pas");
        m.insert("text/plain", "txt");
        m.insert("text/richtext", "rtf");
        m.insert("text/sgml", "sgml");
        m.insert("text/tab-separated-values", "tsv");
        m.insert("text/uri-list", "uri");
        m.insert("text/webviewhtml", "htt");
        m.insert("text/x-asm", "asm");
        m.insert("text/x-audiosoft-intra", "aip");
        m.insert("text/x-c", "c");
        m.insert("text/x-component", "htc");
        m.insert("text/x-fortran", "f");
        m.insert("text/x-h", "h");
        m.insert("text/x-java-source", "java");
        m.insert("text/x-la-asf", "lsx");
        m.insert("text/x-m", "m");
        m.insert("text/x-pascal", "p");
        m.insert("text/x-script", "hlb");
        m.insert("text/x-server-parsed-html", "shtml");
        m.insert("text/x-setext", "etx");
        m.insert("text/x-sgml", "sgml");
        m.insert("text/x-speech", "spc");
        m.insert("text/x-uil", "uil");
        m.insert("text/x-uuencode", "uu");
        m.insert("text/x-vcalendar", "vcs");
        m.insert("text/xml", "xml");
        m.insert("video/animaflex", "afl");
        m.insert("video/avi", "avi");
        m.insert("video/avs-video", "avs");
        m.insert("video/dl", "dl");
        m.insert("video/fli", "fli");
        m.insert("video/gl", "gl");
        m.insert("video/mp4", "mp4");
        m.insert("video/mpeg", "mpeg");
        m.insert("video/msvideo", "avi");
        m.insert("video/quicktime", "mov");
        m.insert("video/vdo", "vdo");
        m.insert("video/vivo", "vivo");
        m.insert("video/vosaic", "vos");
        m.insert("video/x-amt-demorun", "xdr");
        m.insert("video/x-amt-showrun", "xsr");
        m.insert("video/x-atomic3d-feature", "fmf");
        m.insert("video/x-dl", "dl");
        m.insert("video/x-dv", "dv");
        m.insert("video/x-fli", "fli");
        m.insert("video/x-gl", "gl");
        m.insert("video/x-isvideo", "isu");
        m.insert("video/x-matroska", "mkv");
        m.insert("video/x-motion-jpeg", "mjpg");
        m.insert("video/x-mpeg", "mp3");
        m.insert("video/x-mpeq2a", "mp2");
        m.insert("video/x-ms-asf-plugin", "asx");
        m.insert("video/x-ms-asf", "asf");
        m.insert("video/x-msvideo", "avi");
        m.insert("video/x-qtc", "qtc");
        m.insert("video/x-scm", "scm");
        m.insert("video/x-sgi-movie", "movie");
        m.insert("windows/metafile", "wmf");
        m.insert("www/mime", "mime");
        m.insert("x-conference/x-cooltalk", "ice");
        m.insert("x-music/x-midi", "mid");
        m.insert("x-world/x-3dmf", "3dmf");
        m.insert("x-world/x-svr", "svr");
        m.insert("x-world/x-vrml", "vrml");
        m.insert("x-world/x-vrt", "vrt");
        m.insert("xgl/drawing", "xgz");
        m.insert("xgl/movie", "xmz");
        m
    };
}

pub fn generate_file_name(
    object: &foxml::Foxml,
    version: &foxml::FoxmlDatastreamVersion,
) -> String {
    let extension = EXTENSIONS
        .get(&version.mime_type.as_str())
        .unwrap_or_else(|| panic!("No extension known for mime type: {}", &version.mime_type));
    let is_filename = EXTENSIONS
        .values()
        .any(|extension| version.label.ends_with(&format!(".{}", extension)));
    if is_filename {
        version.label.clone()
    } else {
        format!("{}.{}.{}", &version.id, &object.pid, &extension)
    }
}
