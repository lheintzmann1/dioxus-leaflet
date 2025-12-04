import type { L, Id, MapOptions, MapPosition, RustCallback } from "./types";
import { setup, wait } from "./util";

const _maps = new Map<Id, L.Map>();
const _callbacks = new Map<Id, RustCallback<number[], void>>();

export function get_map(map_id: Id): L.Map | undefined {
    return _maps.get(map_id);
}

export async function update_map(map_id: Id, initial_position: MapPosition, options: MapOptions): Promise<void> {
    const l = await setup();

    // Initialize the map with options
    const map = _maps.get(map_id) ?? l.map(`dioxus-leaflet-map-${map_id}`, {
        zoomControl: options.zoom_control,
        scrollWheelZoom: options.scroll_wheel_zoom,
        doubleClickZoom: options.double_click_zoom,
        touchZoom: options.touch_zoom,
        dragging: options.dragging,
        keyboard: options.keyboard,
        attributionControl: options.attribution_control
    });
    _maps.set(map_id, map);

    map.setView(initial_position.coordinates, initial_position.zoom);

    // Add tile layer
    l.tileLayer(options.tile_layer.url, {
        attribution: options.tile_layer.attribution,
        maxZoom: options.tile_layer.max_zoom,
        subdomains: options.tile_layer.subdomains
    }).addTo(map);

    // Force resize to ensure proper display
    await wait(100);
    map.invalidateSize();
}

export function delete_map(map_id: Id) {
    _maps.delete(map_id);
    _callbacks.delete(map_id);
}

export async function on_map_click(map_id: Id, callback: RustCallback<number[], void>): Promise<void> {
    await setup();
    const map = get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when setting onClick handler`);
    }

    _callbacks.set(map_id, callback);
    map.on("click", async (e: L.LeafletMouseEvent) => {
        const cb = _callbacks.get(map_id);
        if (cb) {
            console.log("js-pre");
            await cb([e.latlng.lat, e.latlng.lng]);
            console.log("js-post");
        }
    });
}
