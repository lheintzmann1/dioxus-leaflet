import type { L, Id, LatLngBounds } from "./types";
import { get_map } from "./map";
import { setup } from "./util";

const _overlays = new Map<Id, L.ImageOverlay>();

export async function update_imageoverlay(map_id: Id, imageoverlay_id: Id, image_url: string, bounds: LatLngBounds, options: L.ImageOverlayOptions) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when updating image overlay ${imageoverlay_id}`);
    }

    const latLngBounds = l.latLngBounds(bounds.southwest, bounds.northeast);
    let overlay = _overlays.get(imageoverlay_id);
    if (!overlay) {
        overlay = l.imageOverlay(image_url, latLngBounds, options).addTo(map);
        _overlays.set(imageoverlay_id, overlay);
        return;
    }

    // The overlay already exists: apply the changed properties in place.
    if (overlay.getElement()?.getAttribute("src") !== image_url) {
        overlay.setUrl(image_url);
    }
    if (!overlay.getBounds().equals(latLngBounds)) {
        overlay.setBounds(latLngBounds);
    }
    if (options.opacity !== undefined) {
        overlay.setOpacity(options.opacity);
    }
    if (options.zIndex !== undefined) {
        overlay.setZIndex(options.zIndex);
    }
    // Options that only apply at creation time (alt, crossOrigin, className, ...)
    // are not updated in place by Leaflet.
}

export async function delete_imageoverlay(map_id: Id, imageoverlay_id: Id) {
    await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when deleting image overlay ${imageoverlay_id}`);
    }

    const overlay = _overlays.get(imageoverlay_id);
    if (overlay) {
        overlay.remove();
        _overlays.delete(imageoverlay_id);
    }
}
