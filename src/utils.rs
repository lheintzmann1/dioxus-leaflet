use crate::types::*;

/// Generates a unique map ID
pub fn generate_map_id() -> String {
    format!("dioxus_leaflet_map_{}", fastrand::u32(..))
}

/// Generates the JavaScript initialization script for the map
pub fn generate_map_script(
    map_id: &str,
    initial_position: &MapPosition,
    markers: &[MapMarker],
    options: &MapOptions,
) -> String {
    let markers_json = serde_json::to_string(markers).unwrap_or_else(|_| "[]".to_string());
    let escaped_markers = escape_json_for_js(&markers_json);
    
    format!(
        r#"
        (function() {{
            function initializeDioxusLeafletMap() {{
                function tryInitialize() {{
                    if (typeof L === 'undefined') {{
                        setTimeout(tryInitialize, 100);
                        return;
                    }}
                    
                    var mapElement = document.getElementById('{}');
                    if (!mapElement) {{
                        setTimeout(tryInitialize, 100);
                        return;
                    }}
                    
                    try {{
                        // Initialize the map with options
                        var map = L.map('{}', {{
                            zoomControl: {},
                            scrollWheelZoom: {},
                            doubleClickZoom: {},
                            touchZoom: {},
                            dragging: {},
                            keyboard: {},
                            attributionControl: {}
                        }}).setView([{}, {}], {});

                        // Add tile layer
                        L.tileLayer('{}', {{
                            attribution: '{}',
                            maxZoom: {},
                            subdomains: {}
                        }}).addTo(map);

                        // Add markers
                        var markersData = "{}";
                        var markers = JSON.parse(markersData);
                        
                        markers.forEach(function(markerData) {{
                            var markerOptions = {{}};
                            
                            // Custom icon if provided
                            if (markerData.icon) {{
                                var iconOptions = {{
                                    iconUrl: markerData.icon.icon_url
                                }};
                                
                                if (markerData.icon.icon_size) {{
                                    iconOptions.iconSize = markerData.icon.icon_size;
                                }}
                                if (markerData.icon.icon_anchor) {{
                                    iconOptions.iconAnchor = markerData.icon.icon_anchor;
                                }}
                                if (markerData.icon.popup_anchor) {{
                                    iconOptions.popupAnchor = markerData.icon.popup_anchor;
                                }}
                                if (markerData.icon.shadow_url) {{
                                    iconOptions.shadowUrl = markerData.icon.shadow_url;
                                }}
                                if (markerData.icon.shadow_size) {{
                                    iconOptions.shadowSize = markerData.icon.shadow_size;
                                }}
                                
                                markerOptions.icon = L.icon(iconOptions);
                            }}
                            
                            var marker = L.marker([markerData.lat, markerData.lng], markerOptions).addTo(map);
                            
                            // Add popup if title or description exists
                            if (markerData.title || markerData.description) {{
                                var popupContent = '';
                                if (markerData.title) {{
                                    popupContent += '<b>' + markerData.title + '</b>';
                                }}
                                if (markerData.description) {{
                                    if (markerData.title) popupContent += '<br>';
                                    popupContent += markerData.description;
                                }}
                                
                                var popupOptions = {{}};
                                if (markerData.popup_options) {{
                                    Object.assign(popupOptions, markerData.popup_options);
                                }}
                                
                                marker.bindPopup(popupContent, popupOptions);
                            }}
                        }});
                        
                        // Force resize to ensure proper display
                        setTimeout(function() {{
                            map.invalidateSize();
                        }}, 100);
                        
                        // Store map reference for potential future use
                        window.dioxusLeafletMaps = window.dioxusLeafletMaps || {{}};
                        window.dioxusLeafletMaps['{}'] = map;
                        
                    }} catch (error) {{
                        console.error('Error initializing Dioxus Leaflet map:', error);
                    }}
                }}
                
                tryInitialize();
            }}
            
            // Initialize when DOM is ready
            if (document.readyState === 'loading') {{
                document.addEventListener('DOMContentLoaded', initializeDioxusLeafletMap);
            }} else {{
                initializeDioxusLeafletMap();
            }}
        }})();
        "#,
        map_id,
        map_id,
        bool_to_js(options.zoom_control),
        bool_to_js(options.scroll_wheel_zoom),
        bool_to_js(options.double_click_zoom),
        bool_to_js(options.touch_zoom),
        bool_to_js(options.dragging),
        bool_to_js(options.keyboard),
        bool_to_js(options.attribution_control),
        initial_position.lat,
        initial_position.lng,
        initial_position.zoom,
        options.tile_layer.url,
        options.tile_layer.attribution,
        options.tile_layer.max_zoom,
        serde_json::to_string(&options.tile_layer.subdomains).unwrap_or_else(|_| "[]".to_string()),
        escaped_markers,
        map_id
    )
}

/// Escapes JSON string for safe inclusion in JavaScript
pub fn escape_json_for_js(json: &str) -> String {
    json.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Converts Rust bool to JavaScript boolean string
pub fn bool_to_js(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}
