window.DioxusLeaflet = class DioxusLeaflet {
    static _maps = new Map();
    static _markers = new Map();
    static _polygons = {};

    static async updateMapAsync(recv, send) {
        let options = await recv();
        let error = null;
        try {
            this.updateMap(options);
        }
        catch (e) {
            console.error("Error updating dioxus leaflet map:", e);
            error = e.toString();
        }
        finally {
            send(error);
        }
    }

    static updateMap({ map_id, initial_position, options }) {
        // Initialize the map with options
        const map = this._maps.get(map_id) ?? L.map(`dioxus-leaflet-${map_id}`, {
            zoomControl: options.zoom_control,
            scrollWheelZoom: options.scroll_wheel_zoom,
            doubleClickZoom: options.double_click_zoom,
            touchZoom: options.touch_zoom,
            dragging: options.dragging,
            keyboard: options.keyboard,
            attributionControl: options.attribution_control
        });
        this._maps.set(map_id, map);

        map.setView(initial_position.coordinates, initial_position.zoom);

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

    static async updateMarkerAsync(recv, send) {
        let options = await recv();
        let error = null;
        try {
            this.updateMarker(options);
        }
        catch (e) {
            console.error("Error updating dioxus leaflet map:", e);
            error = e.toString();
        }
        finally {
            send(error);
        }
    }

    static updateMarker({ map_id, marker_id, coordinate, icon }) {
        const map = this._maps.get(map_id);
        const markers = this._markers.get(map_id) ?? new Map();
        this._markers.set(map_id, markers);
        const marker = markers.get(marker_id) ?? L.marker([0, 0]).addTo(map);

        marker.setLatLng(coordinate);

        // Custom icon if provided
        if (icon) {
            marker.setIcon(L.icon(icon));
        }
        
        // Add popup if title or description exists
        // if (markerData.title || markerData.description) {
        //     var popupContent = '';
        //     if (markerData.title) {
        //         popupContent += '<b>' + markerData.title + '</b>';
        //     }
        //     if (markerData.description) {
        //         if (markerData.title) popupContent += '<br>';
        //         popupContent += markerData.description;
        //     }
            
        //     var popupOptions = {};
        //     if (markerData.popup_options) {
        //         Object.assign(popupOptions, markerData.popup_options);
        //     }
            
        //     marker.unbindPopup();
        //     marker.bindPopup(popupContent, popupOptions);
        // }
    }

    static updatePolygons({ map_id, polygons: data }) {
        const map = this._maps[map_id];
        const gons = this._polygons[map_id] ??= [];

        // add polygons
        for (let i = 0; i < data.length; i++) {
            let gonData = data[i];

            let gon = gons[i] ??= L.polygon(gonData.coordinates, gonData.path_options).addTo(map);

            // Add popup if title or description exists
            if (gonData.title || gonData.description) {
                var popupContent = '';
                if (gonData.title) {
                    popupContent += '<b>' + gonData.title + '</b>';
                }
                if (gonData.description) {
                    if (gonData.title) popupContent += '<br>';
                    popupContent += gonData.description;
                }
                
                var popupOptions = {};
                if (gonData.popup_options) {
                    Object.assign(popupOptions, gonData.popup_options);
                }
                
                gon.unbindPopup();
                gon.bindPopup(popupContent, popupOptions);
            }
        }

        // remove polygons
        for (let i = data.length; i < gons.length; i++) {
            gons[i].remove();
        }
        gons.length = data.length;
    }
}