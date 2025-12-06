import type { L, Id, MapOptions, MapPosition, RustCallback, Json } from "./types";
import { setup, wait } from "./util";

const _maps = new Map<Id, L.Map>();

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
}

export async function on_map_click(map_id: Id, callback: RustCallback<number[], void>): Promise<void> {
    await setup();
    const map = get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when setting onClick handler`);
    }

    map.on("click", async (e: L.LeafletMouseEvent) => {
        try {
            await callback([e.latlng.lat, e.latlng.lng]);
        } catch (error) {
            console.error("Error in on_map_click callback:", error);
        }
    });

    // once this method returns, the callback is no longer valid
    // TODO: unregister?
    await new Promise<void>(() => {});
}
