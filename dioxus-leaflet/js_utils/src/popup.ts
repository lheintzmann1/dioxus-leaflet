import { L, Id } from "./types";
import { get_marker } from "./marker";
import { get_polygon } from "./polygon";
import { setup } from "./util";

type PopupRecord = {
    body: HTMLElement,
    options: L.PopupOptions,
};

const _popups = new Map<Id, PopupRecord>();

export function get_popup(marker_id: Id): PopupRecord | undefined {
    return _popups.get(marker_id);
}

export async function update_popup(marker_id: Id, popup_id: Id, options: L.PopupOptions) {
    const l = await setup();
    const id = `dioxus-leaflet-popup-${popup_id}`;
    const body = document.getElementById(id);
    if (!body) {
        throw new Error(`Popup body element with id ${id} not found when updating popup for object ${marker_id}`);
    }
    _popups.set(marker_id, { body, options });

    let context = get_marker(marker_id) ?? get_polygon(marker_id);
    if (context) {
        context.unbindPopup();
        context.bindPopup(body, options);
    }
}