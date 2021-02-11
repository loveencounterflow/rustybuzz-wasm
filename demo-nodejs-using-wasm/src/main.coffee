


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
FS                        = ( require 'fs' ).promises
PATH                      = require 'path'
# { promisify }


############################################################################################################
if module is require.main then do =>
  globalThis.alert  = alert
  globalThis.help   = help
  globalThis.urge   = urge
  globalThis.info   = info
  ### NOTE only works with `wasm-pack build --target nodejs` ###
  HELO = require '../../pkg'
  font_path         = 'EBGaramond08-Italic.otf'
  font_path         = PATH.resolve PATH.join __dirname, '../fonts', font_path
  cfg =
    text:       "text for typesetting"
    font_file:  font_path
  HELO.greet cfg
  delete cfg.help
  delete cfg.text
  delete cfg.perhaps
  HELO.greet cfg
  return null







