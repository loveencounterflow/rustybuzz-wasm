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
      var RBW, cfg, font_bytes, font_bytes_hex, font_path, format, i, len, shy, text, texts;
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
      RBW = require('../../pkg');
      font_path = 'EBGaramond08-Italic.otf';
      font_path = PATH.resolve(PATH.join(__dirname, '../../fonts', font_path));
      // font_path           = '/home/flow/io/mingkwai-rack/jizura-fonts/fonts/EBGaramond08-Italic.otf'
      font_bytes = FS.readFileSync(font_path);
      font_bytes_hex = font_bytes.toString('hex');
      if (!RBW.has_font_bytes()) {
        RBW.set_font_bytes(font_bytes_hex);
      }
      // font_bytes_hex      = 'abcdefgh'
      // format              = 'short'
      // format              = 'json'
      format = 'rusty';
      shy = '\xad';
      texts = ["affix", "af#fix", " ", "#", "-"];
      for (i = 0, len = texts.length; i < len; i++) {
        text = texts[i];
        text = text.replace(/#/g, shy);
        cfg = {format, text};
        info('^223^', RBW.shape_text(cfg));
      }
      return null;
    })();
  }

}).call(this);

//# sourceMappingURL=main.js.map