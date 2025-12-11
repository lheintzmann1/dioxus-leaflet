import type { L, Id } from "./types";
import { get_map } from "./map";
import { setup } from "./util";
import { get_popup } from "./popup";

const _markers = new Map<Id, L.Marker>();

export function get_marker(marker_id: Id): L.Marker | undefined {
    return _markers.get(marker_id);
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
        marker.setIcon(l.icon(icon));
    }

    const popup = get_popup(marker_id);
    if (popup) {
        marker.unbindPopup();
        marker.bindPopup(popup.body, popup.options);
    }
}

export async function delete_marker(map_id: Id, marker_id: Id) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when deleting marker ${marker_id}`);
    }

    const marker = _markers.get(marker_id);
    if (!marker) {
        throw new Error(`Marker with id ${marker_id} not found when deleting`);
    }

    map.removeLayer(marker);
    _markers.delete(marker_id);
}