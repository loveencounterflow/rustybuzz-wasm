/* tslint:disable */
/* eslint-disable */
/**
* @returns {number}
*/
export function inc(): number;
/**
* @returns {number}
*/
export function dec(): number;
/**
* @param {string} font_bytes_hex
*/
export function set_font_bytes(font_bytes_hex: string): void;
/**
* @returns {boolean}
*/
export function has_font_bytes(): boolean;
/**
* @param {any} user_cfg
* @returns {string}
*/
export function shape_text(user_cfg: any): string;
/**
* @param {any} js_glyph_id
* @returns {string}
*/
export function glyph_to_svg_pathdata(js_glyph_id: any): string;
/**
* return JSON `list<number>` with one wordcount per line
* TODO: return list of slab indices
* @param {string} text
* @param {number} width
* @returns {string}
*/
export function wrap_text(text: string, width: number): string;
