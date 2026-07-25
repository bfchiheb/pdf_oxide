/*
 * Copyright 2025-2026 Yury Fedoseev and pdf_oxide contributors.
 * Licensed under MIT OR Apache-2.0.
 */
package fyi.oxide.pdf.conversion;

import fyi.oxide.pdf.compliance.PdfALevel;
import java.util.List;
import java.util.Objects;

public final class ConversionResult {
    private final boolean success;
    private final PdfALevel level;
    private final List<ConversionAction> actions;
    private final List<ConversionError> errors;
    private final byte[] convertedBytes;

    public ConversionResult(
            boolean success,
            PdfALevel level,
            List<ConversionAction> actions,
            List<ConversionError> errors,
            byte[] convertedBytes) {
        this.success = success;
        this.level = Objects.requireNonNull(level, "level");
        this.actions = List.copyOf(Objects.requireNonNull(actions, "actions"));
        this.errors = List.copyOf(Objects.requireNonNull(errors, "errors"));
        this.convertedBytes = Objects.requireNonNull(convertedBytes, "convertedBytes");
    }

    public boolean success() {
        return success;
    }

    public PdfALevel level() {
        return level;
    }

    public List<ConversionAction> actions() {
        return actions;
    }

    public List<ConversionError> errors() {
        return errors;
    }

    public byte[] convertedBytes() {
        return convertedBytes.clone();
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof ConversionResult)) return false;
        ConversionResult r = (ConversionResult) o;
        return success == r.success
                && level == r.level
                && actions.equals(r.actions)
                && errors.equals(r.errors)
                && java.util.Arrays.equals(convertedBytes, r.convertedBytes);
    }

    @Override
    public int hashCode() {
        int result = Objects.hash(success, level, actions, errors);
        result = 31 * result + java.util.Arrays.hashCode(convertedBytes);
        return result;
    }

    @Override
    public String toString() {
        return "ConversionResult[success=" + success
                + " level=" + level
                + " actions=" + actions.size()
                + " errors=" + errors.size()
                + " bytes=" + convertedBytes.length + "]";
    }
}
