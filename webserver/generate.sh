SELF=$(pwd)
cd "../../3. Semester/smith/smith-codegen"
cargo run -- --export-type rust --output-file "$SELF/src/smith_types.rs" --schema-file "$SELF/web/schema.smith"
cargo run -- --export-type typescript --output-file "$SELF/web/smith_types.ts" --schema-file "$SELF/web/schema.smith"
cp -r "../smith-js/pkg" "$SELF/web/"
# cargo run ../../3.\Semester/smith/smith-codegen


