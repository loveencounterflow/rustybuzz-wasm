


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
  R = me._prv_fontidx += 1
  RBW.register_font R, @_get_font_bytes me, font_entry.path
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
  whisper '^33443^ demo_text_wrapping'
  me        = @new_demo()
  text = """Knuth–Liang hyphenation operates at the level of individual words, but there can be ambiguity as
  to what constitutes a word. All hyphenation dictionaries handle the expected set of word-forming graphemes
  from their respective alphabets, but some also accept punctuation marks such as hyphens and apostrophes,
  and are thus capable of handling hyphen-joined compound words or elisions. Even so, it's generally
  preferable to handle punctuation at the level of segmentation, as it affords greater control over the
  final result (such as where to break hyphen-joined compounds, or whether to set a leading hyphen on new
  lines).
  在文本的显示中， 换行 （line wrap）是指文本在一行已满的情况下转到新行，使得每一行都能在窗口范围看到，不需要任何水平的滚动。 自动换行 （word wrap） 是 大 多 数 文 字 編 輯 器 、 文書處理器、和网页浏览器的一个附加功能。它用于在行间或一行里的单词间隔处分行，不考虑一个单词超过一行长度的情况。
  """
  text          = "The ela#bo#ra#te sphinx told me a rid#dle."
  # text          = "The elaborate sphinx told me a riddle."
  #.........................................................................................................
  text          = text.replace /#/g, me.shy
  text          = text.replace /\s+/g, ' '
  width         = 10
  lines         = RBW.wrap_text text, width
  debug '^3383^', lines
  return null
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
  return null

#-----------------------------------------------------------------------------------------------------------
@demo_text_wrapping_advanced = ->
  me        = @new_demo()
  RBW.wrap_text_with_arbitrary_slabs()


############################################################################################################
if module is require.main then do =>
  # @demo_text_shaping()
  # @demo_svg_typesetting()
  # @demo_text_wrapping()
  @demo_text_wrapping_advanced()
  return null
