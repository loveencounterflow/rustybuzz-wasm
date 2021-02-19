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
  this._set_globals = function() {
    globalThis.alert = alert;
    globalThis.help = help;
    globalThis.urge = urge;
    globalThis.info = info;
    globalThis.debug = debug;
    // globalThis.read_file      = ( path ) ->
    //   urge '^44877^', rpr path
    //   # return FS.readFileSync path
    //   return true
    return null;
  };

  //-----------------------------------------------------------------------------------------------------------
  this._resolve_font_path = function(me, font_path) {
    if (font_path.startsWith('/')) {
      return font_path;
    }
    return PATH.resolve(PATH.join(__dirname, '../../fonts', font_path));
  };

  //-----------------------------------------------------------------------------------------------------------
  this._get_font_bytes = function(me, font_path) {
    return (FS.readFileSync(font_path)).toString('hex');
  };

  //-----------------------------------------------------------------------------------------------------------
  this.register_font = function(me, fontnick) {
    var R, font_entry;
    //.........................................................................................................
    if ((font_entry = me.fonts[fontnick]) == null) {
      throw new Error(`^1w37^ unknown fontnick ${rpr(fontnick)}`);
    }
    //.........................................................................................................
    if (!(me._prv_fontidx < me._last_fontidx)) {
      throw new Error(`^1w37^ capacity of ${me._last_fontidx + 1} fonts exceeded`);
    }
    if ((R = font_entry.font_idx) != null) {
      //.........................................................................................................
      return R;
    }
    //.........................................................................................................
    R = me._prv_fontidx += 1;
    RBW.register_font(R, this._get_font_bytes(me, font_entry.path));
    font_entry.font_idx = R;
    return R;
  };

  //-----------------------------------------------------------------------------------------------------------
  this.new_demo = function() {
    var R, entry, fontname, ref;
    this._set_globals();
    R = {
      shy: '\xad',
      _prv_fontidx: -1,
      _last_fontidx: 15,
      fonts: {
        garamond_italic: {
          path: 'EBGaramond08-Italic.otf'
        },
        amiri: {
          path: 'arabic/Amiri-0.113/Amiri-Bold.ttf'
        },
        tibetan: {
          path: '/usr/share/fonts/truetype/tibetan-machine/TibetanMachineUni.ttf'
        }
      }
    };
    ref = R.fonts;
    //.........................................................................................................
    for (fontname in ref) {
      entry = ref[fontname];
      R.fonts[fontname].font_idx = null;
      R.fonts[fontname].path = this._resolve_font_path(null, entry.path);
    }
    //.........................................................................................................
    return R;
  };

  //-----------------------------------------------------------------------------------------------------------
  this.demo_text_shaping = function() {
    var arrangement, cfg, d, font_idx, fontnick, format, gids, i, len, me, text;
    whisper('^33443^ demo_text_shaping');
    me = this.new_demo();
    fontnick = 'garamond_italic';
    font_idx = this.register_font(me, fontnick);
    // format              = 'short'
    format = 'json';
    // format              = 'rusty'
    text = "a certain minimum";
    text = text.replace(/#/g, me.shy);
    cfg = {format, text};
    arrangement = JSON.parse(RBW.shape_text(cfg));
    //.........................................................................................................
    urge(`glyf IDs and positions of font ${rpr(fontnick)} for text ${rpr(text)}:`);
    for (i = 0, len = arrangement.length; i < len; i++) {
      d = arrangement[i];
      info('^223^', d);
    }
    //.........................................................................................................
    urge("unique glyf IDs in this text:");
    gids = new Set((function() {
      var j, len1, results;
      results = [];
      for (j = 0, len1 = arrangement.length; j < len1; j++) {
        d = arrangement[j];
        results.push(d.gid);
      }
      return results;
    })());
    debug('^3344^', gids);
    return null;
  };

  //-----------------------------------------------------------------------------------------------------------
  this.demo_svg_typesetting = function() {
    var arrangement, cfg, d, font_idx, fontnick, format, gid, gids, i, j, k, last_line_idx, last_word_idx, len, len1, line, line_idx, line_length, lines, me, outline, ref, ref1, text, width, word_idx, words;
    whisper('^33443^ demo_svg_typesetting');
    me = this.new_demo();
    format = 'json'; // 'short', 'rusty'
    //.........................................................................................................
    fontnick = 'tibetan';
    text = "ཨོཾ་མ་ཎི་པདྨེ་ཧཱུྃ";
    fontnick = 'amiri';
    text = ([..."الخط الأمیری"].reverse()).join('');
    fontnick = 'garamond_italic';
    text = "a certain minimum";
    fontnick = 'garamond_italic';
    text = "af#fix";
    //.........................................................................................................
    font_idx = this.register_font(me, fontnick);
    text = text.replace(/#/g, me.shy);
    //.........................................................................................................
    echo(`<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='6000' height='3000' viewBox='-100 -1500 10500 1500' version='2'>`);
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
      outline = JSON.parse(RBW.glyph_to_svg_pathdata(font_idx, gid));
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
    whisper('^33443^ demo_text_wrapping');
    me = this.new_demo();
    text = `Knuth–Liang hyphenation operates at the level of individual words, but there can be ambiguity as
to what constitutes a word. All hyphenation dictionaries handle the expected set of word-forming graphemes
from their respective alphabets, but some also accept punctuation marks such as hyphens and apostrophes,
and are thus capable of handling hyphen-joined compound words or elisions. Even so, it's generally
preferable to handle punctuation at the level of segmentation, as it affords greater control over the
final result (such as where to break hyphen-joined compounds, or whether to set a leading hyphen on new
lines).
在文本的显示中， 换行 （line wrap）是指文本在一行已满的情况下转到新行，使得每一行都能在窗口范围看到，不需要任何水平的滚动。 自动换行 （word wrap） 是 大 多 数 文 字 編 輯 器 、 文書處理器、和网页浏览器的一个附加功能。它用于在行间或一行里的单词间隔处分行，不考虑一个单词超过一行长度的情况。`;
    text = "The ela#bo#ra#te sphinx told me a rid#dle.";
    // text          = "The elaborate sphinx told me a riddle."
    //.........................................................................................................
    text = text.replace(/#/g, me.shy);
    text = text.replace(/\s+/g, ' ');
    width = 10;
    lines = RBW.wrap_text(text, width);
    debug('^3383^', lines);
    return null;
    lines = lines.split('\n');
    last_line_idx = lines.length - 1;
    debug('^449^', lines);
    for (line_idx = j = 0, len1 = lines.length; j < len1; line_idx = ++j) {
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
          for (word_idx = k = 0, ref1 = last_word_idx; (0 <= ref1 ? k < ref1 : k > ref1); word_idx = 0 <= ref1 ? ++k : --k) {
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

  //-----------------------------------------------------------------------------------------------------------
  this.demo_text_wrapping_advanced = function() {
    var i, len, me, ref, slabline, slablines;
    me = this.new_demo();
    slablines = JSON.parse(RBW.wrap_text_with_arbitrary_slabs());
    debug('^3334^', rpr(slablines));
    ref = slablines.lines;
    for (i = 0, len = ref.length; i < len; i++) {
      slabline = ref[i];
      info(slabline);
    }
    return null;
  };

  //###########################################################################################################
  if (module === require.main) {
    (() => {
      // @demo_text_shaping()
      // @demo_svg_typesetting()
      // @demo_text_wrapping()
      this.demo_text_wrapping_advanced();
      return null;
    })();
  }

}).call(this);

//# sourceMappingURL=main.js.map