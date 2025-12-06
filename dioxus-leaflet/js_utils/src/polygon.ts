import { setup } from "./util";
import { get_map } from "./map";
import type { L, Id, Json } from "./types";
import { get_popup } from "./popup";

const _gons = new Map<Id, L.Polygon>();

export function get_polygon(polygon_id: Id): L.Polygon | undefined {
    return _gons.get(polygon_id);
}

export async function update_polygon(map_id: Id, polygon_id: Id, coordinates: L.LatLngLiteral[][][], options: L.PathOptions) {
    const l = await setup();
    const map = get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when updating polygon ${polygon_id}`);
    }

    const gon = _gons.get(polygon_id) ?? l.polygon([]).addTo(map);
    _gons.set(polygon_id, gon);

    gon.setLatLngs(coordinates);
    gon.setStyle(options);

    const popup = get_popup(polygon_id);
    if (popup) {
        gon.unbindPopup();
        gon.bindPopup(popup.body, popup.options);
    }
}

export async function delete_polygon(map_id: Id, polygon_id: Id) {
    const l = await setup();
    const map = get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when deleting polygon ${polygon_id}`);
    }

    _gons.delete(polygon_id);
}