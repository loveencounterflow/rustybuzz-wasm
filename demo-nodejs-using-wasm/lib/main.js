(function() {
  'use strict';
  var CND, FS, PATH, RBW, alert, badge, debug, echo, help, info, rpr, urge, warn, whisper;

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

  RBW = require('../../pkg');

  //-----------------------------------------------------------------------------------------------------------
  this.demo_text_shaping = function() {
    var arrangement, cfg, d, font_bytes, font_bytes_hex, font_path, format, gid, gids, i, len, outline, ref, shy, text, texts;
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
    font_path = 'EBGaramond08-Italic.otf';
    // font_path           = 'arabic/Amiri-0.113/Amiri-Bold.ttf'
    font_path = PATH.resolve(PATH.join(__dirname, '../../fonts', font_path));
    // font_path           = '/usr/share/fonts/truetype/tibetan-machine/TibetanMachineUni.ttf'
    font_bytes = FS.readFileSync(font_path);
    font_bytes_hex = font_bytes.toString('hex');
    if (!RBW.has_font_bytes()) {
      RBW.set_font_bytes(font_bytes_hex);
    }
    // format              = 'short'
    format = 'json';
    // format              = 'rusty'
    shy = '\xad';
    // "a"
    // "affix"
    // "ཨོཾ་མ་ཎི་པདྨེ་ཧཱུྃ"
    // ( [ "الخط الأمیری"... ].reverse() ).join ''
    texts = ["a certain minimum"];
    // "af#fix-"
    // " "
    // "#"
    // "-"
    echo(`<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='6000' height='3000' viewBox='-100 -1500 10500 1500' version='2'>`);
    // for text in texts
    text = texts[0];
    text = text.replace(/#/g, shy);
    cfg = {format, text};
    arrangement = JSON.parse(RBW.shape_text(cfg));
    gids = new Set((function() {
      var i, len, results;
      results = [];
      for (i = 0, len = arrangement.length; i < len; i++) {
        d = arrangement[i];
        results.push(d.gid);
      }
      return results;
    })());
    debug('^3344^', gids);
    //.........................................................................................................
    echo(`<style>
path {
  stroke:                 transparent;
  stroke-width:           0mm;
  fill:                   black;; }
rect {
  stroke:                 transparent;
  stroke-width:           0;
  fill:                   transparent; }
  </style>`);
    // echo """<style>
    //   path {
    //     stroke:                 black;
    //     stroke-width:           8px;
    //     fill:                   #880000bd;; }
    //   rect {
    //     stroke:                 black;
    //     stroke-width:           3px;
    //     fill:                   #ffeb3b42; }
    //     </style>"""
    //.........................................................................................................
    echo("<defs>");
    ref = gids.values();
    for (gid of ref) {
      outline = JSON.parse(RBW.glyph_to_svg_pathdata(gid));
      debug('^3344^', gid, outline.pd.slice(0, 101));
      // continue if outline.pd is ''
      echo(`<symbol overflow='visible' id='b${gid}'>${outline.br}</symbol>`);
      echo(`<symbol overflow='visible' id='g${gid}'><path d='${outline.pd}'/></symbol>`);
    }
    echo("</defs>");
//.........................................................................................................
    for (i = 0, len = arrangement.length; i < len; i++) {
      d = arrangement[i];
      echo(`<use href='#g${d.gid}' x='${d.x}' y='${d.y}'/>`);
      echo(`<use href='#b${d.gid}' x='${d.x}' y='${d.y}'/>`);
    }
    // echo "<g x='#{d.x}' y='#{d.y + 1000}'>"
    // echo "#{outline.br}"
    // echo "</g>"
    //.........................................................................................................
    echo("</svg>");
    return null;
  };

  //-----------------------------------------------------------------------------------------------------------
  this.demo_text_wrapping = function() {
    var i, j, last_line_idx, last_word_idx, len, line, line_idx, line_length, lines, ref, text, width, word_idx, words;
    text = `Knuth–Liang hyphenation operates at the level of individual words, but there can be ambiguity as
to what constitutes a word. All hyphenation dictionaries handle the expected set of word-forming graphemes
from their respective alphabets, but some also accept punctuation marks such as hyphens and apostrophes,
and are thus capable of handling hyphen-joined compound words or elisions. Even so, it's generally
preferable to handle punctuation at the level of segmentation, as it affords greater control over the
final result (such as where to break hyphen-joined compounds, or whether to set a leading hyphen on new
lines).
在文本的显示中， 换行 （line wrap）是指文本在一行已满的情况下转到新行，使得每一行都能在窗口范围看到，不需要任何水平的滚动。 自动换行 （word
wrap）是大多数文字編輯器、文書處理器、和网页浏览器的一个附加功能。它用于在行间或一行里的单词间隔处分行，不考虑一个单词超过一行长度的情况。`;
    text = text.replace(/\s+/g, ' ');
    width = 50;
    lines = RBW.wrap_text(text, width);
    lines = lines.split('\n');
    last_line_idx = lines.length - 1;
    debug('^449^', lines);
    for (line_idx = i = 0, len = lines.length; i < len; line_idx = ++i) {
      line = lines[line_idx];
      // debug '^499^', words
      if (line_idx < last_line_idx) {
        line_length = line.length;
        words = line.split(/\s+/);
        last_word_idx = words.length - 1;
        while (true) {
          if (last_word_idx < 1) {
            break;
          }
          if (line_length >= width) {
            break;
          }
          for (word_idx = j = 0, ref = last_word_idx; (0 <= ref ? j < ref : j > ref); word_idx = 0 <= ref ? ++j : --j) {
            if (line_length >= width) {
              // debug word_idx
              break;
            }
            if (!(Math.random() > 0.5)) {
              continue;
            }
            line_length++;
            words[word_idx] += ' ';
          }
        }
        info(words.join(' '));
      } else {
        info(line);
      }
    }
    return null;
  };

  //###########################################################################################################
  if (module === require.main) {
    (() => {
      return this.demo_text_wrapping();
    })();
  }

}).call(this);

//# sourceMappingURL=main.js.map