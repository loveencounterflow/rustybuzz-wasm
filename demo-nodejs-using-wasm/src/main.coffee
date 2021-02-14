


'use strict'


############################################################################################################
CND                       = require 'cnd'
rpr                       = CND.rpr
badge                     = 'DEMO-HARFBUZZJS'
debug                     = CND.get_logger 'debug',     badge
alert                     = CND.get_logger 'alert',     badge
whisper                   = CND.get_logger 'whisper',   badge
warn                      = CND.get_logger 'warn',      badge
help                      = CND.get_logger 'help',      badge
urge                      = CND.get_logger 'urge',      badge
info                      = CND.get_logger 'info',      badge
echo                      = CND.echo.bind CND
FS                        = require 'fs'
PATH                      = require 'path'
# { promisify }


############################################################################################################
if module is require.main then do =>
  globalThis.alert          = alert
  globalThis.help           = help
  globalThis.urge           = urge
  globalThis.info           = info
  globalThis.debug          = debug
  # globalThis.read_file      = FS.readFileSync
  globalThis.read_file      = ( path ) ->
    urge '^44877^', rpr path
    # return FS.readFileSync path
    return true
  ### NOTE only works with `wasm-pack build --target nodejs` ###
  RBW                 = require '../../pkg'
  font_path           = 'EBGaramond08-Italic.otf'
  font_path           = 'arabic/Amiri-0.113/Amiri-Bold.ttf'
  font_path           = PATH.resolve PATH.join __dirname, '../../fonts', font_path
  # font_path           = '/home/flow/io/mingkwai-rack/jizura-fonts/fonts/EBGaramond08-Italic.otf'
  font_bytes          = FS.readFileSync font_path
  font_bytes_hex      = font_bytes.toString 'hex'
  RBW.set_font_bytes font_bytes_hex unless RBW.has_font_bytes()
  # font_bytes_hex      = 'abcdefgh'
  # format              = 'short'
  format              = 'json'
  # format              = 'rusty'
  shy                 = '\xad'
  texts               = [
    # "a"
    # "affix"
    ( [ "الخط الأمیری"... ].reverse() ).join ''
    # "af#fix"
    # " "
    # "#"
    # "-"
    ]
  echo """<?xml version='1.0' encoding='UTF-8'?>
    <svg xmlns='http://www.w3.org/2000/svg' width='6000' height='3000' viewBox='-100 -1500 5900 1500' version='2'>"""
  # for text in texts
  text        = texts[ 0 ]
  text        = text.replace /#/g, shy
  cfg         = { format, text, }
  arrangement = JSON.parse RBW.shape_text cfg
  gids        = new Set ( d.gid for d in arrangement )
  debug '^3344^', gids
  #.........................................................................................................
  echo """<style>
    path {
      stroke:                 black;
      stroke-width:           8px;
      fill:                   #880000bd;; }
    rect {
      stroke:                 black;
      stroke-width:           3px;
      fill:                   #ffeb3b42; }
      </style>"""
  #.........................................................................................................
  echo "<defs>"
  for gid from gids.values()
    outline = JSON.parse RBW.glyph_to_svg_pathdata gid
    debug '^3344^', gid, outline
    # continue if outline.pd is ''
    echo "<symbol overflow='visible' id='b#{gid}'>#{outline.br}</symbol>"
    echo "<symbol overflow='visible' id='g#{gid}'><path d='#{outline.pd}'/></symbol>"
  echo "</defs>"
  #.........................................................................................................
  for d in arrangement
    echo "<use href='#g#{d.gid}' x='#{d.x}' y='#{d.y}'/>"
    echo "<use href='#b#{d.gid}' x='#{d.x}' y='#{d.y}'/>"
    # echo "<g x='#{d.x}' y='#{d.y + 1000}'>"
    # echo "#{outline.br}"
    # echo "</g>"
  #.........................................................................................................
  echo "</svg>"
  return null







