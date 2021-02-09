


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

#-----------------------------------------------------------------------------------------------------------
@load_wasm = ( path ) ->
  env =
    memoryBase: 0,
    tableBase: 0,
    memory: new WebAssembly.Memory { initial: 256, }
    table:  new WebAssembly.Table { initial: 0, element: 'anyfunc', }
  data        = await FS.readFile path
  typedArray  = new Uint8Array data
  result      = await WebAssembly.instantiate typedArray #, env.memory
  # result      = await WebAssembly.instantiate typedArray, { env, }
  # result  = await WebAssembly.instantiateStreaming data
  result.instance.exports.memory.grow 400 # each page is 64kb in size
  return result.instance


############################################################################################################
if module is require.main then do =>
  globalThis.alert = alert
  ### NOTE only works with `wasm-pack build --target nodejs` ###
  HELO = require '../../pkg'
  HELO.greet 'everyone'
  return null







