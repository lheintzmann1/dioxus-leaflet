window.DioxusLeaflet = class DioxusLeaflet {
    static _maps = new Map();
    static _objects = new Map();
    static _popups = new Map();

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

    static async deleteMap(recv) {
        let {map_id} = await recv();
        this._maps.delete(map_id);
    }

    static async deleteMarker(recv) {
        let {map_id, marker_id} = await recv();
        const marker = this._objects.get(marker_id);
        if (!marker) {
            return;
        }
        const map = this._maps.get(map_id);
        map.removeLayer(marker);
        this._objects.delete(marker_id);
    }

    static async registerOnClickHandlerMapAsync(recv, send) {
        let {map_id} = await recv();
        const map = this._maps.get(map_id);
        map.on('click', (e) => {
            send(e.latlng);
        })

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
            console.error("Error updating dioxus leaflet marker:", e);
            error = e.toString();
        }
        finally {
            send(error);
        }
    }

    static updateMarker({ map_id, marker_id, coordinate, icon }) {
        const map = this._maps.get(map_id);
        const marker = this._objects.get(marker_id) ?? L.marker([0, 0]).addTo(map);
        this._objects.set(marker_id, marker);

        marker.setLatLng(coordinate);
        if (icon) {
            marker.setIcon(L.icon(icon));
        }

        const popup = this._popups.get(marker_id);
        if (popup) {
            marker.unbindPopup();
            marker.bindPopup(popup.body, popup.options);
        }
    }

    static async updatePolygonAsync(recv, send) {
        let options = await recv();
        let error = null;
        try {
            this.updatePolygon(options);
        }
        catch (e) {
            console.error("Error updating dioxus leaflet polygon:", e);
            error = e.toString();
        }
        finally {
            send(error);
        }
    }

    static updatePolygon({ map_id, polygon_id, coordinates, options }) {
        const map = this._maps.get(map_id);
        const gon = this._objects.get(polygon_id) ?? L.polygon([]).addTo(map);
        this._objects.set(polygon_id, gon);

        gon.setLatLngs(coordinates);
        gon.setStyle(options);

        const popup = this._popups.get(polygon_id);
        if (popup) {
            gon.unbindPopup();
            gon.bindPopup(popup.body, popup.options);
        }
    }

    static async updatePopupAsync(recv, send) {
        let options = await recv();
        let error = null;
        try {
            this.updatePopup(options);
        }
        catch (e) {
            console.error("Error updating dioxus leaflet popup:", e);
            error = e.toString();
        }
        finally {
            send(error);
        }
    }

    static updatePopup({ marker_id, body_id, options }) {
        const body = document.getElementById(`dioxus-leaflet-popup-${body_id}`);
        this._popups.set(marker_id, { body, options });

        let marker = this._objects.get(marker_id);
        if (marker) {
            marker.unbindPopup();
            marker.bindPopup(body, options);
        }
    }
}