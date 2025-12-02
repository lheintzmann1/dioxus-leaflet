import { MapId, MarkerId, PopupOptions } from "./types";
import { get_marker } from "./marker";

type PopupRecord = {
    body: HTMLElement,
    options: PopupOptions,
};

const _popups = new Map<number, PopupRecord>();

export async function update_popup(id: number, body_id: number, options: PopupOptions) {
    const body = document.getElementById(`dioxus-leaflet-popup-${body_id}`);
    if (!body) {
        throw new Error(`Popup body element with id dioxus-leaflet-popup-${body_id} not found when updating popup for object ${id}`);
    }
    _popups.set(id, { body, options });

    let marker = get_marker(id);
    if (marker) {
        marker.unbindPopup();
        marker.bindPopup(body, {
            maxWidth: options.max_width,
            minWidth: options.min_width,
            maxHeight: options.max_height,
            autoPan: options.auto_pan,
            keepInView: options.keep_in_view,
            closeButton: options.close_button,
            autoClose: options.auto_close,
            closeOnEscapeKey: options.close_on_escape_key,
            className: options.class_name,
        });
    }
}