import { setup } from "./util";
import { get_map } from "./map";
import type { L, Id } from "./types";
import { get_popup } from "./popup";

const _lines = new Map<Id, L.Polyline>();

export function get_polyline(polyline_id: Id): L.Polyline | undefined {
    return _lines.get(polyline_id);
}

export async function update_polyline(map_id: Id, polyline_id: Id, coordinates: L.LatLngLiteral[][], options: L.PathOptions) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when updating polyline ${polyline_id}`);
    }

    const line = _lines.get(polyline_id) ?? l.polyline([]).addTo(map);
    _lines.set(polyline_id, line);

    line.setLatLngs(coordinates);
    line.setStyle(options);

    const popup = get_popup(polyline_id);
    if (popup) {
        line.unbindPopup();
        line.bindPopup(popup.body, popup.options);
    }
}

export async function delete_polyline(map_id: Id, polyline_id: Id) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when deleting polyline ${polyline_id}`);
    }

    const line = _lines.get(polyline_id);
    if (line) {
        line.remove();
        _lines.delete(polyline_id);
    }
}
