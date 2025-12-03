import { Id, PopupOptions } from "./types";
import { get_marker } from "./marker";

type PopupRecord = {
    body: HTMLElement,
    options: PopupOptions,
};

const _popups = new Map<Id, PopupRecord>();

export async function update_popup(marker_id: Id, popup_id: Id, options: PopupOptions) {
    const id = `dioxus-leaflet-popup-${popup_id}`;
    const body = document.getElementById(id);
    if (!body) {
        throw new Error(`Popup body element with id ${id} not found when updating popup for object ${marker_id}`);
    }
    _popups.set(popup_id, { body, options });

    let marker = get_marker(marker_id);
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