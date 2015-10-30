/* Adapted from example.c from mupdf. */
/* Compile with: gcc -o mupdf-example-simple mupdf-example-simple.c -lmupdf -lmujs -lfreetype -ljbig2dec -ljpeg -lopenjpeg -lz -lm -lopenjp2 -lcrypto */

#include <stdio.h>

#include <mupdf/fitz.h>

int main() {
  char *filename = "test.pdf";
  
  // Create a context to hold the exception stack and various caches.
  fz_context *ctx = fz_new_context(NULL, NULL, FZ_STORE_UNLIMITED);

  // Register document handlers for the default file types we support.
  fz_register_document_handlers(ctx);

  // Open the PDF, XPS or CBZ document.
  fz_document *doc = fz_open_document(ctx, filename);

  // Retrieve the number of pages
  int pagecount = fz_count_pages(ctx, doc);
  printf("pages: %d\n", pagecount);

  return 0;
}
