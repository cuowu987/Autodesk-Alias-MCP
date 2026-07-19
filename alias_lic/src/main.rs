use reqwest::blocking::Client;
use serde_json::json;
use std::collections::HashMap;

fn print_tool_help(tool: &serde_json::Value) {
    let name = tool.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
    let description = tool.get("description").and_then(|d| d.as_str()).unwrap_or("");

    println!("{}", name);
    println!("{}", "=".repeat(name.len()));
    println!();
    println!("Description:");
    println!("  {}", description);
    println!();

    if let Some(input_schema) = tool.get("inputSchema") {
        if let Some(properties) = input_schema.get("properties") {
            let required: Vec<&str> = input_schema
                .get("required")
                .and_then(|r| r.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();

            println!("Parameters:");
            if let Some(props) = properties.as_object() {
                let mut param_order: Vec<&str> = Vec::new();
                for name in &required {
                    if props.contains_key(*name) {
                        param_order.push(name);
                    }
                }
                for (name, _) in props {
                    if !required.contains(&name.as_str()) {
                        param_order.push(name);
                    }
                }
                
                for param_name in param_order {
                    let param_info = props.get(param_name).unwrap();
                    let param_type = param_info.get("type").and_then(|t| t.as_str()).unwrap_or("unknown");
                    let param_desc = param_info.get("description").and_then(|d| d.as_str()).unwrap_or("");
                    let is_required = required.contains(&param_name);
                    let req_mark = if is_required { "(required)" } else { "(optional)" };
                    println!("  --{} <{}> {}", param_name, param_type, req_mark);
                    println!("     {}", param_desc);
                }
            }
            println!();
        }
    }

    println!("Examples:");
    print_example(tool);
    println!();
}

fn print_example(tool: &serde_json::Value) {
    // 优先尝试从 MCP 工具定义的 examples 字段读取
    if let Some(examples) = tool.get("examples").and_then(|e| e.as_array()) {
        for (i, example) in examples.iter().enumerate() {
            let desc = example.get("description").and_then(|d| d.as_str()).unwrap_or("");
            let cmd = example.get("command").and_then(|c| c.as_str()).unwrap_or("");
            if !desc.is_empty() {
                println!("  # {}", desc);
            }
            if !cmd.is_empty() {
                println!("  {}", cmd);
            }
            if i < examples.len() - 1 {
                println!();
            }
        }
        return;
    }
    
    //  fallback 到简单的默认示例
    let name = tool.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
    
    let mut args1: Vec<String> = Vec::new();
    let mut args2: Vec<String> = Vec::new();
    
    if let Some(input_schema) = tool.get("inputSchema") {
        if let Some(properties) = input_schema.get("properties") {
            let required: Vec<&str> = input_schema
                .get("required")
                .and_then(|r| r.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();

            if let Some(props) = properties.as_object() {
                for param_name in &required {
                    let param_info = props.get(*param_name);
                    let param_type = param_info
                        .and_then(|p| p.get("type"))
                        .and_then(|t| t.as_str())
                        .unwrap_or("string");

                    let example_value1 = match param_type {
                        "number" => "10",
                        "boolean" => "true",
                        "array" => "obj1 obj2",
                        _ => "value1",
                    };
                    let example_value2 = match param_type {
                        "number" => "20",
                        "boolean" => "false",
                        "array" => "obj3 obj4",
                        _ => "value2",
                    };
                    args1.push(format!("--{} {}", param_name, example_value1));
                    args2.push(format!("--{} {}", param_name, example_value2));
                }
            }
        }
    }

    if !args1.is_empty() {
        print!("  # Using example 1");
        println!();
        print!("  alias_lic {}", name);
        for arg in &args1 {
            print!(" {}", arg);
        }
        println!();
        println!();
        print!("  # Using example 2");
        println!();
        print!("  alias_lic {}", name);
        for arg in &args2 {
            print!(" {}", arg);
        }
        println!();
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  alias_lic [--server <url>] <command> [--param <value> ...]");
    println!();
    println!("Commands:");
    println!("  list                                 - Get available tools from MCP server");
    println!("  help <tool_name>                     - Show detailed help for a tool");
    println!();
    println!("Options:");
    println!("  --server <url>                       - MCP server address (default: http://127.0.0.1:9000/mcp)");
}

fn get_tools_from_server(server: &str) -> Result<serde_json::Value, String> {
    let client = Client::new();
    let request_json = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list"
    });

    let response = client
        .post(server)
        .header("Content-Type", "application/json")
        .json(&request_json)
        .send()
        .map_err(|e| format!("Failed to connect to MCP server: {}", e))?;

    let response_text = response.text().map_err(|e| format!("Failed to read response: {}", e))?;
    let json_str = response_text.strip_prefix("data: ").unwrap_or(&response_text);
    serde_json::from_str(json_str).map_err(|e| format!("Failed to parse response: {}", e))
}

fn parse_named_args(args: &[String]) -> HashMap<String, String> {
    let mut cmd_args = HashMap::new();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with("--") {
            let param_name = arg.strip_prefix("--").unwrap();
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                cmd_args.insert(param_name.to_string(), args[i + 1].clone());
                i += 2;
            } else {
                // 可能是不带值的标志，设为 true
                cmd_args.insert(param_name.to_string(), "true".to_string());
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    cmd_args
}

fn parse_args(args: &[String]) -> Result<(String, String, HashMap<String, serde_json::Value>), String> {
    let mut args = args.iter().skip(1).peekable();
    let mut server = "http://127.0.0.1:9000/mcp".to_string();
    let mut command = String::new();
    let mut remaining_args: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        if arg == "--server" {
            server = args.next().ok_or("Missing server URL")?.clone();
        } else if command.is_empty() {
            command = arg.clone();
        } else {
            remaining_args.push(arg.clone());
        }
    }

    if command.is_empty() {
        return Err("No command specified".to_string());
    }

    if command == "list" {
        match get_tools_from_server(&server) {
            Ok(response_json) => {
                if let Some(result) = response_json.get("result") {
                    if let Some(tools) = result.get("tools").and_then(|t| t.as_array()) {
                        let mut tools_vec: Vec<&serde_json::Value> = tools.iter().collect();
                        tools_vec.sort_by(|a, b| {
                            let name_a = a.get("name").and_then(|n| n.as_str()).unwrap_or("");
                            let name_b = b.get("name").and_then(|n| n.as_str()).unwrap_or("");
                            name_a.cmp(name_b)
                        });
                        println!("Available tools:");
                        println!("=================");
                        for (i, tool) in tools_vec.iter().enumerate() {
                            let name = tool.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
                            let description = tool.get("description").and_then(|d| d.as_str()).unwrap_or("");
                            println!("{}. {} - {}", i + 1, name, description);
                        }
                        std::process::exit(0);
                    }
                }
                println!("{}", response_json);
            }
            Err(e) => println!("Error: {}", e),
        }
        std::process::exit(0);
    }

    if command == "help" {
        let tool_name = remaining_args.first().cloned().unwrap_or_default();
        if tool_name.is_empty() {
            println!("Usage: alias_lic help <tool_name>");
            println!("Example: alias_lic help object_create_point");
            std::process::exit(1);
        }

        match get_tools_from_server(&server) {
            Ok(response_json) => {
                if let Some(result) = response_json.get("result") {
                    if let Some(tools) = result.get("tools").and_then(|t| t.as_array()) {
                        let tool = tools.iter().find(|t| {
                            t.get("name").and_then(|n| n.as_str()) == Some(&tool_name)
                        });

                        if let Some(tool) = tool {
                            print_tool_help(tool);
                        } else {
                            println!("Tool '{}' not found. Use 'alias_lic list' to see available tools.", tool_name);
                        }
                        std::process::exit(0);
                    }
                }
                println!("{}", response_json);
            }
            Err(e) => println!("Error: {}", e),
        }
        std::process::exit(0);
    }

    let response_json = get_tools_from_server(&server)?;
    let tools = response_json
        .get("result")
        .and_then(|r| r.get("tools"))
        .and_then(|t| t.as_array())
        .ok_or("Failed to get tools from server")?;

    let tool = tools.iter().find(|t| {
        t.get("name").and_then(|n| n.as_str()) == Some(&command)
    }).or_else(|| {
        let mapped_name = if command == "square_surface_create" {
            "squareSurface_create"
        } else {
            return None;
        };
        tools.iter().find(|t| {
            t.get("name").and_then(|n| n.as_str()) == Some(mapped_name)
        })
    }).ok_or_else(|| format!("Tool '{}' not found. Use 'alias_lic list' to see available tools.", command))?;

    let tool_name = tool.get("name").and_then(|n| n.as_str()).unwrap_or(&command).to_string();

    let input_schema = tool.get("inputSchema").ok_or("Tool has no arguments schema")?;
    let properties = input_schema.get("properties").and_then(|p| p.as_object()).ok_or("Tool arguments has no properties")?;
    
    let required: Vec<&str> = input_schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();

    // 解析命名参数 --param value
    let named_args = parse_named_args(&remaining_args);
    
    let mut cmd_args_map = HashMap::new();

    // 根据 inputSchema 转换类型
    for (param_name, param_str_value) in named_args {
        if let Some(param_info) = properties.get(&param_name) {
            let param_type = param_info.get("type").and_then(|t| t.as_str()).unwrap_or("string");
            let value = match param_type {
                "number" => {
                    let num: f64 = param_str_value.parse().map_err(|_| format!("Invalid number value for '{}'", param_name))?;
                    json!(num)
                }
                "integer" => {
                    let num: i64 = param_str_value.parse().map_err(|_| format!("Invalid integer value for '{}'", param_name))?;
                    json!(num)
                }
                "boolean" => {
                    let b: bool = param_str_value.parse().map_err(|_| format!("Invalid boolean value for '{}'", param_name))?;
                    json!(b)
                }
                "string" => json!(param_str_value),
                "array" => {
                    let arr: Vec<String> = param_str_value.split(' ').map(|s| s.to_string()).collect();
                    json!(arr)
                }
                _ => json!(param_str_value),
            };
            cmd_args_map.insert(param_name, value);
        }
    }

    // 检查 required 参数是否都存在
    for req in required {
        if !cmd_args_map.contains_key(req) {
            return Err(format!("Required parameter --{} is missing", req));
        }
    }

    Ok((server, tool_name, cmd_args_map))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let (server, tool_name, args_map) = parse_args(&args)?;

    let request_json = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": args_map
        }
    });

    let client = Client::new();
    let response = client
        .post(&server)
        .header("Content-Type", "application/json")
        .json(&request_json)
        .send()?;

    let response_text = response.text()?;
    let json_str = response_text.strip_prefix("data: ").unwrap_or(&response_text);

    if let Ok(response_json) = serde_json::from_str::<serde_json::Value>(json_str) {
        if let Some(result) = response_json.get("result") {
            if let Some(content) = result.get("content") {
                if let Some(text) = content.as_array().and_then(|arr| arr.first()).and_then(|obj| obj.get("text")).and_then(|t| t.as_str()) {
                    println!("{}", text);
                    return Ok(());
                }
            }
            if let Some(value) = result.get("value") {
                println!("{}", value);
                return Ok(());
            }
            println!("{}", result);
            return Ok(());
        }
    }

    println!("{}", response_text);
    Ok(())
}
