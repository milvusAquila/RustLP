# RustLP
A simple software inspired by [OpenLP](https://openlp.org/)

## Installation
Get the pdfium lib [there](https://github.com/bblanchon/pdfium-binaries)
(extract the archive and get the pdfium-\*/lib/libpdfium.so file)

For example :
```bash
wget https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-x64.tgz
tar -zxf pdfium-linux-x64.tgz lib/libpdfium.so
mv lib/libpdfium.so .
rm -r lib
```
