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
      var HELO, cfg, font_path;
      globalThis.alert = alert;
      globalThis.help = help;
      globalThis.urge = urge;
      globalThis.info = info;
      HELO = require('../../pkg');
      font_path = 'EBGaramond08-Italic.otf';
      font_path = PATH.resolve(PATH.join(__dirname, '../fonts', font_path));
      cfg = {
        text: "text for typesetting",
        font_file: font_path
      };
      HELO.greet(cfg);
      delete cfg.help;
      delete cfg.text;
      delete cfg.perhaps;
      HELO.greet(cfg);
      return null;
    })();
  }

}).call(this);

//# sourceMappingURL=main.js.map