(function() {
  'use strict';
  var CND, FS, PATH, alert, badge, debug, echo, help, info, rpr, urge, warn, whisper;

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

  FS = (require('fs')).promises;

  PATH = require('path');

  // { promisify }

  //###########################################################################################################
  if (module === require.main) {
    (() => {
      /* NOTE only works with `wasm-pack build --target nodejs` */
      var HELO;
      globalThis.alert = alert;
      globalThis.help = help;
      globalThis.urge = urge;
      globalThis.info = info;
      HELO = require('../../pkg');
      HELO.greet('everyone');
      return null;
    })();
  }

}).call(this);

//# sourceMappingURL=main.js.map