/*
 * Copyright 2025-2026 Yury Fedoseev and pdf_oxide contributors.
 * Licensed under MIT OR Apache-2.0.
 */
package fyi.oxide.pdf;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import fyi.oxide.pdf.compliance.PdfALevel;
import fyi.oxide.pdf.conversion.ConversionResult;
import fyi.oxide.pdf.exception.PdfUnsupportedException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

class PdfAConverterTest {

    private static Path fixturesDir;
    private static byte[] pdfBytes;

    @BeforeAll
    static void resolveFixtures() throws Exception{
        fixturesDir = Paths.get("..")
                .resolve("tests")
                .resolve("fixtures")
                .toAbsolutePath()
                .normalize();
        org.junit.jupiter.api.Assumptions.assumeTrue(
                Files.isDirectory(fixturesDir), "fixtures dir not present: " + fixturesDir);
        pdfBytes = Files.readAllBytes(fixturesDir.resolve("1008.3918v2.pdf"));
    }

    @Test
    void convertToPdfA1bReturnsValidPdf() throws Exception {
        assertConvertProducesValidPdf(PdfALevel.A_1B);
    }

    @Test
    void convertToPdfA2bReturnsValidPdf() throws Exception {
        assertConvertProducesValidPdf(PdfALevel.A_2B);
    }

    @Test
    void convertToPdfA2uReturnsValidPdf() throws Exception {
        assertConvertProducesValidPdf(PdfALevel.A_2U);
    }

    @Test
    void convertToPdfA3bReturnsValidPdf() throws Exception {
        assertConvertProducesValidPdf(PdfALevel.A_3B);
    }

    private static void assertConvertProducesValidPdf(PdfALevel level) throws Exception {
        ConversionResult result = PdfAConverter.convert(pdfBytes, level);
        assertThat(result).isNotNull();
        assertThat(result.level()).isEqualTo(level);
        assertThat(result.actions()).isNotNull();
        assertThat(result.errors()).isNotNull();
        assertThat(result.convertedBytes()).isNotEmpty();
        assertThat(result.convertedBytes()).startsWith("%PDF".getBytes());
        // Verify the converted bytes are a valid PDF with page content
        try (PdfDocument doc = PdfDocument.open(result.convertedBytes())) {
            assertThat(doc.pageCount()).isGreaterThan(0);
        }
    }

    @Test
    void pdfA4LevelsThrowUnsupported() throws Exception {
        byte[] pdfBytes = Files.readAllBytes(fixturesDir.resolve("1008.3918v2.pdf"));
        assertThatThrownBy(() -> PdfAConverter.convert(pdfBytes, PdfALevel.A_4))
                .isInstanceOf(PdfUnsupportedException.class);
        assertThatThrownBy(() -> PdfAConverter.convert(pdfBytes, PdfALevel.A_4E))
                .isInstanceOf(PdfUnsupportedException.class);
        assertThatThrownBy(() -> PdfAConverter.convert(pdfBytes, PdfALevel.A_4F))
                .isInstanceOf(PdfUnsupportedException.class);
    }

    @Test
    void conversionRecordsXmpAction() throws Exception {
        byte[] pdfBytes = Files.readAllBytes(fixturesDir.resolve("1008.3918v2.pdf"));
        ConversionResult result = PdfAConverter.convert(pdfBytes, PdfALevel.A_2B);
        assertThat(result.actions()).isNotEmpty();
        assertThat(result.actions().stream()
                .anyMatch(a -> a.actionType().name().contains("XMP"))).isTrue();
    }

    @Test
    void convertedBytesRoundTrip() throws Exception {
        byte[] pdfBytes = Files.readAllBytes(fixturesDir.resolve("1008.3918v2.pdf"));
        long originalPages;
        try (PdfDocument original = PdfDocument.open(pdfBytes)) {
            originalPages = original.pageCount();
        }
        ConversionResult result = PdfAConverter.convert(pdfBytes, PdfALevel.A_2B);
        try (PdfDocument converted = PdfDocument.open(result.convertedBytes())) {
            assertThat(converted.pageCount()).isEqualTo(originalPages);
        }
    }
}
