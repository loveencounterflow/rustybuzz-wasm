


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
FS                        = require 'fs'
PATH                      = require 'path'
RBW                       = require '../../pkg'
INTERTEXT                 = require 'intertext'

#-----------------------------------------------------------------------------------------------------------
@_set_globals = ->
  globalThis.alert          = alert
  globalThis.help           = help
  globalThis.urge           = urge
  globalThis.info           = info
  globalThis.debug          = debug
  # globalThis.read_file      = ( path ) ->
  #   urge '^44877^', rpr path
  #   # return FS.readFileSync path
  #   return true
  return null

#-----------------------------------------------------------------------------------------------------------
@_resolve_font_path = ( me, font_path ) ->
  return font_path if font_path.startsWith '/'
  return PATH.resolve PATH.join __dirname, '../../fonts', font_path

#-----------------------------------------------------------------------------------------------------------
@_get_font_bytes = ( me, font_path ) -> ( FS.readFileSync font_path ).toString 'hex'

#-----------------------------------------------------------------------------------------------------------
@register_font = ( me, fontnick ) ->
  #.........................................................................................................
  unless ( font_entry = me.fonts[ fontnick ] )?
    throw new Error "^1w37^ unknown fontnick #{rpr fontnick}"
  #.........................................................................................................
  unless me._prv_fontidx < me._last_fontidx
    throw new Error "^1w37^ capacity of #{me._last_fontidx + 1} fonts exceeded"
  #.........................................................................................................
  return R if ( R = font_entry.font_idx )?
  #.........................................................................................................
  R           = me._prv_fontidx += 1
  whisper "^register_font@1^ reading font #{fontnick}..."
  font_bytes  = @_get_font_bytes me, font_entry.path
  whisper "^register_font@2^ ...done"
  whisper "^register_font@3^ sending font #{fontnick} to registry..."
  RBW.register_font R, font_bytes
  whisper "^register_font@4^ ...done"
  font_entry.font_idx = R
  return R

#-----------------------------------------------------------------------------------------------------------
@new_demo = ->
  @_set_globals()
  R =
    shy:          '\xad'
    _prv_fontidx:   -1
    _last_fontidx:  15
    fonts:
      garamond_italic:  { path: 'EBGaramond08-Italic.otf', }
      amiri:            { path: 'arabic/Amiri-0.113/Amiri-Bold.ttf', }
      tibetan:          { path: '/usr/share/fonts/truetype/tibetan-machine/TibetanMachineUni.ttf', }
      notoserif:        { path: 'NotoSerifJP/NotoSerifJP-Medium.otf', }
    ### TAINT disregarding font, size for the moment ###
    slab_widths: {}
  #.........................................................................................................
  for fontname, entry of R.fonts
    R.fonts[ fontname ].font_idx  = null
    R.fonts[ fontname ].path      = @_resolve_font_path null, entry.path
  #.........................................................................................................
  return R

#-----------------------------------------------------------------------------------------------------------
@demo_text_shaping = ->
  whisper '^33443^ demo_text_shaping'
  me                  = @new_demo()
  fontnick            = 'garamond_italic'
  font_idx            = @register_font me, fontnick
  # format              = 'short'
  format              = 'json'
  # format              = 'rusty'
  text                = "a certain minimum"
  text                = text.replace /#/g, me.shy
  cfg                 = { format, text, }
  arrangement         = JSON.parse RBW.shape_text cfg
  #.........................................................................................................
  urge "glyf IDs and positions of font #{rpr fontnick} for text #{rpr text}:"
  for d in arrangement
    info '^223^', d
  #.........................................................................................................
  urge "unique glyf IDs in this text:"
  gids                = new Set ( d.gid for d in arrangement )
  debug '^3344^', gids
  return null

