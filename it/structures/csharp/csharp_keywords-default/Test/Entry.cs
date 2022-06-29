using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Entry {
        [JsonProperty("abstract")]
        public String _abstract {
            get;
        }

        [JsonProperty("as")]
        public String _as {
            get;
        }

        [JsonProperty("base")]
        public String _base {
            get;
        }

        [JsonProperty("bool")]
        public String _bool {
            get;
        }

        [JsonProperty("break")]
        public String _break {
            get;
        }

        [JsonProperty("byte")]
        public String _byte {
            get;
        }

        [JsonProperty("case")]
        public String _case {
            get;
        }

        [JsonProperty("catch")]
        public String _catch {
            get;
        }

        [JsonProperty("char")]
        public String _char {
            get;
        }

        [JsonProperty("checked")]
        public String _checked {
            get;
        }

        [JsonProperty("class")]
        public String _class {
            get;
        }

        [JsonProperty("const")]
        public String _const {
            get;
        }

        [JsonProperty("continue")]
        public String _continue {
            get;
        }

        [JsonProperty("decimal")]
        public String _decimal {
            get;
        }

        [JsonProperty("default")]
        public String _default {
            get;
        }

        [JsonProperty("delegate")]
        public String _delegate {
            get;
        }

        [JsonProperty("do")]
        public String _do {
            get;
        }

        [JsonProperty("double")]
        public String _double {
            get;
        }

        [JsonProperty("else")]
        public String _else {
            get;
        }

        [JsonProperty("enum")]
        public String _enum {
            get;
        }

        [JsonProperty("event")]
        public String _event {
            get;
        }

        [JsonProperty("explicit")]
        public String _explicit {
            get;
        }

        [JsonProperty("extern")]
        public String _extern {
            get;
        }

        [JsonProperty("false")]
        public String _false {
            get;
        }

        [JsonProperty("finally")]
        public String _finally {
            get;
        }

        [JsonProperty("fixed")]
        public String _fixed {
            get;
        }

        [JsonProperty("float")]
        public String _float {
            get;
        }

        [JsonProperty("for")]
        public String _for {
            get;
        }

        [JsonProperty("foreach")]
        public String _foreach {
            get;
        }

        [JsonProperty("goto")]
        public String _goto {
            get;
        }

        [JsonProperty("if")]
        public String _if {
            get;
        }

        [JsonProperty("implicit")]
        public String _implicit {
            get;
        }

        [JsonProperty("in")]
        public String _in {
            get;
        }

        [JsonProperty("int")]
        public String _int {
            get;
        }

        [JsonProperty("interface")]
        public String _interface {
            get;
        }

        [JsonProperty("internal")]
        public String _internal {
            get;
        }

        [JsonProperty("is")]
        public String _is {
            get;
        }

        [JsonProperty("lock")]
        public String _lock {
            get;
        }

        [JsonProperty("long")]
        public String _long {
            get;
        }

        [JsonProperty("namespace")]
        public String _namespace {
            get;
        }

        [JsonProperty("new")]
        public String _new {
            get;
        }

        [JsonProperty("null")]
        public String _null {
            get;
        }

        [JsonProperty("object")]
        public String _object {
            get;
        }

        [JsonProperty("operator")]
        public String _operator {
            get;
        }

        [JsonProperty("out")]
        public String _out {
            get;
        }

        [JsonProperty("override")]
        public String _override {
            get;
        }

        [JsonProperty("params")]
        public String _params {
            get;
        }

        [JsonProperty("private")]
        public String _private {
            get;
        }

        [JsonProperty("protected")]
        public String _protected {
            get;
        }

        [JsonProperty("public")]
        public String _public {
            get;
        }

        [JsonProperty("readonly")]
        public String _readonly {
            get;
        }

        [JsonProperty("ref")]
        public String _ref {
            get;
        }

        [JsonProperty("return")]
        public String _return {
            get;
        }

        [JsonProperty("sbyte")]
        public String _sbyte {
            get;
        }

        [JsonProperty("sealed")]
        public String _sealed {
            get;
        }

        [JsonProperty("short")]
        public String _short {
            get;
        }

        [JsonProperty("sizeof")]
        public String _sizeof {
            get;
        }

        [JsonProperty("stackalloc")]
        public String _stackalloc {
            get;
        }

        [JsonProperty("static")]
        public String _static {
            get;
        }

        [JsonProperty("string")]
        public String _string {
            get;
        }

        [JsonProperty("struct")]
        public String _struct {
            get;
        }

        [JsonProperty("switch")]
        public String _switch {
            get;
        }

        [JsonProperty("this")]
        public String _this {
            get;
        }

        [JsonProperty("throw")]
        public String _throw {
            get;
        }

        [JsonProperty("true")]
        public String _true {
            get;
        }

        [JsonProperty("try")]
        public String _try {
            get;
        }

        [JsonProperty("typeof")]
        public String _typeof {
            get;
        }

        [JsonProperty("uint")]
        public String _uint {
            get;
        }

        [JsonProperty("ulong")]
        public String _ulong {
            get;
        }

        [JsonProperty("unchecked")]
        public String _unchecked {
            get;
        }

        [JsonProperty("unsafe")]
        public String _unsafe {
            get;
        }

        [JsonProperty("ushort")]
        public String _ushort {
            get;
        }

        [JsonProperty("using")]
        public String _using {
            get;
        }

        [JsonProperty("virtual")]
        public String _virtual {
            get;
        }

        [JsonProperty("void")]
        public String _void {
            get;
        }

        [JsonProperty("volatile")]
        public String _volatile {
            get;
        }

        [JsonProperty("while")]
        public String _while {
            get;
        }

        [JsonConstructor]
        public Entry (
            [JsonProperty("abstract")] String _abstract,
            [JsonProperty("as")] String _as,
            [JsonProperty("base")] String _base,
            [JsonProperty("bool")] String _bool,
            [JsonProperty("break")] String _break,
            [JsonProperty("byte")] String _byte,
            [JsonProperty("case")] String _case,
            [JsonProperty("catch")] String _catch,
            [JsonProperty("char")] String _char,
            [JsonProperty("checked")] String _checked,
            [JsonProperty("class")] String _class,
            [JsonProperty("const")] String _const,
            [JsonProperty("continue")] String _continue,
            [JsonProperty("decimal")] String _decimal,
            [JsonProperty("default")] String _default,
            [JsonProperty("delegate")] String _delegate,
            [JsonProperty("do")] String _do,
            [JsonProperty("double")] String _double,
            [JsonProperty("else")] String _else,
            [JsonProperty("enum")] String _enum,
            [JsonProperty("event")] String _event,
            [JsonProperty("explicit")] String _explicit,
            [JsonProperty("extern")] String _extern,
            [JsonProperty("false")] String _false,
            [JsonProperty("finally")] String _finally,
            [JsonProperty("fixed")] String _fixed,
            [JsonProperty("float")] String _float,
            [JsonProperty("for")] String _for,
            [JsonProperty("foreach")] String _foreach,
            [JsonProperty("goto")] String _goto,
            [JsonProperty("if")] String _if,
            [JsonProperty("implicit")] String _implicit,
            [JsonProperty("in")] String _in,
            [JsonProperty("int")] String _int,
            [JsonProperty("interface")] String _interface,
            [JsonProperty("internal")] String _internal,
            [JsonProperty("is")] String _is,
            [JsonProperty("lock")] String _lock,
            [JsonProperty("long")] String _long,
            [JsonProperty("namespace")] String _namespace,
            [JsonProperty("new")] String _new,
            [JsonProperty("null")] String _null,
            [JsonProperty("object")] String _object,
            [JsonProperty("operator")] String _operator,
            [JsonProperty("out")] String _out,
            [JsonProperty("override")] String _override,
            [JsonProperty("params")] String _params,
            [JsonProperty("private")] String _private,
            [JsonProperty("protected")] String _protected,
            [JsonProperty("public")] String _public,
            [JsonProperty("readonly")] String _readonly,
            [JsonProperty("ref")] String _ref,
            [JsonProperty("return")] String _return,
            [JsonProperty("sbyte")] String _sbyte,
            [JsonProperty("sealed")] String _sealed,
            [JsonProperty("short")] String _short,
            [JsonProperty("sizeof")] String _sizeof,
            [JsonProperty("stackalloc")] String _stackalloc,
            [JsonProperty("static")] String _static,
            [JsonProperty("string")] String _string,
            [JsonProperty("struct")] String _struct,
            [JsonProperty("switch")] String _switch,
            [JsonProperty("this")] String _this,
            [JsonProperty("throw")] String _throw,
            [JsonProperty("true")] String _true,
            [JsonProperty("try")] String _try,
            [JsonProperty("typeof")] String _typeof,
            [JsonProperty("uint")] String _uint,
            [JsonProperty("ulong")] String _ulong,
            [JsonProperty("unchecked")] String _unchecked,
            [JsonProperty("unsafe")] String _unsafe,
            [JsonProperty("ushort")] String _ushort,
            [JsonProperty("using")] String _using,
            [JsonProperty("virtual")] String _virtual,
            [JsonProperty("void")] String _void,
            [JsonProperty("volatile")] String _volatile,
            [JsonProperty("while")] String _while
        ) {
            this._abstract = _abstract;
            this._as = _as;
            this._base = _base;
            this._bool = _bool;
            this._break = _break;
            this._byte = _byte;
            this._case = _case;
            this._catch = _catch;
            this._char = _char;
            this._checked = _checked;
            this._class = _class;
            this._const = _const;
            this._continue = _continue;
            this._decimal = _decimal;
            this._default = _default;
            this._delegate = _delegate;
            this._do = _do;
            this._double = _double;
            this._else = _else;
            this._enum = _enum;
            this._event = _event;
            this._explicit = _explicit;
            this._extern = _extern;
            this._false = _false;
            this._finally = _finally;
            this._fixed = _fixed;
            this._float = _float;
            this._for = _for;
            this._foreach = _foreach;
            this._goto = _goto;
            this._if = _if;
            this._implicit = _implicit;
            this._in = _in;
            this._int = _int;
            this._interface = _interface;
            this._internal = _internal;
            this._is = _is;
            this._lock = _lock;
            this._long = _long;
            this._namespace = _namespace;
            this._new = _new;
            this._null = _null;
            this._object = _object;
            this._operator = _operator;
            this._out = _out;
            this._override = _override;
            this._params = _params;
            this._private = _private;
            this._protected = _protected;
            this._public = _public;
            this._readonly = _readonly;
            this._ref = _ref;
            this._return = _return;
            this._sbyte = _sbyte;
            this._sealed = _sealed;
            this._short = _short;
            this._sizeof = _sizeof;
            this._stackalloc = _stackalloc;
            this._static = _static;
            this._string = _string;
            this._struct = _struct;
            this._switch = _switch;
            this._this = _this;
            this._throw = _throw;
            this._true = _true;
            this._try = _try;
            this._typeof = _typeof;
            this._uint = _uint;
            this._ulong = _ulong;
            this._unchecked = _unchecked;
            this._unsafe = _unsafe;
            this._ushort = _ushort;
            this._using = _using;
            this._virtual = _virtual;
            this._void = _void;
            this._volatile = _volatile;
            this._while = _while;
        }

        public override bool Equals(Object other) {
            Entry o = other as Entry;

            if (o == null) {
                return false;
            }

            if (this._abstract == null) {
                if (o._abstract != null) {
                    return false;
                }
            } else {
                if (!this._abstract.Equals(o._abstract)) {
                    return false;
                }
            }

            if (this._as == null) {
                if (o._as != null) {
                    return false;
                }
            } else {
                if (!this._as.Equals(o._as)) {
                    return false;
                }
            }

            if (this._base == null) {
                if (o._base != null) {
                    return false;
                }
            } else {
                if (!this._base.Equals(o._base)) {
                    return false;
                }
            }

            if (this._bool == null) {
                if (o._bool != null) {
                    return false;
                }
            } else {
                if (!this._bool.Equals(o._bool)) {
                    return false;
                }
            }

            if (this._break == null) {
                if (o._break != null) {
                    return false;
                }
            } else {
                if (!this._break.Equals(o._break)) {
                    return false;
                }
            }

            if (this._byte == null) {
                if (o._byte != null) {
                    return false;
                }
            } else {
                if (!this._byte.Equals(o._byte)) {
                    return false;
                }
            }

            if (this._case == null) {
                if (o._case != null) {
                    return false;
                }
            } else {
                if (!this._case.Equals(o._case)) {
                    return false;
                }
            }

            if (this._catch == null) {
                if (o._catch != null) {
                    return false;
                }
            } else {
                if (!this._catch.Equals(o._catch)) {
                    return false;
                }
            }

            if (this._char == null) {
                if (o._char != null) {
                    return false;
                }
            } else {
                if (!this._char.Equals(o._char)) {
                    return false;
                }
            }

            if (this._checked == null) {
                if (o._checked != null) {
                    return false;
                }
            } else {
                if (!this._checked.Equals(o._checked)) {
                    return false;
                }
            }

            if (this._class == null) {
                if (o._class != null) {
                    return false;
                }
            } else {
                if (!this._class.Equals(o._class)) {
                    return false;
                }
            }

            if (this._const == null) {
                if (o._const != null) {
                    return false;
                }
            } else {
                if (!this._const.Equals(o._const)) {
                    return false;
                }
            }

            if (this._continue == null) {
                if (o._continue != null) {
                    return false;
                }
            } else {
                if (!this._continue.Equals(o._continue)) {
                    return false;
                }
            }

            if (this._decimal == null) {
                if (o._decimal != null) {
                    return false;
                }
            } else {
                if (!this._decimal.Equals(o._decimal)) {
                    return false;
                }
            }

            if (this._default == null) {
                if (o._default != null) {
                    return false;
                }
            } else {
                if (!this._default.Equals(o._default)) {
                    return false;
                }
            }

            if (this._delegate == null) {
                if (o._delegate != null) {
                    return false;
                }
            } else {
                if (!this._delegate.Equals(o._delegate)) {
                    return false;
                }
            }

            if (this._do == null) {
                if (o._do != null) {
                    return false;
                }
            } else {
                if (!this._do.Equals(o._do)) {
                    return false;
                }
            }

            if (this._double == null) {
                if (o._double != null) {
                    return false;
                }
            } else {
                if (!this._double.Equals(o._double)) {
                    return false;
                }
            }

            if (this._else == null) {
                if (o._else != null) {
                    return false;
                }
            } else {
                if (!this._else.Equals(o._else)) {
                    return false;
                }
            }

            if (this._enum == null) {
                if (o._enum != null) {
                    return false;
                }
            } else {
                if (!this._enum.Equals(o._enum)) {
                    return false;
                }
            }

            if (this._event == null) {
                if (o._event != null) {
                    return false;
                }
            } else {
                if (!this._event.Equals(o._event)) {
                    return false;
                }
            }

            if (this._explicit == null) {
                if (o._explicit != null) {
                    return false;
                }
            } else {
                if (!this._explicit.Equals(o._explicit)) {
                    return false;
                }
            }

            if (this._extern == null) {
                if (o._extern != null) {
                    return false;
                }
            } else {
                if (!this._extern.Equals(o._extern)) {
                    return false;
                }
            }

            if (this._false == null) {
                if (o._false != null) {
                    return false;
                }
            } else {
                if (!this._false.Equals(o._false)) {
                    return false;
                }
            }

            if (this._finally == null) {
                if (o._finally != null) {
                    return false;
                }
            } else {
                if (!this._finally.Equals(o._finally)) {
                    return false;
                }
            }

            if (this._fixed == null) {
                if (o._fixed != null) {
                    return false;
                }
            } else {
                if (!this._fixed.Equals(o._fixed)) {
                    return false;
                }
            }

            if (this._float == null) {
                if (o._float != null) {
                    return false;
                }
            } else {
                if (!this._float.Equals(o._float)) {
                    return false;
                }
            }

            if (this._for == null) {
                if (o._for != null) {
                    return false;
                }
            } else {
                if (!this._for.Equals(o._for)) {
                    return false;
                }
            }

            if (this._foreach == null) {
                if (o._foreach != null) {
                    return false;
                }
            } else {
                if (!this._foreach.Equals(o._foreach)) {
                    return false;
                }
            }

            if (this._goto == null) {
                if (o._goto != null) {
                    return false;
                }
            } else {
                if (!this._goto.Equals(o._goto)) {
                    return false;
                }
            }

            if (this._if == null) {
                if (o._if != null) {
                    return false;
                }
            } else {
                if (!this._if.Equals(o._if)) {
                    return false;
                }
            }

            if (this._implicit == null) {
                if (o._implicit != null) {
                    return false;
                }
            } else {
                if (!this._implicit.Equals(o._implicit)) {
                    return false;
                }
            }

            if (this._in == null) {
                if (o._in != null) {
                    return false;
                }
            } else {
                if (!this._in.Equals(o._in)) {
                    return false;
                }
            }

            if (this._int == null) {
                if (o._int != null) {
                    return false;
                }
            } else {
                if (!this._int.Equals(o._int)) {
                    return false;
                }
            }

            if (this._interface == null) {
                if (o._interface != null) {
                    return false;
                }
            } else {
                if (!this._interface.Equals(o._interface)) {
                    return false;
                }
            }

            if (this._internal == null) {
                if (o._internal != null) {
                    return false;
                }
            } else {
                if (!this._internal.Equals(o._internal)) {
                    return false;
                }
            }

            if (this._is == null) {
                if (o._is != null) {
                    return false;
                }
            } else {
                if (!this._is.Equals(o._is)) {
                    return false;
                }
            }

            if (this._lock == null) {
                if (o._lock != null) {
                    return false;
                }
            } else {
                if (!this._lock.Equals(o._lock)) {
                    return false;
                }
            }

            if (this._long == null) {
                if (o._long != null) {
                    return false;
                }
            } else {
                if (!this._long.Equals(o._long)) {
                    return false;
                }
            }

            if (this._namespace == null) {
                if (o._namespace != null) {
                    return false;
                }
            } else {
                if (!this._namespace.Equals(o._namespace)) {
                    return false;
                }
            }

            if (this._new == null) {
                if (o._new != null) {
                    return false;
                }
            } else {
                if (!this._new.Equals(o._new)) {
                    return false;
                }
            }

            if (this._null == null) {
                if (o._null != null) {
                    return false;
                }
            } else {
                if (!this._null.Equals(o._null)) {
                    return false;
                }
            }

            if (this._object == null) {
                if (o._object != null) {
                    return false;
                }
            } else {
                if (!this._object.Equals(o._object)) {
                    return false;
                }
            }

            if (this._operator == null) {
                if (o._operator != null) {
                    return false;
                }
            } else {
                if (!this._operator.Equals(o._operator)) {
                    return false;
                }
            }

            if (this._out == null) {
                if (o._out != null) {
                    return false;
                }
            } else {
                if (!this._out.Equals(o._out)) {
                    return false;
                }
            }

            if (this._override == null) {
                if (o._override != null) {
                    return false;
                }
            } else {
                if (!this._override.Equals(o._override)) {
                    return false;
                }
            }

            if (this._params == null) {
                if (o._params != null) {
                    return false;
                }
            } else {
                if (!this._params.Equals(o._params)) {
                    return false;
                }
            }

            if (this._private == null) {
                if (o._private != null) {
                    return false;
                }
            } else {
                if (!this._private.Equals(o._private)) {
                    return false;
                }
            }

            if (this._protected == null) {
                if (o._protected != null) {
                    return false;
                }
            } else {
                if (!this._protected.Equals(o._protected)) {
                    return false;
                }
            }

            if (this._public == null) {
                if (o._public != null) {
                    return false;
                }
            } else {
                if (!this._public.Equals(o._public)) {
                    return false;
                }
            }

            if (this._readonly == null) {
                if (o._readonly != null) {
                    return false;
                }
            } else {
                if (!this._readonly.Equals(o._readonly)) {
                    return false;
                }
            }

            if (this._ref == null) {
                if (o._ref != null) {
                    return false;
                }
            } else {
                if (!this._ref.Equals(o._ref)) {
                    return false;
                }
            }

            if (this._return == null) {
                if (o._return != null) {
                    return false;
                }
            } else {
                if (!this._return.Equals(o._return)) {
                    return false;
                }
            }

            if (this._sbyte == null) {
                if (o._sbyte != null) {
                    return false;
                }
            } else {
                if (!this._sbyte.Equals(o._sbyte)) {
                    return false;
                }
            }

            if (this._sealed == null) {
                if (o._sealed != null) {
                    return false;
                }
            } else {
                if (!this._sealed.Equals(o._sealed)) {
                    return false;
                }
            }

            if (this._short == null) {
                if (o._short != null) {
                    return false;
                }
            } else {
                if (!this._short.Equals(o._short)) {
                    return false;
                }
            }

            if (this._sizeof == null) {
                if (o._sizeof != null) {
                    return false;
                }
            } else {
                if (!this._sizeof.Equals(o._sizeof)) {
                    return false;
                }
            }

            if (this._stackalloc == null) {
                if (o._stackalloc != null) {
                    return false;
                }
            } else {
                if (!this._stackalloc.Equals(o._stackalloc)) {
                    return false;
                }
            }

            if (this._static == null) {
                if (o._static != null) {
                    return false;
                }
            } else {
                if (!this._static.Equals(o._static)) {
                    return false;
                }
            }

            if (this._string == null) {
                if (o._string != null) {
                    return false;
                }
            } else {
                if (!this._string.Equals(o._string)) {
                    return false;
                }
            }

            if (this._struct == null) {
                if (o._struct != null) {
                    return false;
                }
            } else {
                if (!this._struct.Equals(o._struct)) {
                    return false;
                }
            }

            if (this._switch == null) {
                if (o._switch != null) {
                    return false;
                }
            } else {
                if (!this._switch.Equals(o._switch)) {
                    return false;
                }
            }

            if (this._this == null) {
                if (o._this != null) {
                    return false;
                }
            } else {
                if (!this._this.Equals(o._this)) {
                    return false;
                }
            }

            if (this._throw == null) {
                if (o._throw != null) {
                    return false;
                }
            } else {
                if (!this._throw.Equals(o._throw)) {
                    return false;
                }
            }

            if (this._true == null) {
                if (o._true != null) {
                    return false;
                }
            } else {
                if (!this._true.Equals(o._true)) {
                    return false;
                }
            }

            if (this._try == null) {
                if (o._try != null) {
                    return false;
                }
            } else {
                if (!this._try.Equals(o._try)) {
                    return false;
                }
            }

            if (this._typeof == null) {
                if (o._typeof != null) {
                    return false;
                }
            } else {
                if (!this._typeof.Equals(o._typeof)) {
                    return false;
                }
            }

            if (this._uint == null) {
                if (o._uint != null) {
                    return false;
                }
            } else {
                if (!this._uint.Equals(o._uint)) {
                    return false;
                }
            }

            if (this._ulong == null) {
                if (o._ulong != null) {
                    return false;
                }
            } else {
                if (!this._ulong.Equals(o._ulong)) {
                    return false;
                }
            }

            if (this._unchecked == null) {
                if (o._unchecked != null) {
                    return false;
                }
            } else {
                if (!this._unchecked.Equals(o._unchecked)) {
                    return false;
                }
            }

            if (this._unsafe == null) {
                if (o._unsafe != null) {
                    return false;
                }
            } else {
                if (!this._unsafe.Equals(o._unsafe)) {
                    return false;
                }
            }

            if (this._ushort == null) {
                if (o._ushort != null) {
                    return false;
                }
            } else {
                if (!this._ushort.Equals(o._ushort)) {
                    return false;
                }
            }

            if (this._using == null) {
                if (o._using != null) {
                    return false;
                }
            } else {
                if (!this._using.Equals(o._using)) {
                    return false;
                }
            }

            if (this._virtual == null) {
                if (o._virtual != null) {
                    return false;
                }
            } else {
                if (!this._virtual.Equals(o._virtual)) {
                    return false;
                }
            }

            if (this._void == null) {
                if (o._void != null) {
                    return false;
                }
            } else {
                if (!this._void.Equals(o._void)) {
                    return false;
                }
            }

            if (this._volatile == null) {
                if (o._volatile != null) {
                    return false;
                }
            } else {
                if (!this._volatile.Equals(o._volatile)) {
                    return false;
                }
            }

            if (this._while == null) {
                if (o._while != null) {
                    return false;
                }
            } else {
                if (!this._while.Equals(o._while)) {
                    return false;
                }
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            result = result * 31 + this._abstract.GetHashCode();
            result = result * 31 + this._as.GetHashCode();
            result = result * 31 + this._base.GetHashCode();
            result = result * 31 + this._bool.GetHashCode();
            result = result * 31 + this._break.GetHashCode();
            result = result * 31 + this._byte.GetHashCode();
            result = result * 31 + this._case.GetHashCode();
            result = result * 31 + this._catch.GetHashCode();
            result = result * 31 + this._char.GetHashCode();
            result = result * 31 + this._checked.GetHashCode();
            result = result * 31 + this._class.GetHashCode();
            result = result * 31 + this._const.GetHashCode();
            result = result * 31 + this._continue.GetHashCode();
            result = result * 31 + this._decimal.GetHashCode();
            result = result * 31 + this._default.GetHashCode();
            result = result * 31 + this._delegate.GetHashCode();
            result = result * 31 + this._do.GetHashCode();
            result = result * 31 + this._double.GetHashCode();
            result = result * 31 + this._else.GetHashCode();
            result = result * 31 + this._enum.GetHashCode();
            result = result * 31 + this._event.GetHashCode();
            result = result * 31 + this._explicit.GetHashCode();
            result = result * 31 + this._extern.GetHashCode();
            result = result * 31 + this._false.GetHashCode();
            result = result * 31 + this._finally.GetHashCode();
            result = result * 31 + this._fixed.GetHashCode();
            result = result * 31 + this._float.GetHashCode();
            result = result * 31 + this._for.GetHashCode();
            result = result * 31 + this._foreach.GetHashCode();
            result = result * 31 + this._goto.GetHashCode();
            result = result * 31 + this._if.GetHashCode();
            result = result * 31 + this._implicit.GetHashCode();
            result = result * 31 + this._in.GetHashCode();
            result = result * 31 + this._int.GetHashCode();
            result = result * 31 + this._interface.GetHashCode();
            result = result * 31 + this._internal.GetHashCode();
            result = result * 31 + this._is.GetHashCode();
            result = result * 31 + this._lock.GetHashCode();
            result = result * 31 + this._long.GetHashCode();
            result = result * 31 + this._namespace.GetHashCode();
            result = result * 31 + this._new.GetHashCode();
            result = result * 31 + this._null.GetHashCode();
            result = result * 31 + this._object.GetHashCode();
            result = result * 31 + this._operator.GetHashCode();
            result = result * 31 + this._out.GetHashCode();
            result = result * 31 + this._override.GetHashCode();
            result = result * 31 + this._params.GetHashCode();
            result = result * 31 + this._private.GetHashCode();
            result = result * 31 + this._protected.GetHashCode();
            result = result * 31 + this._public.GetHashCode();
            result = result * 31 + this._readonly.GetHashCode();
            result = result * 31 + this._ref.GetHashCode();
            result = result * 31 + this._return.GetHashCode();
            result = result * 31 + this._sbyte.GetHashCode();
            result = result * 31 + this._sealed.GetHashCode();
            result = result * 31 + this._short.GetHashCode();
            result = result * 31 + this._sizeof.GetHashCode();
            result = result * 31 + this._stackalloc.GetHashCode();
            result = result * 31 + this._static.GetHashCode();
            result = result * 31 + this._string.GetHashCode();
            result = result * 31 + this._struct.GetHashCode();
            result = result * 31 + this._switch.GetHashCode();
            result = result * 31 + this._this.GetHashCode();
            result = result * 31 + this._throw.GetHashCode();
            result = result * 31 + this._true.GetHashCode();
            result = result * 31 + this._try.GetHashCode();
            result = result * 31 + this._typeof.GetHashCode();
            result = result * 31 + this._uint.GetHashCode();
            result = result * 31 + this._ulong.GetHashCode();
            result = result * 31 + this._unchecked.GetHashCode();
            result = result * 31 + this._unsafe.GetHashCode();
            result = result * 31 + this._ushort.GetHashCode();
            result = result * 31 + this._using.GetHashCode();
            result = result * 31 + this._virtual.GetHashCode();
            result = result * 31 + this._void.GetHashCode();
            result = result * 31 + this._volatile.GetHashCode();
            result = result * 31 + this._while.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Entry(");
            b.Append("abstract=");
            b.Append(this._abstract);
            b.Append(", ");
            b.Append("as=");
            b.Append(this._as);
            b.Append(", ");
            b.Append("base=");
            b.Append(this._base);
            b.Append(", ");
            b.Append("bool=");
            b.Append(this._bool);
            b.Append(", ");
            b.Append("break=");
            b.Append(this._break);
            b.Append(", ");
            b.Append("byte=");
            b.Append(this._byte);
            b.Append(", ");
            b.Append("case=");
            b.Append(this._case);
            b.Append(", ");
            b.Append("catch=");
            b.Append(this._catch);
            b.Append(", ");
            b.Append("char=");
            b.Append(this._char);
            b.Append(", ");
            b.Append("checked=");
            b.Append(this._checked);
            b.Append(", ");
            b.Append("class=");
            b.Append(this._class);
            b.Append(", ");
            b.Append("const=");
            b.Append(this._const);
            b.Append(", ");
            b.Append("continue=");
            b.Append(this._continue);
            b.Append(", ");
            b.Append("decimal=");
            b.Append(this._decimal);
            b.Append(", ");
            b.Append("default=");
            b.Append(this._default);
            b.Append(", ");
            b.Append("delegate=");
            b.Append(this._delegate);
            b.Append(", ");
            b.Append("do=");
            b.Append(this._do);
            b.Append(", ");
            b.Append("double=");
            b.Append(this._double);
            b.Append(", ");
            b.Append("else=");
            b.Append(this._else);
            b.Append(", ");
            b.Append("enum=");
            b.Append(this._enum);
            b.Append(", ");
            b.Append("event=");
            b.Append(this._event);
            b.Append(", ");
            b.Append("explicit=");
            b.Append(this._explicit);
            b.Append(", ");
            b.Append("extern=");
            b.Append(this._extern);
            b.Append(", ");
            b.Append("false=");
            b.Append(this._false);
            b.Append(", ");
            b.Append("finally=");
            b.Append(this._finally);
            b.Append(", ");
            b.Append("fixed=");
            b.Append(this._fixed);
            b.Append(", ");
            b.Append("float=");
            b.Append(this._float);
            b.Append(", ");
            b.Append("for=");
            b.Append(this._for);
            b.Append(", ");
            b.Append("foreach=");
            b.Append(this._foreach);
            b.Append(", ");
            b.Append("goto=");
            b.Append(this._goto);
            b.Append(", ");
            b.Append("if=");
            b.Append(this._if);
            b.Append(", ");
            b.Append("implicit=");
            b.Append(this._implicit);
            b.Append(", ");
            b.Append("in=");
            b.Append(this._in);
            b.Append(", ");
            b.Append("int=");
            b.Append(this._int);
            b.Append(", ");
            b.Append("interface=");
            b.Append(this._interface);
            b.Append(", ");
            b.Append("internal=");
            b.Append(this._internal);
            b.Append(", ");
            b.Append("is=");
            b.Append(this._is);
            b.Append(", ");
            b.Append("lock=");
            b.Append(this._lock);
            b.Append(", ");
            b.Append("long=");
            b.Append(this._long);
            b.Append(", ");
            b.Append("namespace=");
            b.Append(this._namespace);
            b.Append(", ");
            b.Append("new=");
            b.Append(this._new);
            b.Append(", ");
            b.Append("null=");
            b.Append(this._null);
            b.Append(", ");
            b.Append("object=");
            b.Append(this._object);
            b.Append(", ");
            b.Append("operator=");
            b.Append(this._operator);
            b.Append(", ");
            b.Append("out=");
            b.Append(this._out);
            b.Append(", ");
            b.Append("override=");
            b.Append(this._override);
            b.Append(", ");
            b.Append("params=");
            b.Append(this._params);
            b.Append(", ");
            b.Append("private=");
            b.Append(this._private);
            b.Append(", ");
            b.Append("protected=");
            b.Append(this._protected);
            b.Append(", ");
            b.Append("public=");
            b.Append(this._public);
            b.Append(", ");
            b.Append("readonly=");
            b.Append(this._readonly);
            b.Append(", ");
            b.Append("ref=");
            b.Append(this._ref);
            b.Append(", ");
            b.Append("return=");
            b.Append(this._return);
            b.Append(", ");
            b.Append("sbyte=");
            b.Append(this._sbyte);
            b.Append(", ");
            b.Append("sealed=");
            b.Append(this._sealed);
            b.Append(", ");
            b.Append("short=");
            b.Append(this._short);
            b.Append(", ");
            b.Append("sizeof=");
            b.Append(this._sizeof);
            b.Append(", ");
            b.Append("stackalloc=");
            b.Append(this._stackalloc);
            b.Append(", ");
            b.Append("static=");
            b.Append(this._static);
            b.Append(", ");
            b.Append("string=");
            b.Append(this._string);
            b.Append(", ");
            b.Append("struct=");
            b.Append(this._struct);
            b.Append(", ");
            b.Append("switch=");
            b.Append(this._switch);
            b.Append(", ");
            b.Append("this=");
            b.Append(this._this);
            b.Append(", ");
            b.Append("throw=");
            b.Append(this._throw);
            b.Append(", ");
            b.Append("true=");
            b.Append(this._true);
            b.Append(", ");
            b.Append("try=");
            b.Append(this._try);
            b.Append(", ");
            b.Append("typeof=");
            b.Append(this._typeof);
            b.Append(", ");
            b.Append("uint=");
            b.Append(this._uint);
            b.Append(", ");
            b.Append("ulong=");
            b.Append(this._ulong);
            b.Append(", ");
            b.Append("unchecked=");
            b.Append(this._unchecked);
            b.Append(", ");
            b.Append("unsafe=");
            b.Append(this._unsafe);
            b.Append(", ");
            b.Append("ushort=");
            b.Append(this._ushort);
            b.Append(", ");
            b.Append("using=");
            b.Append(this._using);
            b.Append(", ");
            b.Append("virtual=");
            b.Append(this._virtual);
            b.Append(", ");
            b.Append("void=");
            b.Append(this._void);
            b.Append(", ");
            b.Append("volatile=");
            b.Append(this._volatile);
            b.Append(", ");
            b.Append("while=");
            b.Append(this._while);
            b.Append(")");

            return b.ToString();
        }
    }
}
