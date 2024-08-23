pub struct ArgcTag;

impl ArgcTag {
    pub fn get_describe(description: &str) -> String {
        Self::format("describe", "", description)
    }

    pub fn get_meta_version(version: &str) -> String {
        Self::format_meta("version", version)
    }

    pub fn get_meta_author(author: &str) -> String {
        Self::format_meta("author", author)
    }

    pub fn get_meta_dotenv(dotenv: &str) -> String {
        Self::format_meta("dotenv", dotenv)
    }

    pub fn get_meta_require_tools(require_tools: &[String]) -> String {
        Self::format_meta("require-tools", &require_tools.join(","))
    }

    pub fn get_meta_man_section(man_section: u8) -> String {
        Self::format_meta("man-section", &man_section.to_string())
    }

    pub fn get_meta_inherit_flag_options() -> String {
        Self::format_meta("inherit-flag-options", "")
    }

    pub fn get_meta_combine_shorts() -> String {
        Self::format_meta("combine-shorts", "")
    }

    pub fn get_meta_symbol(symbol: &str) -> String {
        Self::format_meta("symbol", symbol)
    }

    pub fn format_meta(name: &str, value: &str) -> String {
        Self::format("meta", name, value)
    }

    pub fn format(key: &str, name: &str, value: &str) -> String {
        let mut buf = vec![key, name, value];
        buf.retain(|&x| !x.is_empty());
        format!("# @{}", buf.join(" "))
    }
}