#-----------------------------------------------------------------------------------------------------------
@demo_svg_typesetting = ->
  whisper '^33443^ demo_svg_typesetting'
  me        = @new_demo()
  format    = 'json' # 'short', 'rusty'
  #.........................................................................................................
  fontnick  = 'tibetan';          text =  "ཨོཾ་མ་ཎི་པདྨེ་ཧཱུྃ"
  fontnick  = 'amiri';            text = ( [ "الخط الأمیری"... ].reverse() ).join ''
  fontnick  = 'garamond_italic';  text = "a certain minimum"
  fontnick  = 'garamond_italic';  text = "af#fix"
  #.........................................................................................................
  font_idx  = @register_font me, fontnick
  text      = text.replace /#/g, me.shy
  #.........................................................................................................
  echo """<?xml version='1.0' encoding='UTF-8'?>
    <svg xmlns='http://www.w3.org/2000/svg' width='6000' height='3000' viewBox='-100 -1500 10500 1500' version='2'>"""
  cfg         = { format, text, }
  arrangement = JSON.parse RBW.shape_text cfg
  gids        = new Set ( d.gid for d in arrangement )
  debug '^3344^', gids
  #.........................................................................................................
  echo """<style>
    path {
      stroke:                 transparent;
      stroke-width:           0mm;
      fill:                   black;; }
    rect {
      stroke:                 transparent;
      stroke-width:           0;
      fill:                   transparent; }
      </style>"""
  # echo """<style>
  #   path {
  #     stroke:                 black;
  #     stroke-width:           8px;
  #     fill:                   #880000bd;; }
  #   rect {
  #     stroke:                 black;
  #     stroke-width:           3px;
  #     fill:                   #ffeb3b42; }
  #     </style>"""
  #.........................................................................................................
  echo "<defs>"
  for gid from gids.values()
    outline = JSON.parse RBW.glyph_to_svg_pathdata font_idx, gid
    debug '^3344^', gid, outline.pd[ .. 100 ]
    # continue if outline.pd is ''
    echo "<symbol overflow='visible' id='b#{gid}'>#{outline.br}</symbol>"
    echo "<symbol overflow='visible' id='g#{gid}'><path d='#{outline.pd}'/></symbol>"
  echo "</defs>"
  #.........................................................................................................
  for d in arrangement
    echo "<use href='#g#{d.gid}' x='#{d.x}' y='#{d.y}'/>"
    echo "<use href='#b#{d.gid}' x='#{d.x}' y='#{d.y}'/>"
    # echo "<g x='#{d.x}' y='#{d.y + 1000}'>"
    # echo "#{outline.br}"
    # echo "</g>"
  #.........................................................................................................
  echo "</svg>"
  return null

#-----------------------------------------------------------------------------------------------------------
@demo_text_wrapping_advanced = ->
  me        = @new_demo()
  ### TAINT use triplets `[m,w,p,]` (material width, whitespace width, penalty width) instead to make JSON significantly smaller ###
  slabs     = [
    { width: 5,   whitespace_width: 1, penalty_width: 1, },
    { width: 3,   whitespace_width: 1, penalty_width: 1, },
    { width: 4,   whitespace_width: 1, penalty_width: 1, },
    { width: 2,   whitespace_width: 1, penalty_width: 1, },
    { width: 5,   whitespace_width: 1, penalty_width: 1, },
    { width: 10,  whitespace_width: 1, penalty_width: 1, }, ];
  slablines = JSON.parse RBW.wrap_text_with_arbitrary_slabs slabs
  debug '^3334^', rpr slablines
  for slabline in slablines.lines
    info slabline
  return null

#-----------------------------------------------------------------------------------------------------------
@find_widths_from_slabs = ( me, slabs ) ->
  debug '^443^', slabs
  new_slabs = new Set()
  for slab_text, slab_idx in slabs.slabs
    end       = slabs.ends[ slab_idx ]
    slab_code = slab_text + end
    continue if me.slab_widths[ slab_code ]?
    switch end
      when '_'
        new_slabs.add slab_text # + ''
      when '|'
        new_slabs.add slab_text # + ''
        new_slabs.add slab_text + '-'
      when 'x'
        new_slabs.add slab_text # + ''
  debug '^3344^', new_slabs
  return null

#-----------------------------------------------------------------------------------------------------------
@get_font_metrics = ( me, font_idx ) ->
  ### TAINT cache font metrics ###
  ### TAINT validate arguments, results ###
  format      = 'json'
  text        = " - –"
  arrangement = JSON.parse RBW.shape_text { font_idx, text, format, }
  space       = arrangement[ 0 ]
  hyphen      = arrangement[ 1 ]
  endash      = arrangement[ 3 ]
  debug '^get_font_metrics@445^', arrangement
  return {
    space:  { gid: space.gid, dx: space.dx},
    hyphen: { gid: hyphen.gid, dx: hyphen.dx},
    endash: { gid: endash.gid, dx: endash.dx}, }

#-----------------------------------------------------------------------------------------------------------
@_firstchr = ( buffer, idx ) ->
  ### Return the first character found in buffer at given position, assuming UTF-8. ###
  return ( Array.from buffer[ idx ... idx + 4 ].toString() )[ 0 ]

#-----------------------------------------------------------------------------------------------------------
@_slice_buffer = ( buffer, start_idx, stop_idx ) ->
  return buffer[ start_idx ... stop_idx ].toString()

