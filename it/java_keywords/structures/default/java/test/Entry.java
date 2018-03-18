package test;

import _true.Empty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Entry {
  @JsonProperty("abstract")
  private final Optional<String> _abstract;
  @JsonProperty("assert")
  private final Optional<String> _assert;
  @JsonProperty("boolean")
  private final Optional<String> _boolean;
  @JsonProperty("break")
  private final Optional<String> _break;
  @JsonProperty("byte")
  private final Optional<String> _byte;
  @JsonProperty("case")
  private final Optional<String> _case;
  @JsonProperty("catch")
  private final Optional<String> _catch;
  @JsonProperty("char")
  private final Optional<String> _char;
  @JsonProperty("class")
  private final Optional<String> _class;
  @JsonProperty("const")
  private final Optional<String> _const;
  @JsonProperty("continue")
  private final Optional<String> _continue;
  @JsonProperty("default")
  private final Optional<String> _default;
  @JsonProperty("do")
  private final Optional<String> _do;
  @JsonProperty("double")
  private final Optional<String> _double;
  @JsonProperty("else")
  private final Optional<String> _else;
  @JsonProperty("enum")
  private final Optional<String> _enum;
  @JsonProperty("extends")
  private final Optional<String> _extends;
  @JsonProperty("false")
  private final Optional<String> _false;
  @JsonProperty("final")
  private final Optional<String> _final;
  @JsonProperty("finally")
  private final Optional<String> _finally;
  @JsonProperty("float")
  private final Optional<String> _float;
  @JsonProperty("for")
  private final Optional<String> _for;
  @JsonProperty("goto")
  private final Optional<String> _goto;
  @JsonProperty("if")
  private final Optional<String> _if;
  @JsonProperty("implements")
  private final Optional<String> _implements;
  @JsonProperty("import")
  private final Optional<String> _import;
  @JsonProperty("imported")
  private final Optional<Empty> imported;
  @JsonProperty("instanceof")
  private final Optional<String> _instanceof;
  @JsonProperty("int")
  private final Optional<String> _int;
  @JsonProperty("interface")
  private final Optional<String> _interface;
  @JsonProperty("long")
  private final Optional<String> _long;
  @JsonProperty("native")
  private final Optional<String> _native;
  @JsonProperty("new")
  private final Optional<String> _new;
  @JsonProperty("null")
  private final Optional<String> _null;
  @JsonProperty("package")
  private final Optional<String> _package;
  @JsonProperty("private")
  private final Optional<String> _private;
  @JsonProperty("protected")
  private final Optional<String> _protected;
  @JsonProperty("public")
  private final Optional<String> _public;
  @JsonProperty("return")
  private final Optional<String> _return;
  @JsonProperty("short")
  private final Optional<String> _short;
  @JsonProperty("static")
  private final Optional<String> _static;
  @JsonProperty("strictfp")
  private final Optional<String> _strictfp;
  @JsonProperty("super")
  private final Optional<String> _super;
  @JsonProperty("switch")
  private final Optional<String> _switch;
  @JsonProperty("synchronized")
  private final Optional<String> _synchronized;
  @JsonProperty("this")
  private final Optional<String> _this;
  @JsonProperty("throw")
  private final Optional<String> _throw;
  @JsonProperty("throws")
  private final Optional<String> _throws;
  @JsonProperty("transient")
  private final Optional<String> _transient;
  @JsonProperty("true")
  private final Optional<String> _true;
  @JsonProperty("try")
  private final Optional<String> _try;
  @JsonProperty("void")
  private final Optional<String> _void;
  @JsonProperty("volatile")
  private final Optional<String> _volatile;
  @JsonProperty("while")
  private final Optional<String> _while;

  @JsonCreator
  public Entry(
    @JsonProperty("abstract") final Optional<String> _abstract,
    @JsonProperty("assert") final Optional<String> _assert,
    @JsonProperty("boolean") final Optional<String> _boolean,
    @JsonProperty("break") final Optional<String> _break,
    @JsonProperty("byte") final Optional<String> _byte,
    @JsonProperty("case") final Optional<String> _case,
    @JsonProperty("catch") final Optional<String> _catch,
    @JsonProperty("char") final Optional<String> _char,
    @JsonProperty("class") final Optional<String> _class,
    @JsonProperty("const") final Optional<String> _const,
    @JsonProperty("continue") final Optional<String> _continue,
    @JsonProperty("default") final Optional<String> _default,
    @JsonProperty("do") final Optional<String> _do,
    @JsonProperty("double") final Optional<String> _double,
    @JsonProperty("else") final Optional<String> _else,
    @JsonProperty("enum") final Optional<String> _enum,
    @JsonProperty("extends") final Optional<String> _extends,
    @JsonProperty("false") final Optional<String> _false,
    @JsonProperty("final") final Optional<String> _final,
    @JsonProperty("finally") final Optional<String> _finally,
    @JsonProperty("float") final Optional<String> _float,
    @JsonProperty("for") final Optional<String> _for,
    @JsonProperty("goto") final Optional<String> _goto,
    @JsonProperty("if") final Optional<String> _if,
    @JsonProperty("implements") final Optional<String> _implements,
    @JsonProperty("import") final Optional<String> _import,
    @JsonProperty("imported") final Optional<Empty> imported,
    @JsonProperty("instanceof") final Optional<String> _instanceof,
    @JsonProperty("int") final Optional<String> _int,
    @JsonProperty("interface") final Optional<String> _interface,
    @JsonProperty("long") final Optional<String> _long,
    @JsonProperty("native") final Optional<String> _native,
    @JsonProperty("new") final Optional<String> _new,
    @JsonProperty("null") final Optional<String> _null,
    @JsonProperty("package") final Optional<String> _package,
    @JsonProperty("private") final Optional<String> _private,
    @JsonProperty("protected") final Optional<String> _protected,
    @JsonProperty("public") final Optional<String> _public,
    @JsonProperty("return") final Optional<String> _return,
    @JsonProperty("short") final Optional<String> _short,
    @JsonProperty("static") final Optional<String> _static,
    @JsonProperty("strictfp") final Optional<String> _strictfp,
    @JsonProperty("super") final Optional<String> _super,
    @JsonProperty("switch") final Optional<String> _switch,
    @JsonProperty("synchronized") final Optional<String> _synchronized,
    @JsonProperty("this") final Optional<String> _this,
    @JsonProperty("throw") final Optional<String> _throw,
    @JsonProperty("throws") final Optional<String> _throws,
    @JsonProperty("transient") final Optional<String> _transient,
    @JsonProperty("true") final Optional<String> _true,
    @JsonProperty("try") final Optional<String> _try,
    @JsonProperty("void") final Optional<String> _void,
    @JsonProperty("volatile") final Optional<String> _volatile,
    @JsonProperty("while") final Optional<String> _while
  ) {
    Objects.requireNonNull(_abstract, "abstract");
    this._abstract = _abstract;
    Objects.requireNonNull(_assert, "assert");
    this._assert = _assert;
    Objects.requireNonNull(_boolean, "boolean");
    this._boolean = _boolean;
    Objects.requireNonNull(_break, "break");
    this._break = _break;
    Objects.requireNonNull(_byte, "byte");
    this._byte = _byte;
    Objects.requireNonNull(_case, "case");
    this._case = _case;
    Objects.requireNonNull(_catch, "catch");
    this._catch = _catch;
    Objects.requireNonNull(_char, "char");
    this._char = _char;
    Objects.requireNonNull(_class, "class");
    this._class = _class;
    Objects.requireNonNull(_const, "const");
    this._const = _const;
    Objects.requireNonNull(_continue, "continue");
    this._continue = _continue;
    Objects.requireNonNull(_default, "default");
    this._default = _default;
    Objects.requireNonNull(_do, "do");
    this._do = _do;
    Objects.requireNonNull(_double, "double");
    this._double = _double;
    Objects.requireNonNull(_else, "else");
    this._else = _else;
    Objects.requireNonNull(_enum, "enum");
    this._enum = _enum;
    Objects.requireNonNull(_extends, "extends");
    this._extends = _extends;
    Objects.requireNonNull(_false, "false");
    this._false = _false;
    Objects.requireNonNull(_final, "final");
    this._final = _final;
    Objects.requireNonNull(_finally, "finally");
    this._finally = _finally;
    Objects.requireNonNull(_float, "float");
    this._float = _float;
    Objects.requireNonNull(_for, "for");
    this._for = _for;
    Objects.requireNonNull(_goto, "goto");
    this._goto = _goto;
    Objects.requireNonNull(_if, "if");
    this._if = _if;
    Objects.requireNonNull(_implements, "implements");
    this._implements = _implements;
    Objects.requireNonNull(_import, "import");
    this._import = _import;
    Objects.requireNonNull(imported, "imported");
    this.imported = imported;
    Objects.requireNonNull(_instanceof, "instanceof");
    this._instanceof = _instanceof;
    Objects.requireNonNull(_int, "int");
    this._int = _int;
    Objects.requireNonNull(_interface, "interface");
    this._interface = _interface;
    Objects.requireNonNull(_long, "long");
    this._long = _long;
    Objects.requireNonNull(_native, "native");
    this._native = _native;
    Objects.requireNonNull(_new, "new");
    this._new = _new;
    Objects.requireNonNull(_null, "null");
    this._null = _null;
    Objects.requireNonNull(_package, "package");
    this._package = _package;
    Objects.requireNonNull(_private, "private");
    this._private = _private;
    Objects.requireNonNull(_protected, "protected");
    this._protected = _protected;
    Objects.requireNonNull(_public, "public");
    this._public = _public;
    Objects.requireNonNull(_return, "return");
    this._return = _return;
    Objects.requireNonNull(_short, "short");
    this._short = _short;
    Objects.requireNonNull(_static, "static");
    this._static = _static;
    Objects.requireNonNull(_strictfp, "strictfp");
    this._strictfp = _strictfp;
    Objects.requireNonNull(_super, "super");
    this._super = _super;
    Objects.requireNonNull(_switch, "switch");
    this._switch = _switch;
    Objects.requireNonNull(_synchronized, "synchronized");
    this._synchronized = _synchronized;
    Objects.requireNonNull(_this, "this");
    this._this = _this;
    Objects.requireNonNull(_throw, "throw");
    this._throw = _throw;
    Objects.requireNonNull(_throws, "throws");
    this._throws = _throws;
    Objects.requireNonNull(_transient, "transient");
    this._transient = _transient;
    Objects.requireNonNull(_true, "true");
    this._true = _true;
    Objects.requireNonNull(_try, "try");
    this._try = _try;
    Objects.requireNonNull(_void, "void");
    this._void = _void;
    Objects.requireNonNull(_volatile, "volatile");
    this._volatile = _volatile;
    Objects.requireNonNull(_while, "while");
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
  public int hashCode() {
    int result = 1;
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
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Entry)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Entry o = (Entry) other;

    if (!this._abstract.equals(o._abstract)) {
      return false;
    }

    if (!this._assert.equals(o._assert)) {
      return false;
    }

    if (!this._boolean.equals(o._boolean)) {
      return false;
    }

    if (!this._break.equals(o._break)) {
      return false;
    }

    if (!this._byte.equals(o._byte)) {
      return false;
    }

    if (!this._case.equals(o._case)) {
      return false;
    }

    if (!this._catch.equals(o._catch)) {
      return false;
    }

    if (!this._char.equals(o._char)) {
      return false;
    }

    if (!this._class.equals(o._class)) {
      return false;
    }

    if (!this._const.equals(o._const)) {
      return false;
    }

    if (!this._continue.equals(o._continue)) {
      return false;
    }

    if (!this._default.equals(o._default)) {
      return false;
    }

    if (!this._do.equals(o._do)) {
      return false;
    }

    if (!this._double.equals(o._double)) {
      return false;
    }

    if (!this._else.equals(o._else)) {
      return false;
    }

    if (!this._enum.equals(o._enum)) {
      return false;
    }

    if (!this._extends.equals(o._extends)) {
      return false;
    }

    if (!this._false.equals(o._false)) {
      return false;
    }

    if (!this._final.equals(o._final)) {
      return false;
    }

    if (!this._finally.equals(o._finally)) {
      return false;
    }

    if (!this._float.equals(o._float)) {
      return false;
    }

    if (!this._for.equals(o._for)) {
      return false;
    }

    if (!this._goto.equals(o._goto)) {
      return false;
    }

    if (!this._if.equals(o._if)) {
      return false;
    }

    if (!this._implements.equals(o._implements)) {
      return false;
    }

    if (!this._import.equals(o._import)) {
      return false;
    }

    if (!this.imported.equals(o.imported)) {
      return false;
    }

    if (!this._instanceof.equals(o._instanceof)) {
      return false;
    }

    if (!this._int.equals(o._int)) {
      return false;
    }

    if (!this._interface.equals(o._interface)) {
      return false;
    }

    if (!this._long.equals(o._long)) {
      return false;
    }

    if (!this._native.equals(o._native)) {
      return false;
    }

    if (!this._new.equals(o._new)) {
      return false;
    }

    if (!this._null.equals(o._null)) {
      return false;
    }

    if (!this._package.equals(o._package)) {
      return false;
    }

    if (!this._private.equals(o._private)) {
      return false;
    }

    if (!this._protected.equals(o._protected)) {
      return false;
    }

    if (!this._public.equals(o._public)) {
      return false;
    }

    if (!this._return.equals(o._return)) {
      return false;
    }

    if (!this._short.equals(o._short)) {
      return false;
    }

    if (!this._static.equals(o._static)) {
      return false;
    }

    if (!this._strictfp.equals(o._strictfp)) {
      return false;
    }

    if (!this._super.equals(o._super)) {
      return false;
    }

    if (!this._switch.equals(o._switch)) {
      return false;
    }

    if (!this._synchronized.equals(o._synchronized)) {
      return false;
    }

    if (!this._this.equals(o._this)) {
      return false;
    }

    if (!this._throw.equals(o._throw)) {
      return false;
    }

    if (!this._throws.equals(o._throws)) {
      return false;
    }

    if (!this._transient.equals(o._transient)) {
      return false;
    }

    if (!this._true.equals(o._true)) {
      return false;
    }

    if (!this._try.equals(o._try)) {
      return false;
    }

    if (!this._void.equals(o._void)) {
      return false;
    }

    if (!this._volatile.equals(o._volatile)) {
      return false;
    }

    if (!this._while.equals(o._while)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
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

  public static class Builder {
    private Optional<String> _abstract = Optional.empty();
    private Optional<String> _assert = Optional.empty();
    private Optional<String> _boolean = Optional.empty();
    private Optional<String> _break = Optional.empty();
    private Optional<String> _byte = Optional.empty();
    private Optional<String> _case = Optional.empty();
    private Optional<String> _catch = Optional.empty();
    private Optional<String> _char = Optional.empty();
    private Optional<String> _class = Optional.empty();
    private Optional<String> _const = Optional.empty();
    private Optional<String> _continue = Optional.empty();
    private Optional<String> _default = Optional.empty();
    private Optional<String> _do = Optional.empty();
    private Optional<String> _double = Optional.empty();
    private Optional<String> _else = Optional.empty();
    private Optional<String> _enum = Optional.empty();
    private Optional<String> _extends = Optional.empty();
    private Optional<String> _false = Optional.empty();
    private Optional<String> _final = Optional.empty();
    private Optional<String> _finally = Optional.empty();
    private Optional<String> _float = Optional.empty();
    private Optional<String> _for = Optional.empty();
    private Optional<String> _goto = Optional.empty();
    private Optional<String> _if = Optional.empty();
    private Optional<String> _implements = Optional.empty();
    private Optional<String> _import = Optional.empty();
    private Optional<Empty> imported = Optional.empty();
    private Optional<String> _instanceof = Optional.empty();
    private Optional<String> _int = Optional.empty();
    private Optional<String> _interface = Optional.empty();
    private Optional<String> _long = Optional.empty();
    private Optional<String> _native = Optional.empty();
    private Optional<String> _new = Optional.empty();
    private Optional<String> _null = Optional.empty();
    private Optional<String> _package = Optional.empty();
    private Optional<String> _private = Optional.empty();
    private Optional<String> _protected = Optional.empty();
    private Optional<String> _public = Optional.empty();
    private Optional<String> _return = Optional.empty();
    private Optional<String> _short = Optional.empty();
    private Optional<String> _static = Optional.empty();
    private Optional<String> _strictfp = Optional.empty();
    private Optional<String> _super = Optional.empty();
    private Optional<String> _switch = Optional.empty();
    private Optional<String> _synchronized = Optional.empty();
    private Optional<String> _this = Optional.empty();
    private Optional<String> _throw = Optional.empty();
    private Optional<String> _throws = Optional.empty();
    private Optional<String> _transient = Optional.empty();
    private Optional<String> _true = Optional.empty();
    private Optional<String> _try = Optional.empty();
    private Optional<String> _void = Optional.empty();
    private Optional<String> _volatile = Optional.empty();
    private Optional<String> _while = Optional.empty();

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

    public Entry build() {
      final Optional<String> _abstract = this._abstract;
      final Optional<String> _assert = this._assert;
      final Optional<String> _boolean = this._boolean;
      final Optional<String> _break = this._break;
      final Optional<String> _byte = this._byte;
      final Optional<String> _case = this._case;
      final Optional<String> _catch = this._catch;
      final Optional<String> _char = this._char;
      final Optional<String> _class = this._class;
      final Optional<String> _const = this._const;
      final Optional<String> _continue = this._continue;
      final Optional<String> _default = this._default;
      final Optional<String> _do = this._do;
      final Optional<String> _double = this._double;
      final Optional<String> _else = this._else;
      final Optional<String> _enum = this._enum;
      final Optional<String> _extends = this._extends;
      final Optional<String> _false = this._false;
      final Optional<String> _final = this._final;
      final Optional<String> _finally = this._finally;
      final Optional<String> _float = this._float;
      final Optional<String> _for = this._for;
      final Optional<String> _goto = this._goto;
      final Optional<String> _if = this._if;
      final Optional<String> _implements = this._implements;
      final Optional<String> _import = this._import;
      final Optional<Empty> imported = this.imported;
      final Optional<String> _instanceof = this._instanceof;
      final Optional<String> _int = this._int;
      final Optional<String> _interface = this._interface;
      final Optional<String> _long = this._long;
      final Optional<String> _native = this._native;
      final Optional<String> _new = this._new;
      final Optional<String> _null = this._null;
      final Optional<String> _package = this._package;
      final Optional<String> _private = this._private;
      final Optional<String> _protected = this._protected;
      final Optional<String> _public = this._public;
      final Optional<String> _return = this._return;
      final Optional<String> _short = this._short;
      final Optional<String> _static = this._static;
      final Optional<String> _strictfp = this._strictfp;
      final Optional<String> _super = this._super;
      final Optional<String> _switch = this._switch;
      final Optional<String> _synchronized = this._synchronized;
      final Optional<String> _this = this._this;
      final Optional<String> _throw = this._throw;
      final Optional<String> _throws = this._throws;
      final Optional<String> _transient = this._transient;
      final Optional<String> _true = this._true;
      final Optional<String> _try = this._try;
      final Optional<String> _void = this._void;
      final Optional<String> _volatile = this._volatile;
      final Optional<String> _while = this._while;

      return new Entry(_abstract, _assert, _boolean, _break, _byte, _case, _catch, _char, _class, _const, _continue, _default, _do, _double, _else, _enum, _extends, _false, _final, _finally, _float, _for, _goto, _if, _implements, _import, imported, _instanceof, _int, _interface, _long, _native, _new, _null, _package, _private, _protected, _public, _return, _short, _static, _strictfp, _super, _switch, _synchronized, _this, _throw, _throws, _transient, _true, _try, _void, _volatile, _while);
    }
  }
}
