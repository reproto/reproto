package test;

import _true.Empty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Optional;

public class Entry {
    @JsonProperty("abstract")
    final Optional<String> _abstract;
    @JsonProperty("assert")
    final Optional<String> _assert;
    @JsonProperty("boolean")
    final Optional<String> _boolean;
    @JsonProperty("break")
    final Optional<String> _break;
    @JsonProperty("byte")
    final Optional<String> _byte;
    @JsonProperty("case")
    final Optional<String> _case;
    @JsonProperty("catch")
    final Optional<String> _catch;
    @JsonProperty("char")
    final Optional<String> _char;
    @JsonProperty("class")
    final Optional<String> _class;
    @JsonProperty("const")
    final Optional<String> _const;
    @JsonProperty("continue")
    final Optional<String> _continue;
    @JsonProperty("default")
    final Optional<String> _default;
    @JsonProperty("do")
    final Optional<String> _do;
    @JsonProperty("double")
    final Optional<String> _double;
    @JsonProperty("else")
    final Optional<String> _else;
    @JsonProperty("enum")
    final Optional<String> _enum;
    @JsonProperty("extends")
    final Optional<String> _extends;
    @JsonProperty("false")
    final Optional<String> _false;
    @JsonProperty("final")
    final Optional<String> _final;
    @JsonProperty("finally")
    final Optional<String> _finally;
    @JsonProperty("float")
    final Optional<String> _float;
    @JsonProperty("for")
    final Optional<String> _for;
    @JsonProperty("goto")
    final Optional<String> _goto;
    @JsonProperty("if")
    final Optional<String> _if;
    @JsonProperty("implements")
    final Optional<String> _implements;
    @JsonProperty("import")
    final Optional<String> _import;
    @JsonProperty("imported")
    final Optional<Empty> imported;
    @JsonProperty("instanceof")
    final Optional<String> _instanceof;
    @JsonProperty("int")
    final Optional<String> _int;
    @JsonProperty("interface")
    final Optional<String> _interface;
    @JsonProperty("long")
    final Optional<String> _long;
    @JsonProperty("native")
    final Optional<String> _native;
    @JsonProperty("new")
    final Optional<String> _new;
    @JsonProperty("null")
    final Optional<String> _null;
    @JsonProperty("package")
    final Optional<String> _package;
    @JsonProperty("private")
    final Optional<String> _private;
    @JsonProperty("protected")
    final Optional<String> _protected;
    @JsonProperty("public")
    final Optional<String> _public;
    @JsonProperty("return")
    final Optional<String> _return;
    @JsonProperty("short")
    final Optional<String> _short;
    @JsonProperty("static")
    final Optional<String> _static;
    @JsonProperty("strictfp")
    final Optional<String> _strictfp;
    @JsonProperty("super")
    final Optional<String> _super;
    @JsonProperty("switch")
    final Optional<String> _switch;
    @JsonProperty("synchronized")
    final Optional<String> _synchronized;
    @JsonProperty("this")
    final Optional<String> _this;
    @JsonProperty("throw")
    final Optional<String> _throw;
    @JsonProperty("throws")
    final Optional<String> _throws;
    @JsonProperty("transient")
    final Optional<String> _transient;
    @JsonProperty("true")
    final Optional<String> _true;
    @JsonProperty("try")
    final Optional<String> _try;
    @JsonProperty("void")
    final Optional<String> _void;
    @JsonProperty("volatile")
    final Optional<String> _volatile;
    @JsonProperty("while")
    final Optional<String> _while;

