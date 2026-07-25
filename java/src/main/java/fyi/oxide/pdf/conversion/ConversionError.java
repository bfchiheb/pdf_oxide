/*
 * Copyright 2025-2026 Yury Fedoseev and pdf_oxide contributors.
 * Licensed under MIT OR Apache-2.0.
 */
package fyi.oxide.pdf.conversion;

import java.util.Objects;

public final class ConversionError {
    private final String errorCode;
    private final String reason;

    public ConversionError(String errorCode, String reason) {
        this.errorCode = Objects.requireNonNull(errorCode, "errorCode");
        this.reason = Objects.requireNonNull(reason, "reason");
    }

    public String errorCode() {
        return errorCode;
    }

    public String reason() {
        return reason;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof ConversionError)) return false;
        ConversionError that = (ConversionError) o;
        return errorCode.equals(that.errorCode) && reason.equals(that.reason);
    }

    @Override
    public int hashCode() {
        return Objects.hash(errorCode, reason);
    }

    @Override
    public String toString() {
        return "ConversionError[errorCode=" + errorCode + " reason=" + reason + "]";
    }
}
