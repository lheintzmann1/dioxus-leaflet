window.DioxusLeaflet = class DioxusLeaflet {
    static _maps = {};

    static async initializeAsync(recv, send) {
        let options = await recv();
        let error = null;
        try {
            this._maps[options.map_id] = this.initialize(options);
        }
        catch (e) {
            console.error("Error initializing dioxus leaflet map:", e);
            error = e.toString();
        }
        finally {
            send(error);
        }
    }

    static initialize({ map_id, initial_position, markers, options }) {
        // Initialize the map with options
        var map = L.map(map_id, {
            zoomControl: options.zoom_control,
            scrollWheelZoom: options.scroll_wheel_zoom,
            doubleClickZoom: options.double_click_zoom,
            touchZoom: options.touch_zoom,
            dragging: options.dragging,
            keyboard: options.keyboard,
            attributionControl: options.attribution_control
        }).setView([initial_position.lat, initial_position.lng], initial_position.zoom);

        // Add tile layer
        L.tileLayer(options.tile_layer.url, {
            attribution: options.tile_layer.attribution,
            maxZoom: options.tile_layer.max_zoom,
            subdomains: options.tile_layer.subdomains
        }).addTo(map);

        // Add markers
        for (let markerData of markers) {
            var markerOptions = {};
            
            // Custom icon if provided
            if (markerData.icon) {
                var iconOptions = {
                    iconUrl: markerData.icon.icon_url
                };
                
                if (markerData.icon.icon_size) {
                    iconOptions.iconSize = markerData.icon.icon_size;
                }
                if (markerData.icon.icon_anchor) {
                    iconOptions.iconAnchor = markerData.icon.icon_anchor;
                }
                if (markerData.icon.popup_anchor) {
                    iconOptions.popupAnchor = markerData.icon.popup_anchor;
                }
                if (markerData.icon.shadow_url) {
                    iconOptions.shadowUrl = markerData.icon.shadow_url;
                }
                if (markerData.icon.shadow_size) {
                    iconOptions.shadowSize = markerData.icon.shadow_size;
                }
                
                markerOptions.icon = L.icon(iconOptions);
            }
            
            var marker = L.marker([markerData.lat, markerData.lng], markerOptions).addTo(map);
            
            // Add popup if title or description exists
            if (markerData.title || markerData.description) {
                var popupContent = '';
                if (markerData.title) {
                    popupContent += '<b>' + markerData.title + '</b>';
                }
                if (markerData.description) {
                    if (markerData.title) popupContent += '<br>';
                    popupContent += markerData.description;
                }
                
                var popupOptions = {};
                if (markerData.popup_options) {
                    Object.assign(popupOptions, markerData.popup_options);
                }
                
                marker.bindPopup(popupContent, popupOptions);
            }
        }
        
        // Force resize to ensure proper display
        setTimeout(function() {
            map.invalidateSize();
        }, 100);
        
        return map;
    }
}