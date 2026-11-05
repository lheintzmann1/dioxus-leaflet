import type { L, Id, LatLngBounds } from "./types";
import { get_map } from "./map";
import { setup } from "./util";

const _overlays = new Map<Id, L.ImageOverlay>();

export async function update_imageoverlay(map_id: Id, imageoverlay_id: Id, image_url: string, bounds: LatLngBounds, options: L.ImageOverlayOptions) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when updating marker ${imageoverlay_id}`);
    }

    const overlay = _overlays.get(imageoverlay_id) ?? l.imageOverlay(image_url, l.latLngBounds(bounds.southwest, bounds.northeast), options).addTo(map);
    _overlays.set(imageoverlay_id, overlay);
}

export async function delete_imageoverlay(map_id: Id, imageoverlay_id: Id) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when deleting marker ${imageoverlay_id}`);
    }

    const overlay = _overlays.get(imageoverlay_id);
    if (overlay) {
        overlay.remove();
        _overlays.delete(imageoverlay_id);
    }
}