    @JsonCreator
    public Entry(
        @JsonProperty("abstract") Optional<String> _abstract,
        @JsonProperty("assert") Optional<String> _assert,
        @JsonProperty("boolean") Optional<String> _boolean,
        @JsonProperty("break") Optional<String> _break,
        @JsonProperty("byte") Optional<String> _byte,
        @JsonProperty("case") Optional<String> _case,
        @JsonProperty("catch") Optional<String> _catch,
        @JsonProperty("char") Optional<String> _char,
        @JsonProperty("class") Optional<String> _class,
        @JsonProperty("const") Optional<String> _const,
        @JsonProperty("continue") Optional<String> _continue,
        @JsonProperty("default") Optional<String> _default,
        @JsonProperty("do") Optional<String> _do,
        @JsonProperty("double") Optional<String> _double,
        @JsonProperty("else") Optional<String> _else,
        @JsonProperty("enum") Optional<String> _enum,
        @JsonProperty("extends") Optional<String> _extends,
        @JsonProperty("false") Optional<String> _false,
        @JsonProperty("final") Optional<String> _final,
        @JsonProperty("finally") Optional<String> _finally,
        @JsonProperty("float") Optional<String> _float,
        @JsonProperty("for") Optional<String> _for,
        @JsonProperty("goto") Optional<String> _goto,
        @JsonProperty("if") Optional<String> _if,
        @JsonProperty("implements") Optional<String> _implements,
        @JsonProperty("import") Optional<String> _import,
        @JsonProperty("imported") Optional<Empty> imported,
        @JsonProperty("instanceof") Optional<String> _instanceof,
        @JsonProperty("int") Optional<String> _int,
        @JsonProperty("interface") Optional<String> _interface,
        @JsonProperty("long") Optional<String> _long,
        @JsonProperty("native") Optional<String> _native,
        @JsonProperty("new") Optional<String> _new,
        @JsonProperty("null") Optional<String> _null,
        @JsonProperty("package") Optional<String> _package,
        @JsonProperty("private") Optional<String> _private,
        @JsonProperty("protected") Optional<String> _protected,
        @JsonProperty("public") Optional<String> _public,
        @JsonProperty("return") Optional<String> _return,
        @JsonProperty("short") Optional<String> _short,
        @JsonProperty("static") Optional<String> _static,
        @JsonProperty("strictfp") Optional<String> _strictfp,
        @JsonProperty("super") Optional<String> _super,
        @JsonProperty("switch") Optional<String> _switch,
        @JsonProperty("synchronized") Optional<String> _synchronized,
        @JsonProperty("this") Optional<String> _this,
        @JsonProperty("throw") Optional<String> _throw,
        @JsonProperty("throws") Optional<String> _throws,
        @JsonProperty("transient") Optional<String> _transient,
        @JsonProperty("true") Optional<String> _true,
        @JsonProperty("try") Optional<String> _try,
        @JsonProperty("void") Optional<String> _void,
        @JsonProperty("volatile") Optional<String> _volatile,
        @JsonProperty("while") Optional<String> _while
    ) {
        this._abstract = _abstract;
        this._assert = _assert;
        this._boolean = _boolean;
        this._break = _break;
        this._byte = _byte;
        this._case = _case;
        this._catch = _catch;
        this._char = _char;
        this._class = _class;
        this._const = _const;
        this._continue = _continue;
        this._default = _default;
        this._do = _do;
        this._double = _double;
        this._else = _else;
        this._enum = _enum;
        this._extends = _extends;
        this._false = _false;
        this._final = _final;
        this._finally = _finally;
        this._float = _float;
        this._for = _for;
        this._goto = _goto;
        this._if = _if;
        this._implements = _implements;
        this._import = _import;
        this.imported = imported;
        this._instanceof = _instanceof;
        this._int = _int;
        this._interface = _interface;
        this._long = _long;
        this._native = _native;
        this._new = _new;
        this._null = _null;
        this._package = _package;
        this._private = _private;
        this._protected = _protected;
        this._public = _public;
        this._return = _return;
        this._short = _short;
        this._static = _static;
        this._strictfp = _strictfp;
        this._super = _super;
        this._switch = _switch;
        this._synchronized = _synchronized;
        this._this = _this;
        this._throw = _throw;
        this._throws = _throws;
        this._transient = _transient;
        this._true = _true;
        this._try = _try;
        this._void = _void;
        this._volatile = _volatile;
        this._while = _while;
    }

    @JsonProperty("abstract")
    public Optional<String> getAbstract() {
        return this._abstract;
    }

    @JsonProperty("assert")
    public Optional<String> getAssert() {
        return this._assert;
    }

    @JsonProperty("boolean")
    public Optional<String> getBoolean() {
        return this._boolean;
    }

    @JsonProperty("break")
    public Optional<String> getBreak() {
        return this._break;
    }

    @JsonProperty("byte")
    public Optional<String> getByte() {
        return this._byte;
    }

    @JsonProperty("case")
    public Optional<String> getCase() {
        return this._case;
    }

