const DFINITY_ORG: &str = "@dfinity";
const ICP_SDK_CORE: &str = "@icp-sdk/core";

pub fn update_icp_sdk_imports(content: &String) -> String {
    let updated_content = content
        .replace(
            &format!(
                "import type {{ Principal }} from '{}/principal';",
                DFINITY_ORG
            ),
            &format!(
                "import type {{ Principal }} from '{}/principal';",
                ICP_SDK_CORE
            ),
        )
        .replace(
            &format!(
                "import type {{ ActorMethod }} from '{}/agent';",
                DFINITY_ORG
            ),
            &format!(
                "import type {{ ActorMethod }} from '{}/agent';",
                ICP_SDK_CORE
            ),
        )
        .replace(
            &format!("import type {{ IDL }} from '{}/candid';", DFINITY_ORG),
            &format!("import type {{ IDL }} from '{}/candid';", ICP_SDK_CORE),
        );

    updated_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_imports_rewrites_all_known_imports() {
        let input = r#"
import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';
"#
        .to_string();

        let out = update_icp_sdk_imports(&input);

        assert!(
            !out.contains("@dfinity/"),
            "Leftover @dfinity import:\n{out}"
        );
        assert!(out.contains("from '@icp-sdk/core/principal'"));
        assert!(out.contains("from '@icp-sdk/core/agent'"));
        assert!(out.contains("from '@icp-sdk/core/candid'"));
    }

    #[test]
    fn update_imports_is_idempotent() {
        let already_rewritten = r#"
import type { Principal } from '@icp-sdk/core/principal';
import type { ActorMethod } from '@icp-sdk/core/agent';
import type { IDL } from '@icp-sdk/core/candid';
"#
        .to_string();

        let out = update_icp_sdk_imports(&already_rewritten);
        assert_eq!(out, already_rewritten);
    }
}
