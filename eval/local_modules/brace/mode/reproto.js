ace.define("ace/mode/reproto_highlight_rules",["require","exports","module","ace/lib/oop","ace/mode/text_highlight_rules"], function(acequire, exports, module) {
"use strict";

var oop = acequire("../lib/oop");
var TextHighlightRules = acequire("./text_highlight_rules").TextHighlightRules;

var stringEscape = /\\(?:[nrt0'"\\]|x[\da-fA-F]{2}|u\{[\da-fA-F]{6}\})/.source;

var reprotoHighlightRules = function() {
  this.$rules = { start: [
    { token: 'variable.other.reproto',
      regex: '\\b[A-Z][a-zA-Z0-9_]*\\b' },
    { token: 'string.quoted.double.reproto',
      regex: '"',
      push: [
        { token: 'string.quoted.double.reproto',
          regex: '"',
          next: 'pop' },
        { token: 'constant.character.escape.reproto',
          regex: stringEscape },
        { defaultToken: 'string.quoted.double.reproto' }
      ]
    },
    { token: 'keyword.reproto',
      regex: '\\b(?:type|interface|enum|tuple|service|use|as)\\b' },
    { token: 'comment.block.documentation.reproto',
      regex: '(//!.*|///.*)$' },
    { token: 'comment.line.double-slash.reproto',
      regex: '//.*$' },
    { token: 'storage.type.reproto',
      regex: '\\b(any|float|double|boolean|string|bytes|datetime|u32|u64|i32|i64)\\b' },
    { token: 'storage.modifier.reproto',
      regex: '\\b(?:stream)\\b' },
    { token : "paren.lparen", regex : /[\[({]/ },
    { token : "paren.rparen", regex : /[\])}]/ },
    { token: 'constant.language.reproto',
      regex: '\\b(true|false)\\b' },
    { token: 'constant.numeric.reproto',
      regex: /\b([0-9][0-9_]*)(?:\.[0-9][0-9_]*)?(?:[Ee][+-][0-9][0-9_]*)?\b/ }
  ] };

  this.normalizeRules();
};

reprotoHighlightRules.metaData = {
  fileTypes: [ 'reproto' ],
  name: 'reproto',
  scopeName: 'source.reproto'
};

oop.inherits(reprotoHighlightRules, TextHighlightRules);

exports.reprotoHighlightRules = reprotoHighlightRules;
});

ace.define("ace/mode/reproto",["require","exports","module","ace/lib/oop","ace/mode/text","ace/mode/reproto_highlight_rules"], function(acequire, exports, module) {
"use strict";

var oop = acequire("../lib/oop");
var TextMode = acequire("./text").Mode;
var reprotoHighlightRules = acequire("./reproto_highlight_rules").reprotoHighlightRules;

var Mode = function() {
    this.HighlightRules = reprotoHighlightRules;
    this.$behaviour = this.$defaultBehaviour;
};

oop.inherits(Mode, TextMode);

(function() {
    this.lineCommentStart = "//";
    this.$quotes = { '"': '"' };
    this.$id = "ace/mode/reproto";
}).call(Mode.prototype);

exports.Mode = Mode;
});
