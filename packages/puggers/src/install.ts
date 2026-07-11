import { installNativeExecutable } from "./native.js";

try {
  installNativeExecutable();
} catch (error) {
  const message = error instanceof Error ? error.message : String(error);
  console.warn(`[puggers] Could not prepare native executable: ${message}`);
}
