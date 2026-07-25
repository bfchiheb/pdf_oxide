/*
 * Copyright 2025-2026 Yury Fedoseev and pdf_oxide contributors.
 * Licensed under MIT OR Apache-2.0.
 */
package fyi.oxide.pdf.conversion;

public enum ActionType {
    ADDED_XMP_METADATA,
    ADDED_PDFA_IDENTIFICATION,
    EMBEDDED_FONT,
    ADDED_OUTPUT_INTENT,
    REMOVED_JAVASCRIPT,
    REMOVED_ENCRYPTION,
    FLATTENED_TRANSPARENCY,
    REMOVED_EMBEDDED_FILES,
    ADDED_STRUCTURE,
    FIXED_ANNOTATION,
    ADDED_LANGUAGE
}
