#!/bin/bash
set -e

# Ensure we are in the sdk root or handle paths correctly
# This script assumes it's run from the cli/rust-sdk directory or we can find the schema relative to it.

SCHEMA_PATH="../../apps/docs/public/api-reference.json"
TEMP_DIR=".openapi-temp"
TYPES_OUTPUT="src/types.rs"

echo "Generating Rust types from ${SCHEMA_PATH}..."

# check if schema exists
if [ ! -f "$SCHEMA_PATH" ]; then
    echo "Error: Schema file not found at $SCHEMA_PATH"
    exit 1
fi

# Check if pnpm is available
if ! command -v pnpm &> /dev/null; then
    echo "Error: pnpm not found. Please install Node.js and pnpm."
    exit 1
fi

echo "Note: This requires Java to be installed."
echo ""

# Clean up temp directory if it exists
rm -rf "$TEMP_DIR"

# Generate Rust client using openapi-generator-cli to temp directory
pnpm dlx @openapitools/openapi-generator-cli generate \
  -i "$SCHEMA_PATH" \
  -g rust \
  -o "$TEMP_DIR" \
  --additional-properties=packageName=unsent,packageVersion=1.0.1

# Extract only the model types and combine into a single file
echo "// Auto-generated types from OpenAPI specification" > "$TYPES_OUTPUT"
echo "// Do not edit manually" >> "$TYPES_OUTPUT"
echo "" >> "$TYPES_OUTPUT"
echo "use serde::{Deserialize, Serialize};" >> "$TYPES_OUTPUT"
echo "" >> "$TYPES_OUTPUT"

# Concatenate all model files (excluding mod.rs)
# Concatenate all model files (excluding mod.rs)
find "$TEMP_DIR/src/models" -name "*.rs" ! -name "mod.rs" -print0 | while IFS= read -r -d '' file; do
    echo "// From: $(basename "$file")" >> "$TYPES_OUTPUT"
    # Filter out imports that are already at the top of the file
    grep -vE "^use serde::\{Deserialize, Serialize\};" "$file" | grep -vE "^use crate::models;" >> "$TYPES_OUTPUT"
    echo "" >> "$TYPES_OUTPUT"
done

# Post-processing: Remove 'V1' prefix from type names and fix module references
if command -v perl &> /dev/null; then
    perl -pi -e 's/\bV1([A-Z])/$1/g' "$TYPES_OUTPUT"
    perl -pi -e 's/models:://g' "$TYPES_OUTPUT"
    perl -pi -e 's/use crate::models;//g' "$TYPES_OUTPUT"
    
    # Fix AnyOf types that generated empty structs
    perl -pi -e 's/to: Box<SendEmailRequestTo>/to: serde_json::Value/g' "$TYPES_OUTPUT"
    perl -pi -e 's/reply_to: Option<Box<SendEmailRequestTo>>/reply_to: Option<serde_json::Value>/g' "$TYPES_OUTPUT"
    perl -pi -e 's/cc: Option<Box<SendEmailRequestTo>>/cc: Option<serde_json::Value>/g' "$TYPES_OUTPUT"
    perl -pi -e 's/bcc: Option<Box<SendEmailRequestTo>>/bcc: Option<serde_json::Value>/g' "$TYPES_OUTPUT"
    perl -pi -e 's/to: SendEmailRequestTo/to: serde_json::Value/g' "$TYPES_OUTPUT"
    
    perl -pi -e 's/reply_to: Option<Box<CreateCampaignRequestReplyTo>>/reply_to: Option<serde_json::Value>/g' "$TYPES_OUTPUT"
    perl -pi -e 's/cc: Option<Box<CreateCampaignRequestReplyTo>>/cc: Option<serde_json::Value>/g' "$TYPES_OUTPUT"
    perl -pi -e 's/bcc: Option<Box<CreateCampaignRequestReplyTo>>/bcc: Option<serde_json::Value>/g' "$TYPES_OUTPUT"
    
    # Fix constructor
    perl -pi -e 's/to: Box::new\(to\)/to: to/g' "$TYPES_OUTPUT"
else
    sed -i '' 's/V1\([A-Z]\)/\1/g' "$TYPES_OUTPUT" 2>/dev/null || sed -i 's/V1\([A-Z]\)/\1/g' "$TYPES_OUTPUT"
    sed -i '' 's/models:://g' "$TYPES_OUTPUT" 2>/dev/null || sed -i 's/models:://g' "$TYPES_OUTPUT"
    sed -i '' 's/use crate::models;//g' "$TYPES_OUTPUT" 2>/dev/null || sed -i 's/use crate::models;//g' "$TYPES_OUTPUT"
    
    # Sed is harder for these complex replacements, deferring to perl which is available on mac
fi

# Clean up generated files and temp directory
rm -rf "$TEMP_DIR"
rm -rf "src/generated"

echo ""
echo "✓ Types generated at ${TYPES_OUTPUT}"
echo ""
echo "Generated a single file containing all Rust type definitions."
echo "All other generated files have been cleaned up."
