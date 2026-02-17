use anyhow::Result;
use fancy_regex::Regex;
use serde_json::{Map, Value, json};
use std::collections::HashMap;

pub struct ArkenfoxExtractor {
    section_regex: Regex,
    subsection_regex: Regex,
    pref_regex: Regex,
    disabled_pref_regex: Regex,
}

impl ArkenfoxExtractor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            section_regex: Regex::new(r"^/\*\*\* \[SECTION (\d{4})\]: (.*?)( \*\*\*\/)?$")?,
            subsection_regex: Regex::new(r"^/\* (\d{4}): (.*?)( \*\*\*\/)?$")?,
            pref_regex: Regex::new(r#"^\s*user_pref\("(.*?)", (.*?)\);"#)?,
            disabled_pref_regex: Regex::new(r#"^\s.*// user_pref\("(.*?)", (.*?)\);"#)?,
        })
    }

    pub fn extract(&mut self, content: &str) -> Result<Value> {
        let mut result = Map::new();
        let mut current_section = String::from("0000");
        let mut current_section_title = String::from("TOPLEVEL");

        let mut current_subsection = String::new();
        let mut current_subsection_title = String::new();
        let mut current_subsection_settings: Vec<Value> = Vec::new();

        let mut in_section_description = false;
        let mut in_subsection = false;
        let mut viewed_sections: HashMap<String, ()> = HashMap::new();
        let mut viewed_subsections: HashMap<String, ()> = HashMap::new();

        let lines: Vec<&str> = content.lines().collect();

        // Initialize first section with meta
        let mut section_obj = Map::new();
        let mut meta = Map::new();
        meta.insert(
            "title".to_string(),
            Value::String(self.nix_sanitize(&current_section_title)),
        );
        section_obj.insert("meta".to_string(), Value::Object(meta));
        result.insert(current_section.clone(), Value::Object(section_obj));

        for line in lines {
            // Skip section description lines
            if in_section_description {
                if line.ends_with("***/") {
                    in_section_description = false;
                }
                continue;
            }

            // Check for preferences
            if let Ok(Some(captures)) = self.pref_regex.captures(line) {
                let pref_name = captures[1].to_string();
                let pref_value_str = captures[2].to_string();

                // Skip parrot preferences (they were only used for meta)
                if pref_name == "_user.js.parrot" {
                    continue;
                }

                if !in_subsection {
                    return Err(anyhow::anyhow!(
                        "Found preference \"{}\" outside subsection",
                        pref_name
                    ));
                }

                let pref_value = self.parse_value(&pref_value_str);
                current_subsection_settings.push(json!({
                    "name": pref_name,
                    "enabled": true,
                    "value": pref_value
                }));
                continue;
            }

            // Check for disabled preferences
            if let Ok(Some(captures)) = self.disabled_pref_regex.captures(line) {
                let pref_name = captures[1].to_string();
                let pref_value_str = captures[2].to_string();

                // Skip parrot preferences (they were only used for meta)
                if pref_name == "_user.js.parrot" {
                    continue;
                }

                if !in_subsection {
                    return Err(anyhow::anyhow!(
                        "Found preference \"{}\" outside subsection",
                        pref_name
                    ));
                }

                let pref_value = self.parse_value(&pref_value_str);
                current_subsection_settings.push(json!({
                    "name": pref_name,
                    "enabled": false,
                    "value": pref_value
                }));
                continue;
            }

            // Skip subsection metadata lines (we only care about title now)
            if in_subsection && line.trim_start().starts_with("* ") {
                continue;
            }

            // Check for subsection start
            if let Ok(Some(captures)) = self.subsection_regex.captures(line) {
                // Close previous subsection if exists
                if in_subsection {
                    self.close_subsection(
                        &mut result,
                        &current_section,
                        &current_subsection,
                        &current_subsection_title,
                        &current_subsection_settings,
                    );
                }

                let subsection_num = captures[1].to_string();
                let subsection_title = captures[2].to_string();

                current_subsection =
                    self.select_new_subsection(&subsection_num, &mut viewed_subsections);
                current_subsection_title = subsection_title;
                current_subsection_settings.clear();
                in_subsection = true;
                continue;
            }

            // Check for section start
            if let Ok(Some(captures)) = self.section_regex.captures(line) {
                let section_num = captures[1].to_string();
                let section_title = captures[2].to_string();

                // Stop if we reach section 9999
                if section_num == "9999" {
                    break;
                }

                // Close previous subsection if exists
                if in_subsection {
                    self.close_subsection(
                        &mut result,
                        &current_section,
                        &current_subsection,
                        &current_subsection_title,
                        &current_subsection_settings,
                    );
                    in_subsection = false;
                }

                // Start new section
                current_section = self.select_new_section(&section_num, &mut viewed_sections);
                current_section_title = section_title;

                in_section_description = !line.ends_with("***/");

                // Initialize new section with meta
                let mut section_obj = Map::new();
                let mut meta = Map::new();
                meta.insert(
                    "title".to_string(),
                    Value::String(self.nix_sanitize(&current_section_title)),
                );
                section_obj.insert("meta".to_string(), Value::Object(meta));
                result.insert(current_section.clone(), Value::Object(section_obj));

                // Handle special case for section 9000
                if section_num == "9000" {
                    in_subsection = true;
                    current_subsection =
                        self.select_new_subsection("9000", &mut viewed_subsections);
                    current_subsection_title = "PERSONAL".to_string();
                    current_subsection_settings.clear();
                }
                continue;
            }
        }

        // Close final subsection if exists
        if in_subsection {
            self.close_subsection(
                &mut result,
                &current_section,
                &current_subsection,
                &current_subsection_title,
                &current_subsection_settings,
            );
        }

        Ok(Value::Object(result))
    }

    fn nix_sanitize(&self, s: &str) -> String {
        s.trim()
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
    }

    fn select_new_section(&self, section: &str, viewed: &mut HashMap<String, ()>) -> String {
        self.select_new(section, viewed)
    }

    fn select_new_subsection(&self, subsection: &str, viewed: &mut HashMap<String, ()>) -> String {
        self.select_new(subsection, viewed)
    }

    fn select_new(&self, stub: &str, viewed: &mut HashMap<String, ()>) -> String {
        if !viewed.contains_key(stub) {
            viewed.insert(stub.to_string(), ());
            return stub.to_string();
        }

        let mut i = 1;
        loop {
            let attempt = format!("{}-{}", stub, i);
            if !viewed.contains_key(&attempt) {
                viewed.insert(attempt.clone(), ());
                return attempt;
            }
            i += 1;
        }
    }

    fn close_subsection(
        &self,
        result: &mut Map<String, Value>,
        section_key: &str,
        subsection_key: &str,
        subsection_title: &str,
        subsection_settings: &[Value],
    ) {
        if let Some(Value::Object(section)) = result.get_mut(section_key) {
            let mut meta = Map::new();
            meta.insert(
                "title".to_string(),
                Value::String(self.nix_sanitize(subsection_title)),
            );

            let subsection_obj = json!({
                "settings": subsection_settings,
                "meta": meta
            });

            section.insert(subsection_key.to_string(), subsection_obj);
        }
    }

    fn parse_value(&self, value_str: &str) -> Value {
        // Try to parse as JSON first
        if let Ok(value) = serde_json::from_str(value_str) {
            value
        } else {
            // If that fails, treat as string
            Value::String(value_str.to_string())
        }
    }
}
