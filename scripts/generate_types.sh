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

# Check if Java is available
if ! command -v java &> /dev/null; then
    echo "Error: Java not found. Please install Java 11 or higher."
    echo "  - On macOS with Homebrew: brew install openjdk@11"
    echo "  - Or download from: https://adoptium.net/"
    exit 1
fi

# Check Java version (need Java 11+)
JAVA_VERSION=$(java -version 2>&1 | awk -F '"' '/version/ {print $2}' | cut -d'.' -f1)
if [ "$JAVA_VERSION" -lt 11 ]; then
    echo "Error: Java 11 or higher is required, but found Java $JAVA_VERSION"
    echo "  - On macOS with Homebrew: brew install openjdk@11"
    echo "  - Or download from: https://adoptium.net/"
    echo ""
    echo "After installing, you may need to set JAVA_HOME:"
    echo "  export JAVA_HOME=\$(/usr/libexec/java_home -v 11)"
    exit 1
fi

echo "✓ Java $JAVA_VERSION detected"
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

# Create a deduplication tracker
TEMP_PROCESSED_TYPES="/tmp/rust_sdk_processed_types.txt"
: > "$TEMP_PROCESSED_TYPES"

# Sort files alphabetically to ensure consistent ordering and easier deduplication
find "$TEMP_DIR/src/models" -name "*.rs" ! -name "mod.rs" | sort | while IFS= read -r file; do
    BASENAME=$(basename "$file")
    
    # Extract type names (enums and structs) from the file
    TYPE_NAMES=$(grep -E "^pub (struct|enum) [A-Za-z]" "$file" | awk '{print $3}' | tr -d '{')
    
    # Check if any of these types have already been processed
    SKIP_FILE=false
    for TYPE_NAME in $TYPE_NAMES; do
        if grep -q "^$TYPE_NAME$" "$TEMP_PROCESSED_TYPES" 2>/dev/null; then
            echo "⚠ Skipping duplicate type: $TYPE_NAME in $BASENAME"
            SKIP_FILE=true
            break
        fi
    done
    
    # If file contains duplicates, skip it
    if [ "$SKIP_FILE" = true ]; then
        continue
    fi
    
    # Mark these types as processed
    for TYPE_NAME in $TYPE_NAMES; do
        echo "$TYPE_NAME" >> "$TEMP_PROCESSED_TYPES"
    done
    
    # Add the file content
    echo "// From: $BASENAME" >> "$TYPES_OUTPUT"
    # Filter out imports that are already at the top of the file
    grep -vE "^use serde::\{Deserialize, Serialize\};" "$file" | grep -vE "^use crate::models;" >> "$TYPES_OUTPUT"
    echo "" >> "$TYPES_OUTPUT"
done

# Clean up temp files
rm -f "$TEMP_PROCESSED_TYPES"

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
echo "Duplicates were automatically removed during generation."
echo "All other generated files have been cleaned up."
