import { setup } from "./util";
import { get_map } from "./map";
import type { L, MapId, PolygonId, PolygonOptions } from "./types";

export async function update_polygon(map_id: MapId, polygon_id: number, coordinates: L.LatLngExpression[][], options: PolygonOptions) {
    const l = await setup();
    const map = get_map(map_id);
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