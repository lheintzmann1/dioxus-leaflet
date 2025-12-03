export * as L from "./leaflet";

export type Json = string | number | boolean | null | { [key: string]: Json } | Json[];

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

export interface MarkerIcon {
    icon_url: string,
    icon_size?: [number, number],
    icon_anchor?: [number, number],
    popup_anchor?: [number, number],
    shadow_url?: string,
    shadow_size?: [number, number],
}

export interface PopupOptions {
    max_width?: number,
    min_width?: number,
    max_height?: number,
    auto_pan?: boolean,
    keep_in_view?: boolean,
    close_button?: boolean,
    auto_close?: boolean,
    close_on_escape_key?: boolean,
    class_name?: string,
}