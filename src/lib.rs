use jni::JNIEnv;
use jni::objects::{JClass, JString, JByteArray};
use jni::sys::{jlong, jstring, jbyteArray, jboolean, jint};
use std::time::Duration;
use yara_x::{Compiler, Rules, Scanner};

struct NativeCompiler {
    compiler: Compiler<'static>,
}

struct NativeRules {
    rules: Rules,
}

struct NativeScanner<'r> {
    scanner: Scanner<'r>,
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Compiler_create(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let compiler = Compiler::new();
    let native_compiler = Box::new(NativeCompiler { compiler });
    Box::into_raw(native_compiler) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Compiler_addSource(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    source: JString,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }

    let compiler_ptr = unsafe { &mut *(ptr as *mut NativeCompiler) };
    let source_str = match env.get_string(&source) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match compiler_ptr.compiler.add_source(source_str.to_str().unwrap_or("")) {
        Ok(_) => std::ptr::null_mut(),
        Err(e) => {
            let msg = format!("{:?}", e);
            match env.new_string(msg) {
                Ok(jstr) => jstr.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Compiler_createNamespace(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    namespace: JString,
) -> jlong {
    if ptr == 0 {
        return 0;
    }

    let compiler_ptr = unsafe { &mut *(ptr as *mut NativeCompiler) };
    let namespace_str = match env.get_string(&namespace) {
        Ok(s) => s,
        Err(_) => return 0,
    };

    compiler_ptr.compiler.new_namespace(namespace_str.to_str().unwrap_or(""));
    1
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Compiler_defineGlobal(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
    value: JString,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }

    let compiler_ptr = unsafe { &mut *(ptr as *mut NativeCompiler) };
    let name_str = match env.get_string(&name) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let value_str = match env.get_string(&value) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match compiler_ptr.compiler.define_global(name_str.to_str().unwrap_or(""), value_str.to_str().unwrap_or("")) {
        Ok(_) => std::ptr::null_mut(),
        Err(e) => {
            let msg = format!("{:?}", e);
            match env.new_string(msg) {
                Ok(jstr) => jstr.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Compiler_build(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr == 0 {
        return 0;
    }

    let compiler_ptr = unsafe { Box::from_raw(ptr as *mut NativeCompiler) };
    let rules = compiler_ptr.compiler.build();
    let native_rules = Box::new(NativeRules { rules });
    Box::into_raw(native_rules) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Compiler_destroy(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut NativeCompiler);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Rules_serialize(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jbyteArray {
    if ptr == 0 {
        return std::ptr::null_mut();
    }

    let rules_ptr = unsafe { &mut *(ptr as *mut NativeRules) };
    match rules_ptr.rules.serialize() {
        Ok(serialized) => {
            let output: Vec<i8> = serialized.iter().map(|&b| b as i8).collect();
            match env.new_byte_array(output.len() as i32) {
                Ok(java_array) => {
                    env.set_byte_array_region(&java_array, 0, &output).unwrap_or(());
                    java_array.into_raw()
                }
                Err(_) => std::ptr::null_mut(),
            }
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Rules_deserializeFromBytes(
    env: JNIEnv,
    _class: JClass,
    bytes: JByteArray,
) -> jlong {
    let bytes_len = env.get_array_length(&bytes).unwrap_or(0) as usize;
    if bytes_len == 0 {
        return 0;
    }

    let mut buffer = vec![0u8; bytes_len];
    let buffer_i8: &mut [i8] = unsafe { std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut i8, bytes_len) };
    env.get_byte_array_region(&bytes, 0, buffer_i8).unwrap_or(());

    match Rules::deserialize(&buffer) {
        Ok(rules) => {
            let native_rules = Box::new(NativeRules { rules });
            Box::into_raw(native_rules) as jlong
        }
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Rules_destroy(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut NativeRules);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Scanner_create(
    _env: JNIEnv,
    _class: JClass,
    rules_ptr: jlong,
) -> jlong {
    if rules_ptr == 0 {
        return 0;
    }

    let native_rules = unsafe { &*(rules_ptr as *mut NativeRules) };
    let scanner = Scanner::new(&native_rules.rules);
    let native_scanner = Box::new(NativeScanner { scanner });
    Box::into_raw(native_scanner) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Scanner_setTimeout(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    seconds: jint,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }

    let scanner_ptr = unsafe { &mut *(ptr as *mut NativeScanner) };
    let duration = Duration::from_secs(seconds as u64);
    scanner_ptr.scanner.set_timeout(duration);
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Scanner_setMaxMatchesPerPattern(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max_matches: jint,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }

    let scanner_ptr = unsafe { &mut *(ptr as *mut NativeScanner) };
    scanner_ptr.scanner.max_matches_per_pattern(max_matches as usize);
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Scanner_setGlobal(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    name: JString,
    value: JString,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }

    let scanner_ptr = unsafe { &mut *(ptr as *mut NativeScanner) };
    let name_str = match env.get_string(&name) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let value_str = match env.get_string(&value) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match scanner_ptr.scanner.set_global(name_str.to_str().unwrap_or(""), value_str.to_str().unwrap_or("")) {
        Ok(_) => std::ptr::null_mut(),
        Err(e) => {
            let msg = format!("{:?}", e);
            match env.new_string(msg) {
                Ok(jstr) => jstr.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Scanner_useMmap(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }

    let scanner_ptr = unsafe { &mut *(ptr as *mut NativeScanner) };
    scanner_ptr.scanner.use_mmap(enabled != 0);
    std::ptr::null_mut()
}

/// Escape a string for safe JSON embedding.
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => out.push(c),
        }
    }
    out
}

fn scan_results_to_json(results: &yara_x::ScanResults) -> String {
    let mut json = serde_json::Map::new();

    // Matching rules
    let mut matching_arr = Vec::new();
    for rule in results.matching_rules() {
        let mut rule_obj = serde_json::Map::new();
        rule_obj.insert("identifier".into(), serde_json::Value::String(rule.identifier().to_string()));
        rule_obj.insert("namespace".into(), serde_json::Value::String(rule.namespace().to_string()));

        // Tags
        let tags: Vec<serde_json::Value> = rule.tags().map(|t| serde_json::Value::String(t.identifier().to_string())).collect();
        rule_obj.insert("tags".into(), serde_json::Value::Array(tags));

        // Metadata
        let metadata = rule.metadata();
        if !metadata.is_empty() {
            let meta_json = metadata.into_json();
            rule_obj.insert("metadata".into(), meta_json);
        } else {
            rule_obj.insert("metadata".into(), serde_json::Value::Array(Vec::new()));
        }

        // Patterns
        let mut patterns_arr = Vec::new();
        for pattern in rule.patterns() {
            let mut pat_obj = serde_json::Map::new();
            pat_obj.insert("identifier".into(), serde_json::Value::String(pattern.identifier().to_string()));

            let mut matches_arr = Vec::new();
            for m in pattern.matches() {
                let mut match_obj = serde_json::Map::new();
                let range = m.range();
                match_obj.insert("offset".into(), serde_json::Value::Number(serde_json::Number::from(range.start)));
                match_obj.insert("length".into(), serde_json::Value::Number(serde_json::Number::from(range.end - range.start)));

                // Include matched data as hex for binary-safe output
                let data = m.data();
                let hex: String = data.iter().map(|b| format!("{:02x}", b)).collect();
                match_obj.insert("data".into(), serde_json::Value::String(hex));

                // Try to include as UTF-8 if valid
                if let Ok(s) = std::str::from_utf8(data) {
                    match_obj.insert("data_str".into(), serde_json::Value::String(s.to_string()));
                }

                if let Some(key) = m.xor_key() {
                    match_obj.insert("xor_key".into(), serde_json::Value::Number(serde_json::Number::from(key)));
                }

                matches_arr.push(serde_json::Value::Object(match_obj));
            }
            pat_obj.insert("matches".into(), serde_json::Value::Array(matches_arr));
            patterns_arr.push(serde_json::Value::Object(pat_obj));
        }
        rule_obj.insert("patterns".into(), serde_json::Value::Array(patterns_arr));

        matching_arr.push(serde_json::Value::Object(rule_obj));
    }
    json.insert("matches".into(), serde_json::Value::Array(matching_arr));

    // Non-matching rules
    let mut non_matching_arr = Vec::new();
    for rule in results.non_matching_rules() {
        let mut rule_obj = serde_json::Map::new();
        rule_obj.insert("identifier".into(), serde_json::Value::String(rule.identifier().to_string()));
        rule_obj.insert("namespace".into(), serde_json::Value::String(rule.namespace().to_string()));
        non_matching_arr.push(serde_json::Value::Object(rule_obj));
    }
    json.insert("nonMatching".into(), serde_json::Value::Array(non_matching_arr));

    serde_json::to_string(&serde_json::Value::Object(json)).unwrap_or_else(|_| "{}".to_string())
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Scanner_scanBytes(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    data: JByteArray,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }

    let data_len = env.get_array_length(&data).unwrap_or(0) as usize;
    if data_len == 0 {
        match env.new_string("{\"matches\":[],\"nonMatching\":[]}") {
            Ok(jstr) => return jstr.into_raw(),
            Err(_) => return std::ptr::null_mut(),
        }
    }

    let mut buffer = vec![0u8; data_len];
    let buffer_i8: &mut [i8] = unsafe { std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut i8, data_len) };
    env.get_byte_array_region(&data, 0, buffer_i8).unwrap_or(());

    let scanner_ptr = unsafe { &mut *(ptr as *mut NativeScanner) };
    match scanner_ptr.scanner.scan(&buffer) {
        Ok(results) => {
            let json = scan_results_to_json(&results);
            match env.new_string(json) {
                Ok(jstr) => jstr.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
        Err(e) => {
            let json = format!("{{\"error\":\"{}\"}}", json_escape(&format!("{:?}", e)));
            match env.new_string(json) {
                Ok(jstr) => jstr.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Scanner_scanFile(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    path: JString,
) -> jstring {
    if ptr == 0 {
        return std::ptr::null_mut();
    }

    let path_str = match env.get_string(&path) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let scanner_ptr = unsafe { &mut *(ptr as *mut NativeScanner) };
    match scanner_ptr.scanner.scan_file(path_str.to_str().unwrap_or("")) {
        Ok(results) => {
            let json = scan_results_to_json(&results);
            match env.new_string(json) {
                Ok(jstr) => jstr.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
        Err(e) => {
            let json = format!("{{\"error\":\"{}\"}}", json_escape(&format!("{:?}", e)));
            match env.new_string(json) {
                Ok(jstr) => jstr.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_revengi_abhi_yarax_Scanner_destroy(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut NativeScanner);
        }
    }
}
