use crate::*;
use serde_json::{Value, json};

pub struct WindowDisplayToggle;

impl WindowDisplayToggle {
    pub fn info() -> Value {
        json!({
            "name": "window_display_toggle",
            "description": "Set display options in Alias view windows to ON or OFF. Supports items like grid, model, pivots, guidelines, locators, construction objects, canvases, lights, textures, cameras, image planes, clouds, and non-proportional scaling. The tool will check the current state and only change it if necessary.",
            "examples": [
                {
                    "description": "Turn on grid display in the current view window",
                    "command": "alias_lic window_display_toggle --item kGrid --state true"
                },
                {
                    "description": "Turn off model display in the current view window",
                    "command": "alias_lic window_display_toggle --item kModel --state false"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "item": {
                        "type": "string",
                        "description": "The display item to set. Valid values: kModel, kModelOnly, kPivots, kGrid, kGuidelines, kLocators, kConstructionObjects, kCanvases, kLights, kTextures, kCameras, kImagePlanes, kClouds, kNonProportional",
                        "enum": ["kModel", "kModelOnly", "kPivots", "kGrid", "kGuidelines", "kLocators", "kConstructionObjects", "kCanvases", "kLights", "kTextures", "kCameras", "kImagePlanes", "kClouds", "kNonProportional"]
                    },
                    "state": {
                        "type": "boolean",
                        "description": "The desired state: true for ON, false for OFF"
                    }
                },
                "required": ["item", "state"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let item_str = args["item"].as_str().ok_or("Missing or invalid 'item' parameter")?;
        let desired_state = args["state"].as_bool().ok_or("Missing or invalid 'state' parameter")?;
        
        // Parse the item string to AlWindowToggle enum
        let item = match item_str {
            "kModel" => AlWindowToggle::kModel,
            "kModelOnly" => AlWindowToggle::kModelOnly,
            "kPivots" => AlWindowToggle::kPivots,
            "kGrid" => AlWindowToggle::kGrid,
            "kGuidelines" => AlWindowToggle::kGuidelines,
            "kLocators" => AlWindowToggle::kLocators,
            "kConstructionObjects" => AlWindowToggle::kConstructionObjects,
            "kCanvases" => AlWindowToggle::kCanvases,
            "kLights" => AlWindowToggle::kLights,
            "kTextures" => AlWindowToggle::kTextures,
            "kCameras" => AlWindowToggle::kCameras,
            "kImagePlanes" => AlWindowToggle::kImagePlanes,
            "kClouds" => AlWindowToggle::kClouds,
            "kNonProportional" => AlWindowToggle::kNonProportional,
            _ => return Err(format!("Invalid item: {}", item_str)),
        };

        // Get the current window
        let window = AlUniverse::current_window().ok_or("Failed to get current window")?;
        // First, check the current state by doing a toggle and then toggling back
        // Because we don't have a direct is_* method for each toggle item
        let current_state = window.window_toggle(item)?;
        // Only change state if necessary
        if current_state == desired_state {
            return Ok(json!({
                "jsonrpc": "2.0",
                "id": id_val,
                "result": {
                    "content": [{
                        "type": "text",
                        "text": format!("{} display is already {} - no change needed", item_str, if desired_state { "ON" } else { "OFF" })
                    }]
                }
            }));
        }

        // Set the desired state
        window.set_window_toggle(item, desired_state)?;

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Set {} display to {} (was {})", item_str, if desired_state { "ON" } else { "OFF" }, if current_state { "ON" } else { "OFF" })
                }]
            }
        }))
    }
}
