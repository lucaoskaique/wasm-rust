#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_wasm_package_generated() {
        // Test that wasm-pack build generated the expected files
        let pkg_dir = Path::new("pkg");

        assert!(
            pkg_dir.exists(),
            "pkg directory should exist after wasm-pack build"
        );

        // Check that key files were generated
        assert!(
            pkg_dir.join("package.json").exists(),
            "package.json should be generated"
        );
        assert!(
            pkg_dir.join("angel_api_client_wasm.js").exists(),
            "JS bindings should be generated"
        );
        assert!(
            pkg_dir.join("angel_api_client_wasm.d.ts").exists(),
            "TypeScript definitions should be generated"
        );
        assert!(
            pkg_dir.join("angel_api_client_wasm_bg.wasm").exists(),
            "WASM binary should be generated"
        );
    }

    #[test]
    fn test_typescript_definitions_contain_client() {
        // Test that our Client class is properly exported in TypeScript definitions
        let ts_file = Path::new("pkg/angel_api_client_wasm.d.ts");

        if ts_file.exists() {
            let content =
                fs::read_to_string(ts_file).expect("Should be able to read TypeScript definitions");

            assert!(
                content.contains("class Client"),
                "Client class should be exported"
            );
            assert!(
                content.contains("constructor(host_url: string)"),
                "Client constructor should be typed"
            );
            assert!(
                content.contains("login(login_data: any): Promise<string>"),
                "Login method should be exported"
            );
        }
    }

    #[test]
    fn test_package_json_valid() {
        // Test that package.json was generated correctly
        let package_file = Path::new("pkg/package.json");

        if package_file.exists() {
            let content =
                fs::read_to_string(package_file).expect("Should be able to read package.json");

            // Parse as JSON to ensure it's valid
            let _: serde_json::Value =
                serde_json::from_str(&content).expect("package.json should be valid JSON");

            assert!(
                content.contains("angel-api-client-wasm"),
                "Package name should be correct"
            );
        }
    }
}
