/*
 * Copyright 2025-2026 Yury Fedoseev and pdf_oxide contributors.
 * Licensed under MIT OR Apache-2.0.
 */
package fyi.oxide.pdf;

import fyi.oxide.pdf.compliance.PdfALevel;
import fyi.oxide.pdf.conversion.ConversionResult;
import fyi.oxide.pdf.exception.PdfUnsupportedException;
import fyi.oxide.pdf.internal.NativeLoader;
import java.util.Objects;

public final class PdfAConverter {

    static {
        NativeLoader.ensureLoaded();
    }

    private PdfAConverter() {}

    /**
     * Convert a PDF document to PDF/A compliance.
     *
     * @param pdfBytes the source PDF bytes
     * @param level the target PDF/A conformance level
     * @return ConversionResult containing the converted PDF bytes and metadata
     * @throws PdfUnsupportedException for PDF/A-4 levels
     */
    public static ConversionResult convert(byte[] pdfBytes, PdfALevel level) {
        Objects.requireNonNull(pdfBytes, "pdfBytes");
        Objects.requireNonNull(level, "level");
        if (level.ordinal() >= 8) {
            throw new PdfUnsupportedException("PDF/A-4 levels not yet supported.");
        }
        return nativeConvertToPdfA(pdfBytes, level.ordinal());
    }

    private static native ConversionResult nativeConvertToPdfA(byte[] pdfBytes, int levelOrdinal);
}
