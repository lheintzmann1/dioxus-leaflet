import type { L, Id, RustCallback } from "./types";
import { get_map } from "./map";
import { setup } from "./util";
import { get_popup } from "./popup";

const _markers = new Map<Id, L.Marker>();
const _callbacks = new Map<Id, (map: L.Marker) => void>();
const _promises = new Map<Id, Promise<L.Marker>>();

export async function get_marker(marker_id: Id): Promise<L.Marker> {
    let marker = _markers.get(marker_id);
    if (!marker) {
        let p = _promises.get(marker_id);
        if (!p) {
            p = new Promise<L.Marker>((resolve) => {
                _callbacks.set(marker_id, resolve);
            });
            _promises.set(marker_id, p);
        }
        marker = await p;
    }
    return marker;
}

export async function update_marker(map_id: Id, marker_id: Id, coordinate: L.LatLngExpression, icon?: L.IconOptions) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when updating marker ${marker_id}`);
    }

    const marker = _markers.get(marker_id) ?? l.marker([0, 0]).addTo(map);
    _markers.set(marker_id, marker);

    marker.setLatLng(coordinate);
    if (icon) {
        // Set non-null default values
        icon.popupAnchor ??= [0, 0];
        icon.tooltipAnchor ??= [0, 0];
        marker.setIcon(l.icon(icon));
    }

    const popup = get_popup(marker_id);
    if (popup) {
        marker.unbindPopup();
        marker.bindPopup(popup.body, popup.options);
    }

    // Resolve any pending promises
    if (_callbacks.has(marker_id)) {
        const callback = _callbacks.get(marker_id)!;
        callback(marker);
        _callbacks.delete(marker_id);
        _promises.delete(marker_id);
    }
}

export async function on_marker_click(marker_id: Id, callback: RustCallback<void, void>): Promise<void> {
    const l = await setup();
    const marker = await get_marker(marker_id);
    if (!marker) {
        throw new Error(`Marker with id ${marker_id} not found when setting onClick handler`);
    }
    marker.on("click", async (e: L.LeafletMouseEvent) => {
        try {
            await callback()
        } catch (error) {
            console.error("Error in on_marker_click callback:", error);
        }
    });
}

export async function delete_marker(map_id: Id, marker_id: Id) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when deleting marker ${marker_id}`);
    }

    const marker = await get_marker(marker_id);
    if (!marker) {
        throw new Error(`Marker with id ${marker_id} not found when deleting`);
    }

    map.removeLayer(marker);
    _markers.delete(marker_id);
}