    @JsonProperty("catch")
    public Optional<String> getCatch() {
        return this._catch;
    }

    @JsonProperty("char")
    public Optional<String> getChar() {
        return this._char;
    }

    @JsonProperty("class")
    public Optional<String> getClass_() {
        return this._class;
    }

    @JsonProperty("const")
    public Optional<String> getConst() {
        return this._const;
    }

    @JsonProperty("continue")
    public Optional<String> getContinue() {
        return this._continue;
    }

    @JsonProperty("default")
    public Optional<String> getDefault() {
        return this._default;
    }

    @JsonProperty("do")
    public Optional<String> getDo() {
        return this._do;
    }

    @JsonProperty("double")
    public Optional<String> getDouble() {
        return this._double;
    }

    @JsonProperty("else")
    public Optional<String> getElse() {
        return this._else;
    }

    @JsonProperty("enum")
    public Optional<String> getEnum() {
        return this._enum;
    }

    @JsonProperty("extends")
    public Optional<String> getExtends() {
        return this._extends;
    }

    @JsonProperty("false")
    public Optional<String> getFalse() {
        return this._false;
    }

    @JsonProperty("final")
    public Optional<String> getFinal() {
        return this._final;
    }

    @JsonProperty("finally")
    public Optional<String> getFinally() {
        return this._finally;
    }

    @JsonProperty("float")
    public Optional<String> getFloat() {
        return this._float;
    }

    @JsonProperty("for")
    public Optional<String> getFor() {
        return this._for;
    }

    @JsonProperty("goto")
    public Optional<String> getGoto() {
        return this._goto;
    }

    @JsonProperty("if")
    public Optional<String> getIf() {
        return this._if;
    }

    @JsonProperty("implements")
    public Optional<String> getImplements() {
        return this._implements;
    }

    @JsonProperty("import")
    public Optional<String> getImport() {
        return this._import;
    }

    @JsonProperty("imported")
    public Optional<Empty> getImported() {
        return this.imported;
    }

    @JsonProperty("instanceof")
    public Optional<String> getInstanceof() {
        return this._instanceof;
    }

    @JsonProperty("int")
    public Optional<String> getInt() {
        return this._int;
    }

    @JsonProperty("interface")
    public Optional<String> getInterface() {
        return this._interface;
    }

    @JsonProperty("long")
    public Optional<String> getLong() {
        return this._long;
    }

    @JsonProperty("native")
    public Optional<String> getNative() {
        return this._native;
    }

    @JsonProperty("new")
    public Optional<String> getNew() {
        return this._new;
    }

    @JsonProperty("null")
    public Optional<String> getNull() {
        return this._null;
    }

    @JsonProperty("package")
    public Optional<String> getPackage() {
        return this._package;
    }

    @JsonProperty("private")
    public Optional<String> getPrivate() {
        return this._private;
    }

    @JsonProperty("protected")
    public Optional<String> getProtected() {
        return this._protected;
    }

    @JsonProperty("public")
    public Optional<String> getPublic() {
        return this._public;
    }

    @JsonProperty("return")
    public Optional<String> getReturn() {
        return this._return;
    }

    @JsonProperty("short")
    public Optional<String> getShort() {
        return this._short;
    }

    @JsonProperty("static")
    public Optional<String> getStatic() {
        return this._static;
    }

    @JsonProperty("strictfp")
    public Optional<String> getStrictfp() {
        return this._strictfp;
    }

    @JsonProperty("super")
    public Optional<String> getSuper() {
        return this._super;
    }

    @JsonProperty("switch")
    public Optional<String> getSwitch() {
        return this._switch;
    }

    @JsonProperty("synchronized")
    public Optional<String> getSynchronized() {
        return this._synchronized;
    }

    @JsonProperty("this")
    public Optional<String> getThis() {
        return this._this;
    }

    @JsonProperty("throw")
    public Optional<String> getThrow() {
        return this._throw;
    }

    @JsonProperty("throws")
    public Optional<String> getThrows() {
        return this._throws;
    }

    @JsonProperty("transient")
    public Optional<String> getTransient() {
        return this._transient;
    }

    @JsonProperty("true")
    public Optional<String> getTrue() {
        return this._true;
    }

    @JsonProperty("try")
    public Optional<String> getTry() {
        return this._try;
    }