#-----------------------------------------------------------------------------------------------------------
@demo_typesetting = ->
  me        = @new_demo()
  whisper '^33443^ demo_typesetting'
  #.........................................................................................................
  text = """Knuth–Liang hyphenation operates at the level of individual words, but there can be ambiguity as
  to what constitutes a word. All hyphenation dictionaries handle the expected set of word-forming graphemes
  from their respective alphabets, but some also accept punctuation marks such as hyphens and apostrophes,
  and are thus capable of handling hyphen-joined compound words or elisions. Even so, it's generally
  preferable to handle punctuation at the level of segmentation, as it affords greater control over the
  final result (such as where to break hyphen-joined compounds, or whether to set a leading hyphen on new
  lines).
  在文本的显示中， 换行 （line wrap）是指文本在一行已满的情况下转到新行，使得每一行都能在窗口范围看到，不需要任何水平的滚动。 自动换行 （word wrap） 是 大 多 数 文 字 編 輯 器 、 文書處理器、和网页浏览器的一个附加功能。它用于在行间或一行里的单词间隔处分行，不考虑一个单词超过一行长度的情况。
  """
  # text          = "Knuth–Liang hyphenation" ## en-dash U+2013 ###
  # text          = "Knuth-Liang hyphenation" ### hyphen-minus U+002d ###
  text          = "今日も明日も。"
  # text          = "The elaborate sphinx told me a riddle, told me a riddle, told me a riddle."
  text          = "the affixation"
  # text          = "affix"
  #.........................................................................................................
  # fontnick      = 'notoserif'
  fontnick      = 'garamond_italic'
  font_idx      = @register_font me, fontnick
  debug '^222332^', fm = @get_font_metrics me, font_idx
  #.........................................................................................................
  text          = text.replace /\s+/g, ' '
  text          = INTERTEXT.HYPH.hyphenate text
  text_bfr      = Buffer.from text, { encoding: 'utf-8', }
  lbos          = JSON.parse RBW.find_line_break_positions text
  #.........................................................................................................
  # urge '^454-1^', @_slice_buffer text_bfr, 0, 3
  for idx in [ 0 ... lbos.length - 1 ]
    lbo =lbos[ idx ]
    chunk = @_slice_buffer text_bfr, lbo, lbos[ idx + 1 ]
    chunk = chunk.replace /\xad/g, '|'
    urge '^454-1^', lbo, rpr chunk
  arrangement   = JSON.parse RBW.shape_text { font_idx, text, format: 'json', }
  #.........................................................................................................
  segment_gids  = [ fm.space.gid, fm.hyphen.gid, fm.endash.gid, ]
  slab_idx = 0
  for glyfpos in arrangement
    if ( glyfpos.gid in segment_gids )
      if ( glyfpos.dx is 0 )
        info '^3336^', ( CND.reverse CND.red glyfpos ), ( CND.lime rpr @_firstchr text_bfr, glyfpos.bidx )
      else
        info '^3336^', ( CND.reverse CND.yellow glyfpos ), ( CND.lime rpr @_firstchr text_bfr, glyfpos.bidx )
      slab_idx++
    else
      info '^3336^', glyfpos, ( CND.lime rpr @_firstchr text_bfr, glyfpos.bidx )
  return null
  #.........................................................................................................
  #.........................................................................................................
  #.........................................................................................................
  #.........................................................................................................
  width         = 10
  lines         = RBW.wrap_text text, width
  debug '^3383^', lines
  lines         = lines.split '\n'
  last_line_idx = lines.length - 1
  debug '^449^', lines
  for line, line_idx in lines
    # debug '^499^', words
    if line_idx < last_line_idx
      line_length   = line.length
      words         = line.split /\s+/
      last_word_idx = words.length - 1
      loop
        break if last_word_idx < 1
        break if line_length >= width
        for word_idx in [ 0 ... last_word_idx ]
          # debug word_idx
          break if line_length >= width
          continue unless Math.random() > 0.5
          line_length++
          words[ word_idx ] += ' '
      info words.join ' '
    else
      info line
  #.........................................................................................................
  slabs     = [
    { width: 5,   whitespace_width: 1, penalty_width: 1, },
    { width: 3,   whitespace_width: 1, penalty_width: 1, },
    { width: 4,   whitespace_width: 1, penalty_width: 1, },
    { width: 2,   whitespace_width: 1, penalty_width: 1, },
    { width: 5,   whitespace_width: 1, penalty_width: 1, },
    { width: 10,  whitespace_width: 1, penalty_width: 1, }, ];
  slablines = JSON.parse RBW.wrap_text_with_arbitrary_slabs slabs
  debug '^3334^', rpr slablines
  for slabline in slablines.lines
    info slabline
  return null


############################################################################################################
if module is require.main then do =>
  # @demo_text_shaping()
  # @demo_svg_typesetting()
  # @demo_text_wrapping()
  # @demo_text_wrapping_advanced()
  @demo_typesetting()
  return null



