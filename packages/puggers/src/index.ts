import { loadNativeBinding } from "./native.js";

export type CollapseSingleNestedMode =
  | "off"
  | "top-wins"
  | "bottom-wins"
  | "best-tag-wins";

export type TextWhitespaceMode = "collapse" | "preserve";

export type QuoteStyle = "double" | "single";

export interface ConvertHtmlToPugOptions {
  allowedAttributes?: readonly string[];
  preserveIdAndClassShorthand?: boolean;
  root?: string;
  collapseSingleNested?: CollapseSingleNestedMode;
  textWhitespace?: TextWhitespaceMode;
  keepComments?: boolean;
  indentWidth?: number;
  lineWidth?: number | null;
  useTabs?: boolean;
  quoteStyle?: QuoteStyle;
}

export function convertHtmlToPug(
  input: string,
  options?: ConvertHtmlToPugOptions
): string {
  if (typeof input !== "string") {
    throw new TypeError("convertHtmlToPug input must be a string");
  }

  return loadNativeBinding().convertHtmlToPugNative(
    input,
    options == null ? undefined : JSON.stringify(options)
  );
}