    @JsonProperty("void")
    public Optional<String> getVoid() {
        return this._void;
    }

    @JsonProperty("volatile")
    public Optional<String> getVolatile() {
        return this._volatile;
    }

    @JsonProperty("while")
    public Optional<String> getWhile() {
        return this._while;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Entry(");
        b.append("abstract=");
        b.append(this._abstract.toString());
        b.append(", ");
        b.append("assert=");
        b.append(this._assert.toString());
        b.append(", ");
        b.append("boolean=");
        b.append(this._boolean.toString());
        b.append(", ");
        b.append("break=");
        b.append(this._break.toString());
        b.append(", ");
        b.append("byte=");
        b.append(this._byte.toString());
        b.append(", ");
        b.append("case=");
        b.append(this._case.toString());
        b.append(", ");
        b.append("catch=");
        b.append(this._catch.toString());
        b.append(", ");
        b.append("char=");
        b.append(this._char.toString());
        b.append(", ");
        b.append("class=");
        b.append(this._class.toString());
        b.append(", ");
        b.append("const=");
        b.append(this._const.toString());
        b.append(", ");
        b.append("continue=");
        b.append(this._continue.toString());
        b.append(", ");
        b.append("default=");
        b.append(this._default.toString());
        b.append(", ");
        b.append("do=");
        b.append(this._do.toString());
        b.append(", ");
        b.append("double=");
        b.append(this._double.toString());
        b.append(", ");
        b.append("else=");
        b.append(this._else.toString());
        b.append(", ");
        b.append("enum=");
        b.append(this._enum.toString());
        b.append(", ");
        b.append("extends=");
        b.append(this._extends.toString());
        b.append(", ");
        b.append("false=");
        b.append(this._false.toString());
        b.append(", ");
        b.append("final=");
        b.append(this._final.toString());
        b.append(", ");
        b.append("finally=");
        b.append(this._finally.toString());
        b.append(", ");
        b.append("float=");
        b.append(this._float.toString());
        b.append(", ");
        b.append("for=");
        b.append(this._for.toString());
        b.append(", ");
        b.append("goto=");
        b.append(this._goto.toString());
        b.append(", ");
        b.append("if=");
        b.append(this._if.toString());
        b.append(", ");
        b.append("implements=");
        b.append(this._implements.toString());
        b.append(", ");
        b.append("import=");
        b.append(this._import.toString());
        b.append(", ");
        b.append("imported=");
        b.append(this.imported.toString());
        b.append(", ");
        b.append("instanceof=");
        b.append(this._instanceof.toString());
        b.append(", ");
        b.append("int=");
        b.append(this._int.toString());
        b.append(", ");
        b.append("interface=");
        b.append(this._interface.toString());
        b.append(", ");
        b.append("long=");
        b.append(this._long.toString());
        b.append(", ");
        b.append("native=");
        b.append(this._native.toString());
        b.append(", ");
        b.append("new=");
        b.append(this._new.toString());
        b.append(", ");
        b.append("null=");
        b.append(this._null.toString());
        b.append(", ");
        b.append("package=");
        b.append(this._package.toString());
        b.append(", ");
        b.append("private=");
        b.append(this._private.toString());
        b.append(", ");
        b.append("protected=");
        b.append(this._protected.toString());
        b.append(", ");
        b.append("public=");
        b.append(this._public.toString());
        b.append(", ");
        b.append("return=");
        b.append(this._return.toString());
        b.append(", ");
        b.append("short=");
        b.append(this._short.toString());
        b.append(", ");
        b.append("static=");
        b.append(this._static.toString());
        b.append(", ");
        b.append("strictfp=");
        b.append(this._strictfp.toString());
        b.append(", ");
        b.append("super=");
        b.append(this._super.toString());
        b.append(", ");
        b.append("switch=");
        b.append(this._switch.toString());
        b.append(", ");
        b.append("synchronized=");
        b.append(this._synchronized.toString());
        b.append(", ");
        b.append("this=");
        b.append(this._this.toString());
        b.append(", ");
        b.append("throw=");
        b.append(this._throw.toString());
        b.append(", ");
        b.append("throws=");
        b.append(this._throws.toString());
        b.append(", ");
        b.append("transient=");
        b.append(this._transient.toString());
        b.append(", ");
        b.append("true=");
        b.append(this._true.toString());
        b.append(", ");
        b.append("try=");
        b.append(this._try.toString());
        b.append(", ");
        b.append("void=");
        b.append(this._void.toString());
        b.append(", ");
        b.append("volatile=");
        b.append(this._volatile.toString());
        b.append(", ");
        b.append("while=");
        b.append(this._while.toString());
        b.append(")");

        return b.toString();
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this._abstract.hashCode();
        result = result * 31 + this._assert.hashCode();
        result = result * 31 + this._boolean.hashCode();
        result = result * 31 + this._break.hashCode();
        result = result * 31 + this._byte.hashCode();
        result = result * 31 + this._case.hashCode();
        result = result * 31 + this._catch.hashCode();
        result = result * 31 + this._char.hashCode();
        result = result * 31 + this._class.hashCode();
        result = result * 31 + this._const.hashCode();
        result = result * 31 + this._continue.hashCode();
        result = result * 31 + this._default.hashCode();
        result = result * 31 + this._do.hashCode();
        result = result * 31 + this._double.hashCode();
        result = result * 31 + this._else.hashCode();
        result = result * 31 + this._enum.hashCode();
        result = result * 31 + this._extends.hashCode();
        result = result * 31 + this._false.hashCode();
        result = result * 31 + this._final.hashCode();
        result = result * 31 + this._finally.hashCode();
        result = result * 31 + this._float.hashCode();
        result = result * 31 + this._for.hashCode();
        result = result * 31 + this._goto.hashCode();
        result = result * 31 + this._if.hashCode();
        result = result * 31 + this._implements.hashCode();
        result = result * 31 + this._import.hashCode();
        result = result * 31 + this.imported.hashCode();
        result = result * 31 + this._instanceof.hashCode();
        result = result * 31 + this._int.hashCode();
        result = result * 31 + this._interface.hashCode();
        result = result * 31 + this._long.hashCode();
        result = result * 31 + this._native.hashCode();
        result = result * 31 + this._new.hashCode();
        result = result * 31 + this._null.hashCode();
        result = result * 31 + this._package.hashCode();
        result = result * 31 + this._private.hashCode();
        result = result * 31 + this._protected.hashCode();
        result = result * 31 + this._public.hashCode();
        result = result * 31 + this._return.hashCode();
        result = result * 31 + this._short.hashCode();
        result = result * 31 + this._static.hashCode();
        result = result * 31 + this._strictfp.hashCode();
        result = result * 31 + this._super.hashCode();
        result = result * 31 + this._switch.hashCode();
        result = result * 31 + this._synchronized.hashCode();
        result = result * 31 + this._this.hashCode();
        result = result * 31 + this._throw.hashCode();
        result = result * 31 + this._throws.hashCode();
        result = result * 31 + this._transient.hashCode();
        result = result * 31 + this._true.hashCode();
        result = result * 31 + this._try.hashCode();
        result = result * 31 + this._void.hashCode();
        result = result * 31 + this._volatile.hashCode();
        result = result * 31 + this._while.hashCode();
        return result;
    }

    @Override
    public boolean equals(final Object other_) {
        if (other_ == null) {
            return false;
        }

        if (!(other_ instanceof Entry)) {
            return false;
        }

        @SuppressWarnings("unchecked")
        final Entry o_ = (Entry)other_;

        if (!this._abstract.equals(o_._abstract)) {
            return false;
        }

        if (!this._assert.equals(o_._assert)) {
            return false;
        }

        if (!this._boolean.equals(o_._boolean)) {
            return false;
        }

        if (!this._break.equals(o_._break)) {
            return false;
        }

        if (!this._byte.equals(o_._byte)) {
            return false;
        }

        if (!this._case.equals(o_._case)) {
            return false;
        }

        if (!this._catch.equals(o_._catch)) {
            return false;
        }

        if (!this._char.equals(o_._char)) {
            return false;
        }

        if (!this._class.equals(o_._class)) {
            return false;
        }

        if (!this._const.equals(o_._const)) {
            return false;
        }

        if (!this._continue.equals(o_._continue)) {
            return false;
        }

        if (!this._default.equals(o_._default)) {
            return false;
        }

        if (!this._do.equals(o_._do)) {
            return false;
        }

        if (!this._double.equals(o_._double)) {
            return false;
        }

        if (!this._else.equals(o_._else)) {
            return false;
        }

        if (!this._enum.equals(o_._enum)) {
            return false;
        }

        if (!this._extends.equals(o_._extends)) {
            return false;
        }

        if (!this._false.equals(o_._false)) {
            return false;
        }

        if (!this._final.equals(o_._final)) {
            return false;
        }

        if (!this._finally.equals(o_._finally)) {
            return false;
        }

        if (!this._float.equals(o_._float)) {
            return false;
        }

        if (!this._for.equals(o_._for)) {
            return false;
        }

        if (!this._goto.equals(o_._goto)) {
            return false;
        }

        if (!this._if.equals(o_._if)) {
            return false;
        }

        if (!this._implements.equals(o_._implements)) {
            return false;
        }

        if (!this._import.equals(o_._import)) {
            return false;
        }

        if (!this.imported.equals(o_.imported)) {
            return false;
        }

        if (!this._instanceof.equals(o_._instanceof)) {
            return false;
        }

        if (!this._int.equals(o_._int)) {
            return false;
        }

        if (!this._interface.equals(o_._interface)) {
            return false;
        }

        if (!this._long.equals(o_._long)) {
            return false;
        }

        if (!this._native.equals(o_._native)) {
            return false;
        }

        if (!this._new.equals(o_._new)) {
            return false;
        }

        if (!this._null.equals(o_._null)) {
            return false;
        }

        if (!this._package.equals(o_._package)) {
            return false;
        }

        if (!this._private.equals(o_._private)) {
            return false;
        }

        if (!this._protected.equals(o_._protected)) {
            return false;
        }

        if (!this._public.equals(o_._public)) {
            return false;
        }

        if (!this._return.equals(o_._return)) {
            return false;
        }

        if (!this._short.equals(o_._short)) {
            return false;
        }

        if (!this._static.equals(o_._static)) {
            return false;
        }

        if (!this._strictfp.equals(o_._strictfp)) {
            return false;
        }

        if (!this._super.equals(o_._super)) {
            return false;
        }

        if (!this._switch.equals(o_._switch)) {
            return false;
        }

        if (!this._synchronized.equals(o_._synchronized)) {
            return false;
        }

        if (!this._this.equals(o_._this)) {
            return false;
        }

        if (!this._throw.equals(o_._throw)) {
            return false;
        }

        if (!this._throws.equals(o_._throws)) {
            return false;
        }

        if (!this._transient.equals(o_._transient)) {
            return false;
        }

        if (!this._true.equals(o_._true)) {
            return false;
        }

        if (!this._try.equals(o_._try)) {
            return false;
        }

        if (!this._void.equals(o_._void)) {
            return false;
        }

        if (!this._volatile.equals(o_._volatile)) {
            return false;
        }

        if (!this._while.equals(o_._while)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<String> _abstract;
        private Optional<String> _assert;
        private Optional<String> _boolean;
        private Optional<String> _break;
        private Optional<String> _byte;
        private Optional<String> _case;
        private Optional<String> _catch;
        private Optional<String> _char;
        private Optional<String> _class;
        private Optional<String> _const;
        private Optional<String> _continue;
        private Optional<String> _default;
        private Optional<String> _do;
        private Optional<String> _double;
        private Optional<String> _else;
        private Optional<String> _enum;
        private Optional<String> _extends;
        private Optional<String> _false;
        private Optional<String> _final;
        private Optional<String> _finally;
        private Optional<String> _float;
        private Optional<String> _for;
        private Optional<String> _goto;
        private Optional<String> _if;
        private Optional<String> _implements;
        private Optional<String> _import;
        private Optional<Empty> imported;
        private Optional<String> _instanceof;
        private Optional<String> _int;
        private Optional<String> _interface;
        private Optional<String> _long;
        private Optional<String> _native;
        private Optional<String> _new;
        private Optional<String> _null;
        private Optional<String> _package;
        private Optional<String> _private;
        private Optional<String> _protected;
        private Optional<String> _public;
        private Optional<String> _return;
        private Optional<String> _short;
        private Optional<String> _static;
        private Optional<String> _strictfp;
        private Optional<String> _super;
        private Optional<String> _switch;
        private Optional<String> _synchronized;
        private Optional<String> _this;
        private Optional<String> _throw;
        private Optional<String> _throws;
        private Optional<String> _transient;
        private Optional<String> _true;
        private Optional<String> _try;
        private Optional<String> _void;
        private Optional<String> _volatile;
        private Optional<String> _while;

        private Builder() {
            this._abstract = Optional.empty();
            this._assert = Optional.empty();
            this._boolean = Optional.empty();
            this._break = Optional.empty();
            this._byte = Optional.empty();
            this._case = Optional.empty();
            this._catch = Optional.empty();
            this._char = Optional.empty();
            this._class = Optional.empty();
            this._const = Optional.empty();
            this._continue = Optional.empty();
            this._default = Optional.empty();
            this._do = Optional.empty();
            this._double = Optional.empty();
            this._else = Optional.empty();
            this._enum = Optional.empty();
            this._extends = Optional.empty();
            this._false = Optional.empty();
            this._final = Optional.empty();
            this._finally = Optional.empty();
            this._float = Optional.empty();
            this._for = Optional.empty();
            this._goto = Optional.empty();
            this._if = Optional.empty();
            this._implements = Optional.empty();
            this._import = Optional.empty();
            this.imported = Optional.empty();
            this._instanceof = Optional.empty();
            this._int = Optional.empty();
            this._interface = Optional.empty();
            this._long = Optional.empty();
            this._native = Optional.empty();
            this._new = Optional.empty();
            this._null = Optional.empty();
            this._package = Optional.empty();
            this._private = Optional.empty();
            this._protected = Optional.empty();
            this._public = Optional.empty();
            this._return = Optional.empty();
            this._short = Optional.empty();
            this._static = Optional.empty();
            this._strictfp = Optional.empty();
            this._super = Optional.empty();
            this._switch = Optional.empty();
            this._synchronized = Optional.empty();
            this._this = Optional.empty();
            this._throw = Optional.empty();
            this._throws = Optional.empty();
            this._transient = Optional.empty();
            this._true = Optional.empty();
            this._try = Optional.empty();
            this._void = Optional.empty();
            this._volatile = Optional.empty();
            this._while = Optional.empty();
        }

        public Entry build() {

            return new Entry(
                this._abstract,
                this._assert,
                this._boolean,
                this._break,
                this._byte,
                this._case,
                this._catch,
                this._char,
                this._class,
                this._const,
                this._continue,
                this._default,
                this._do,
                this._double,
                this._else,
                this._enum,
                this._extends,
                this._false,
                this._final,
                this._finally,
                this._float,
                this._for,
                this._goto,
                this._if,
                this._implements,
                this._import,
                this.imported,
                this._instanceof,
                this._int,
                this._interface,
                this._long,
                this._native,
                this._new,
                this._null,
                this._package,
                this._private,
                this._protected,
                this._public,
                this._return,
                this._short,
                this._static,
                this._strictfp,
                this._super,
                this._switch,
                this._synchronized,
                this._this,
                this._throw,
                this._throws,
                this._transient,
                this._true,
                this._try,
                this._void,
                this._volatile,
                this._while
            );
        }

        public Builder _abstract(final String _abstract) {
            this._abstract = Optional.of(_abstract);
            return this;
        }

        public Builder _assert(final String _assert) {
            this._assert = Optional.of(_assert);
            return this;
        }

        public Builder _boolean(final String _boolean) {
            this._boolean = Optional.of(_boolean);
            return this;
        }

        public Builder _break(final String _break) {
            this._break = Optional.of(_break);
            return this;
        }

        public Builder _byte(final String _byte) {
            this._byte = Optional.of(_byte);
            return this;
        }

        public Builder _case(final String _case) {
            this._case = Optional.of(_case);
            return this;
        }

        public Builder _catch(final String _catch) {
            this._catch = Optional.of(_catch);
            return this;
        }

        public Builder _char(final String _char) {
            this._char = Optional.of(_char);
            return this;
        }

        public Builder _class(final String _class) {
            this._class = Optional.of(_class);
            return this;
        }

        public Builder _const(final String _const) {
            this._const = Optional.of(_const);
            return this;
        }

        public Builder _continue(final String _continue) {
            this._continue = Optional.of(_continue);
            return this;
        }

        public Builder _default(final String _default) {
            this._default = Optional.of(_default);
            return this;
        }

        public Builder _do(final String _do) {
            this._do = Optional.of(_do);
            return this;
        }

        public Builder _double(final String _double) {
            this._double = Optional.of(_double);
            return this;
        }

        public Builder _else(final String _else) {
            this._else = Optional.of(_else);
            return this;
        }

        public Builder _enum(final String _enum) {
            this._enum = Optional.of(_enum);
            return this;
        }

        public Builder _extends(final String _extends) {
            this._extends = Optional.of(_extends);
            return this;
        }

        public Builder _false(final String _false) {
            this._false = Optional.of(_false);
            return this;
        }

        public Builder _final(final String _final) {
            this._final = Optional.of(_final);
            return this;
        }

        public Builder _finally(final String _finally) {
            this._finally = Optional.of(_finally);
            return this;
        }

        public Builder _float(final String _float) {
            this._float = Optional.of(_float);
            return this;
        }

        public Builder _for(final String _for) {
            this._for = Optional.of(_for);
            return this;
        }

        public Builder _goto(final String _goto) {
            this._goto = Optional.of(_goto);
            return this;
        }

        public Builder _if(final String _if) {
            this._if = Optional.of(_if);
            return this;
        }

        public Builder _implements(final String _implements) {
            this._implements = Optional.of(_implements);
            return this;
        }

        public Builder _import(final String _import) {
            this._import = Optional.of(_import);
            return this;
        }

        public Builder imported(final Empty imported) {
            this.imported = Optional.of(imported);
            return this;
        }

        public Builder _instanceof(final String _instanceof) {
            this._instanceof = Optional.of(_instanceof);
            return this;
        }

        public Builder _int(final String _int) {
            this._int = Optional.of(_int);
            return this;
        }

        public Builder _interface(final String _interface) {
            this._interface = Optional.of(_interface);
            return this;
        }

        public Builder _long(final String _long) {
            this._long = Optional.of(_long);
            return this;
        }

        public Builder _native(final String _native) {
            this._native = Optional.of(_native);
            return this;
        }

        public Builder _new(final String _new) {
            this._new = Optional.of(_new);
            return this;
        }

        public Builder _null(final String _null) {
            this._null = Optional.of(_null);
            return this;
        }

        public Builder _package(final String _package) {
            this._package = Optional.of(_package);
            return this;
        }

        public Builder _private(final String _private) {
            this._private = Optional.of(_private);
            return this;
        }

        public Builder _protected(final String _protected) {
            this._protected = Optional.of(_protected);
            return this;
        }

        public Builder _public(final String _public) {
            this._public = Optional.of(_public);
            return this;
        }

        public Builder _return(final String _return) {
            this._return = Optional.of(_return);
            return this;
        }

        public Builder _short(final String _short) {
            this._short = Optional.of(_short);
            return this;
        }

        public Builder _static(final String _static) {
            this._static = Optional.of(_static);
            return this;
        }

        public Builder _strictfp(final String _strictfp) {
            this._strictfp = Optional.of(_strictfp);
            return this;
        }

        public Builder _super(final String _super) {
            this._super = Optional.of(_super);
            return this;
        }

        public Builder _switch(final String _switch) {
            this._switch = Optional.of(_switch);
            return this;
        }

        public Builder _synchronized(final String _synchronized) {
            this._synchronized = Optional.of(_synchronized);
            return this;
        }

        public Builder _this(final String _this) {
            this._this = Optional.of(_this);
            return this;
        }

        public Builder _throw(final String _throw) {
            this._throw = Optional.of(_throw);
            return this;
        }

        public Builder _throws(final String _throws) {
            this._throws = Optional.of(_throws);
            return this;
        }

        public Builder _transient(final String _transient) {
            this._transient = Optional.of(_transient);
            return this;
        }

        public Builder _true(final String _true) {
            this._true = Optional.of(_true);
            return this;
        }

        public Builder _try(final String _try) {
            this._try = Optional.of(_try);
            return this;
        }

        public Builder _void(final String _void) {
            this._void = Optional.of(_void);
            return this;
        }

        public Builder _volatile(final String _volatile) {
            this._volatile = Optional.of(_volatile);
            return this;
        }

        public Builder _while(final String _while) {
            this._while = Optional.of(_while);
            return this;
        }
    }

    /**
     * Construct a new builder.
     */
    public static Builder builder() {
        return new Builder();
    }
}
