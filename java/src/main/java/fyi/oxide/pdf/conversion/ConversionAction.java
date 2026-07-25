/*
 * Copyright 2025-2026 Yury Fedoseev and pdf_oxide contributors.
 * Licensed under MIT OR Apache-2.0.
 */
package fyi.oxide.pdf.conversion;

import java.util.Objects;

public final class ConversionAction {
    private final ActionType actionType;
    private final String description;
    private final int fixedErrorCode;

    public ConversionAction(ActionType actionType, String description, int fixedErrorCode) {
        this.actionType = Objects.requireNonNull(actionType, "actionType");
        this.description = Objects.requireNonNull(description, "description");
        this.fixedErrorCode = fixedErrorCode;
    }

    public ActionType actionType() {
        return actionType;
    }

    public String description() {
        return description;
    }

    public int fixedErrorCode() {
        return fixedErrorCode;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof ConversionAction)) return false;
        ConversionAction that = (ConversionAction) o;
        return fixedErrorCode == that.fixedErrorCode
                && actionType == that.actionType
                && description.equals(that.description);
    }

    @Override
    public int hashCode() {
        return Objects.hash(actionType, description, fixedErrorCode);
    }

    @Override
    public String toString() {
        return "ConversionAction[actionType=" + actionType
                + " description=" + description
                + " fixedErrorCode=" + fixedErrorCode + "]";
    }
}
