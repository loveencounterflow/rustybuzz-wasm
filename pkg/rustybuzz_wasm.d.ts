/* tslint:disable */
/* eslint-disable */
/**
* @param {number} font_idx
* @param {string} font_bytes_hex
*/
export function register_font(font_idx: number, font_bytes_hex: string): void;
/**
* @param {number} font_idx
* @returns {boolean}
*/
export function font_register_is_free(font_idx: number): boolean;
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
* @param {any} slabs_js
* @param {any} line_width_js
* @returns {string}
*/
export function wrap_text_with_arbitrary_slabs(slabs_js: any, line_width_js: any): string;
/**
* @param {string} text
* @returns {string}
*/
export function find_line_break_positions(text: string): string;
