


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


############################################################################################################
if module is require.main then do =>
  paths = [
    '../../pkg/hello_wasm.js'
    '../../pkg/hello_wasm_bg.js'
    '../../pkg/hello_wasm_bg.wasm'
    '../../pkg/package.json'
    '@loveencounterflow/hello-wasm'
    ]
  for path in paths
    whisper path
    try debug await require path catch error then warn error.message
  return null







