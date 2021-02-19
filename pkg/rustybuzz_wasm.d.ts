/* tslint:disable */
/* eslint-disable */
/**
* @param {number} font_idx
* @param {string} font_bytes_hex
*/
export function register_font(font_idx: number, font_bytes_hex: string): void;
/**
* @param {any} user_cfg
* @returns {string}
*/
export function shape_text(user_cfg: any): string;
/**
* @param {any} js_font_idx
* @param {any} js_glyph_id
* @returns {string}
*/
export function glyph_to_svg_pathdata(js_font_idx: any, js_glyph_id: any): string;
/**
* return JSON `list<number>` with one wordcount per line
* TODO: return list of slab indices
* @param {string} text
* @param {number} width
* @returns {string}
*/
export function wrap_text(text: string, width: number): string;
/**
*/
export function wrap_text_with_arbitrary_slabs(): void;
