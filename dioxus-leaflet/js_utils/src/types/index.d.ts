export * as L from "./leaflet";

export type Json = string | number | boolean | null | { [key: string]: Json } | Json[];
export type RustCallback<A, R> = (arg: A) => Promise<R>;

export type Id = number;

export interface MapPosition {
    coordinates: [number, number],
    zoom: number,
}

export interface MapOptions {
    zoom_control?: boolean,
    scroll_wheel_zoom?: L.Zoom,
    double_click_zoom?: L.Zoom,
    touch_zoom?: L.Zoom,
    dragging?: boolean,
    keyboard?: boolean,
    attribution_control?: boolean,
    tile_layer: TileLayer,
}

export interface TileLayer {
    url: string,
    attribution: string,
    max_zoom: number,
    subdomains: string[],
}
