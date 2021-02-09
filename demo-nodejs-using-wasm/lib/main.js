(function() {
  'use strict';
  var CND, alert, badge, debug, echo, help, info, rpr, urge, warn, whisper;

  //###########################################################################################################
  CND = require('cnd');

  rpr = CND.rpr;

  badge = 'DEMO-HARFBUZZJS';

  debug = CND.get_logger('debug', badge);

  alert = CND.get_logger('alert', badge);

  whisper = CND.get_logger('whisper', badge);

  warn = CND.get_logger('warn', badge);

  help = CND.get_logger('help', badge);

  urge = CND.get_logger('urge', badge);

  info = CND.get_logger('info', badge);

  echo = CND.echo.bind(CND);

  //###########################################################################################################
  if (module === require.main) {
    (async() => {
      var error, i, len, path, paths;
      paths = ['../../pkg/hello_wasm.js', '../../pkg/hello_wasm_bg.js', '../../pkg/hello_wasm_bg.wasm', '../../pkg/package.json', '@loveencounterflow/hello-wasm'];
      for (i = 0, len = paths.length; i < len; i++) {
        path = paths[i];
        whisper(path);
        try {
          debug((await require(path)));
        } catch (error1) {
          error = error1;
          warn(error.message);
        }
      }
      return null;
    })();
  }

}).call(this);

//# sourceMappingURL=main.js.map