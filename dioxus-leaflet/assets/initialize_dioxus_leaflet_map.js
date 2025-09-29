window.DioxusLeaflet = class DioxusLeaflet {
    static _maps = {};
    static _markers = {};

    static async updateAsync(recv, send) {
        let options = await recv();
        let error = null;
        try {
            this.initialize(options);
            this.updateMarkers(options);
        }
        catch (e) {
            console.error("Error initializing dioxus leaflet map:", e);
            error = e.toString();
        }
        finally {
            send(error);
        }
    }

    static initialize({ map_id, initial_position, options }) {
        if (this._maps[map_id]) {
            return this._maps[map_id];
        }

        // Initialize the map with options
        const map = this._maps[map_id] = L.map(map_id, {
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

        // Force resize to ensure proper display
        setTimeout(function() {
            map.invalidateSize();
        }, 100);
    }

    static updateMarkers({ map_id, markers: data }) {
        const map = this._maps[map_id];
        const markers = this._markers[map_id] ??= [];

        // Add markers
        for (let i = 0; i < data.length; i++) {
            const markerData = data[i];
            let marker = markers[i] ??= L.marker([0, 0]).addTo(map);
            marker.setLatLng([markerData.lat, markerData.lng]);

            // Custom icon if provided
            if (markerData.icon) {
                marker.setIcon(L.icon(markerData.icon));
            }
            
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
                
                marker.unbindPopup();
                marker.bindPopup(popupContent, popupOptions);
            }
        }

        for (let i = data.length; i < markers.length; i++) {
            markers[i].remove();
        }
        markers.length = data.length;
    }
}