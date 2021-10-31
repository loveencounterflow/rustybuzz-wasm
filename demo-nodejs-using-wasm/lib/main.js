(function() {
  'use strict';
  var CND, FS, INTERTEXT, PATH, RBW, _border, alert, badge, debug, echo, help, info, rpr, to_width, urge, warn, whisper;

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

  INTERTEXT = require('intertext');

  ({to_width} = require('to-width'));

  _border = CND.gold('█████     '.repeat(10));

  //-----------------------------------------------------------------------------------------------------------
  this._set_globals = function() {
    var _alert;
    _alert = alert;
    globalThis.alert = (...P) => {
      alert(_border);
      alert();
      _alert(CND.reverse(...P));
      alert();
      return alert(_border);
    };
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
    var R, font_bytes, font_entry;
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
    whisper(`^register_font@1^ reading font ${fontnick}...`);
    font_bytes = this._get_font_bytes(me, font_entry.path);
    whisper("^register_font@2^ ...done");
    whisper(`^register_font@3^ sending font ${fontnick} to registry...`);
    RBW.register_font(R, font_bytes);
    whisper("^register_font@4^ ...done");
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
        },
        notoserif: {
          path: 'NotoSerifJP/NotoSerifJP-Medium.otf'
        },
        djvsi: {
          path: 'DejaVuSerif-Italic.ttf'
        }
      },
      /* TAINT disregarding font, size for the moment */
      slab_widths: {}
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
    var font_idx, fontnick, format, me, text;
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
    fontnick = 'djvsi';
    text = "af#fix";
    //.........................................................................................................
    font_idx = this.register_font(me, fontnick);
    text = text.replace(/#/g, me.shy);
    //.........................................................................................................
    info('^48596^', rpr(RBW.get_font_metrics(font_idx)));
    return null;
  };

  //-----------------------------------------------------------------------------------------------------------
  this.demo_text_wrapping_advanced = function() {
    /* TAINT use triplets `[m,w,p,]` (material width, whitespace width, penalty width) instead to make JSON significantly smaller */
    var i, len, me, ref, slabline, slablines, slabs;
    me = this.new_demo();
    slabs = [
      {
        width: 5,
        whitespace_width: 1,
        penalty_width: 1
      },
      {
        width: 3,
        whitespace_width: 1,
        penalty_width: 1
      },
      {
        width: 4,
        whitespace_width: 1,
        penalty_width: 1
      },
      {
        width: 2,
        whitespace_width: 1,
        penalty_width: 1
      },
      {
        width: 5,
        whitespace_width: 1,
        penalty_width: 1
      },
      {
        width: 10,
        whitespace_width: 1,
        penalty_width: 1
      }
    ];
    slablines = JSON.parse(RBW.wrap_text_with_arbitrary_slabs(slabs));
    debug('^3334^', rpr(slablines));
    ref = slablines.lines;
    for (i = 0, len = ref.length; i < len; i++) {
      slabline = ref[i];
      info(slabline);
    }
    return null;
  };

  //-----------------------------------------------------------------------------------------------------------
  this.find_widths_from_slabs = function(me, slabs) {
    var end, i, len, new_slabs, ref, slab_code, slab_idx, slab_text;
    debug('^443^', slabs);
    new_slabs = new Set();
    ref = slabs.slabs;
    for (slab_idx = i = 0, len = ref.length; i < len; slab_idx = ++i) {
      slab_text = ref[slab_idx];
      end = slabs.ends[slab_idx];
      slab_code = slab_text + end;
      if (me.slab_widths[slab_code] != null) {
        continue;
      }
      switch (end) {
        case '_':
          new_slabs.add(slab_text); // + ''
          break;
        case '|':
          new_slabs.add(slab_text); // + ''
          new_slabs.add(slab_text + '-');
          break;
        case 'x':
          new_slabs.add(slab_text); // + ''
      }
    }
    debug('^3344^', new_slabs);
    return null;
  };

  //-----------------------------------------------------------------------------------------------------------
  this.get_font_metrics = function(me, font_idx) {
    /* TAINT cache font metrics */
    /* TAINT validate arguments, results */
    var arrangement, endash, format, hyphen, space, text;
    format = 'json';
    text = " - –";
    arrangement = JSON.parse(RBW.shape_text({font_idx, text, format}));
    space = arrangement[0];
    hyphen = arrangement[1];
    endash = arrangement[3];
    return {
      // debug '^get_font_metrics@445^', arrangement
      space: {
        gid: space.gid,
        dx: space.dx
      },
      hyphen: {
        gid: hyphen.gid,
        dx: hyphen.dx
      },
      endash: {
        gid: endash.gid,
        dx: endash.dx
      }
    };
  };

  //-----------------------------------------------------------------------------------------------------------
  this._firstchr = function(buffer, idx) {
    /* Return the first character found in buffer at given position, assuming UTF-8. */
    return (Array.from(buffer.slice(idx, idx + 4).toString()))[0];
  };

  //-----------------------------------------------------------------------------------------------------------
  this._slice_buffer = function(buffer, start_idx, stop_idx) {
    return buffer.slice(start_idx, stop_idx).toString();
  };

  //-----------------------------------------------------------------------------------------------------------
  this.demo_typesetting = function() {
    /* TAINT incorrect of course */
    var batch, batch_idx, chunk, first_textshape, fm, font_idx, fontnick, i, j, l, last_line_idx, last_textshape, last_word_idx, lbo_start, lbo_starts, lbo_stop, len, len1, len2, len3, len4, len5, line, line_idx, line_length, line_width, lines, m, me, n, o, p, q, r, ref, ref1, ref2, ref3, ref4, ref5, ref6, shape, shape_batch, shape_batches, shapes, slab, slab_idx, slabline, slabline_idx, slablines, slabs, text, text_bfr, width, word_idx, words;
    me = this.new_demo();
    whisper('^33443^ demo_typesetting');
    //.........................................................................................................
    text = `Knuth–Liang hyphenation operates at the level of individual words, but there can be ambiguity as
to what constitutes a word. All hyphenation dictionaries handle the expected set of word-forming graphemes
from their respective alphabets, but some also accept punctuation marks such as hyphens and apostrophes,
and are thus capable of handling hyphen-joined compound words or elisions. Even so, it's generally
preferable to handle punctuation at the level of segmentation, as it affords greater control over the
final result (such as where to break hyphen-joined compounds, or whether to set a leading hyphen on new
lines).
在文本的显示中， 换行 （line wrap）是指文本在一行已满的情况下转到新行，使得每一行都能在窗口范围看到，不需要任何水平的滚动。 自动换行 （word wrap） 是 大 多 数 文 字 編 輯 器 、 文書處理器、和网页浏览器的一个附加功能。它用于在行间或一行里的单词间隔处分行，不考虑一个单词超过一行长度的情况。`;
    // text          = "Knuth–Liang hyphenation" ## en-dash U+2013 ###
    // text          = "Knuth-Liang hyphenation" ### hyphen-minus U+002d ###
    // text          = "今日も明日も。"
    // text          = "The elaborate sphinx told me a riddle, told me a riddle, told me a riddle."
    text = "the affixation";
    // text          = "affix"
    help("^33376^ text:", to_width(rpr(text), 100));
    //.........................................................................................................
    line_width = 6 * 500;
    fontnick = 'garamond_italic';
    font_idx = this.register_font(me, fontnick);
    fm = this.get_font_metrics(me, font_idx);
    info('^222332^', `fontnick: ${rpr(fontnick)}`);
    info('^222332^', `font metric: ${rpr(fm)}`);
    //.........................................................................................................
    /* Prepare text: normalize whitespace (replace incidental newlines, repeated blanks), then hyphenate it.
     Prepare a buffer so we access the underlying raw bytes (`RBW.find_line_break_positions()` and
     `RBW.shape_text()` both return positions into the raw bytes). Find the line break opportunities (LBOs) as
     a list of byte indexes: */
    text = text.replace(/\s+/g, ' ');
    text = INTERTEXT.HYPH.hyphenate(text);
    text_bfr = Buffer.from(text, {
      encoding: 'utf-8'
    });
    lbo_starts = JSON.parse(RBW.find_line_break_positions(text));
    help("^33376^ lbo_starts:", lbo_starts);
    //.........................................................................................................
    /* We have made it so that the LBO indexes always start with zero and end with the index to the first
     byte after the end of the buffer; hence, we can 'hydrate' the raw indices by looking at the current and
     the following index to find the corresponding 'chunk' (i.e. the piece of text that stretches from the
     previous to the upcomping line break opportunity). Each chunk in turn will, after text shaping, correspond
     to any number of glyf outlines ('shapes'), so we provide a list for them: */
    shape_batches = [];
    for (batch_idx = i = 0, ref = lbo_starts.length - 1; (0 <= ref ? i < ref : i > ref); batch_idx = 0 <= ref ? ++i : --i) {
      lbo_start = lbo_starts[batch_idx];
      lbo_stop = lbo_starts[batch_idx + 1];
      chunk = this._slice_buffer(text_bfr, lbo_start, lbo_stop);
      chunk = chunk.replace(/\xad/g, '|');
      shape_batch = {
        lbo_start,
        lbo_stop,
        chunk,
        shapes: []
      };
      shape_batches.push(shape_batch);
      urge('^454-1^', lbo_start, rpr(chunk), shape_batch);
    }
    help("^33376^ shape_batches:", shape_batches);
    //.........................................................................................................
    /* Now we shape the text. Observe that any number of Unicode codepoints may correspond to any number
     of visible and invisible outlines with any kind of relationship between codepoints and glyf IDs depending
     on the font and the exact sequence of codepoints. This is especially apparent in so-called 'complex
     scripts' like Arabic and Devanagari, but also present in Latin scripts where ligatures are present. These
     ligatures will later on reqquire our attention because they crucially depend on the results of line
     wrapping (e.g. `affix` may be written out with a `ﬃ` ligature when being unhyphenated, but end up as
     `af-`, `ﬁx` when wrapped across two lines). This in turn will result in either incorrect shaping or
     incorrect line wrapping, so should be dealt with. */
    shapes = JSON.parse(RBW.shape_text({
      font_idx,
      text,
      format: 'json'
    }));
    // help "^33376^ shapes:", shapes
    //.........................................................................................................
    /* Bring the chunks that fall out from LBO analysis together with the shapes (positioned outlines)
     that result from text shaping: */
    batch_idx = 0;
    batch = shape_batches[batch_idx];
    for (j = 0, len = shapes.length; j < len; j++) {
      shape = shapes[j];
      if (shape.bidx >= batch.lbo_stop) {
        batch_idx++;
        batch = shape_batches[batch_idx];
        if (!((batch.lbo_start <= (ref1 = shape.bidx) && ref1 < batch.lbo_stop))) {
          throw new Error(`^3332^ POD ${rpr(shape)} does not fit into shape batch ${rpr(batch)}`);
        }
      }
      batch.shapes.push(shape);
    }
// urge '^3332^', batch
//.........................................................................................................
/* Show shape batches: */
    for (l = 0, len1 = shape_batches.length; l < len1; l++) {
      shape_batch = shape_batches[l];
      ({lbo_start, lbo_stop, chunk} = shape_batch);
      help('^3334^ shape_batch:', {
        lbo_start,
        lbo_stop,
        chunk,
        shapes: '...'
      });
      ref2 = shape_batch.shapes;
      for (m = 0, len2 = ref2.length; m < len2; m++) {
        shape = ref2[m];
        info(`  ^3334^ shape: ${rpr(shape)}`);
      }
    }
    //.........................................................................................................
    /* Perform line wrapping: */
    slabs = [];
    for (n = 0, len3 = shape_batches.length; n < len3; n++) {
      shape_batch = shape_batches[n];
      ({shapes} = shape_batch);
      first_textshape = shapes[0];
      last_textshape = shapes[shapes.length - 1];
      width = (last_textshape.x + last_textshape.dx) - first_textshape.x;
      slab = {
        width,
        whitespace_width: fm.space.dx,
        penalty_width: fm.hyphen.dx
      };
      debug('^3345^', slab);
      slabs.push(slab);
    }
    slablines = JSON.parse(RBW.wrap_text_with_arbitrary_slabs(slabs, line_width));
    ref3 = slablines.lines;
    //.........................................................................................................
    /* Show shape batches: */
    // urge "^3334^ slablines:", slablines
    for (slabline_idx = o = 0, len4 = ref3.length; o < len4; slabline_idx = ++o) {
      slabline = ref3[slabline_idx];
      urge(`^3334^ line# ${slabline_idx + 1} slabline: ${rpr(slabline)}`);
      for (slab_idx = p = ref4 = slabline.first_slab_idx, ref5 = slabline.last_slab_idx; (ref4 <= ref5 ? p < ref5 : p > ref5); slab_idx = ref4 <= ref5 ? ++p : --p) {
        // for shape_idx in
        info(`  ^3334^ slab: ${rpr(slabs[slab_idx])}`);
      }
    }
    return null;
    // slabs     = [
    //   { width: 5,   whitespace_width: 1, penalty_width: 1, },
    //   { width: 3,   whitespace_width: 1, penalty_width: 1, },
    //   { width: 4,   whitespace_width: 1, penalty_width: 1, },
    //   { width: 2,   whitespace_width: 1, penalty_width: 1, },
    //   { width: 5,   whitespace_width: 1, penalty_width: 1, },
    //   { width: 10,  whitespace_width: 1, penalty_width: 1, }, ];
    //.........................................................................................................
    /* Write SVG: */
    //.........................................................................................................
    //.........................................................................................................
    //.........................................................................................................
    lines = RBW.wrap_text(text, line_width);
    debug('^3383^', lines);
    lines = lines.split('\n');
    last_line_idx = lines.length - 1;
    debug('^449^', lines);
    for (line_idx = q = 0, len5 = lines.length; q < len5; line_idx = ++q) {
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
          if (line_length >= line_width) {
            break;
          }
          for (word_idx = r = 0, ref6 = last_word_idx; (0 <= ref6 ? r < ref6 : r > ref6); word_idx = 0 <= ref6 ? ++r : --r) {
            if (line_length >= line_width) {
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
      var k;
      // @demo_text_shaping()
      this.demo_svg_typesetting();
      // @demo_get_font_metrics()
      // @demo_text_wrapping()
      // @demo_text_wrapping_advanced()
      // @demo_typesetting()
      debug('^36972^', (function() {
        var results;
        results = [];
        for (k in RBW) {
          results.push(k);
        }
        return results;
      })());
      return null;
    })();
  }

}).call(this);

//# sourceMappingURL=main.js.map