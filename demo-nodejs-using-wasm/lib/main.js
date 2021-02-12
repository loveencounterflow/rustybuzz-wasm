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

  FS = require('fs');

  PATH = require('path');

  // { promisify }

  //###########################################################################################################
  if (module === require.main) {
    (() => {
      /* NOTE only works with `wasm-pack build --target nodejs` */
      var HELO, font_path, xxx;
      globalThis.alert = alert;
      globalThis.help = help;
      globalThis.urge = urge;
      globalThis.info = info;
      globalThis.debug = debug;
      // globalThis.read_file      = FS.readFileSync
      globalThis.read_file = function(path) {
        urge('^44877^', rpr(path));
        // return FS.readFileSync path
        return true;
      };
      HELO = require('../../pkg');
      font_path = 'EBGaramond08-Italic.otf';
      font_path = PATH.resolve(PATH.join(__dirname, '../../fonts', font_path));
      // font_path           = '/home/flow/io/mingkwai-rack/jizura-fonts/fonts/EBGaramond08-Italic.otf'
      xxx = function() {
        var cfg, font_bytes, font_bytes_hex, text;
        font_bytes = FS.readFileSync(font_path);
        font_bytes_hex = font_bytes.toString('hex');
        // font_bytes_hex      = 'abcdefgh'
        text = "text for typesetting";
        cfg = {font_bytes_hex, text};
        // delete cfg.font_path
        // delete cfg.font_bytes
        return info('^223^', HELO.shape_text(cfg));
      };
      xxx();
      info('^223^', HELO.f("abc"));
      return null;
    })();
  }

}).call(this);

//# sourceMappingURL=main.js.map