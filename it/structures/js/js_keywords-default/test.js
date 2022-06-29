import {Empty as t} from "true.js";

export class Entry {
  constructor(_abstract, _await, _boolean, _break, _byte, _case, _catch, _char, _class, _const, _continue, _debugger, _default, _delete, _do, _double, _else, _enum, _export, _extends, _false, _final, _finally, _float, _for, _function, _goto, _if, _implements, _import, imported, _in, _instanceof, _int, _interface, _let, _long, _native, _new, _package, _private, _protected, _public, _return, _short, _static, _super, _switch, _synchronized, _this, _throw, _throws, _transient, _true, _try, _typeof, _var, _void, _volatile, _while, _with, _yield) {
    this._abstract = _abstract;
    this._await = _await;
    this._boolean = _boolean;
    this._break = _break;
    this._byte = _byte;
    this._case = _case;
    this._catch = _catch;
    this._char = _char;
    this._class = _class;
    this._const = _const;
    this._continue = _continue;
    this._debugger = _debugger;
    this._default = _default;
    this._delete = _delete;
    this._do = _do;
    this._double = _double;
    this._else = _else;
    this._enum = _enum;
    this._export = _export;
    this._extends = _extends;
    this._false = _false;
    this._final = _final;
    this._finally = _finally;
    this._float = _float;
    this._for = _for;
    this._function = _function;
    this._goto = _goto;
    this._if = _if;
    this._implements = _implements;
    this._import = _import;
    this.imported = imported;
    this._in = _in;
    this._instanceof = _instanceof;
    this._int = _int;
    this._interface = _interface;
    this._let = _let;
    this._long = _long;
    this._native = _native;
    this._new = _new;
    this._package = _package;
    this._private = _private;
    this._protected = _protected;
    this._public = _public;
    this._return = _return;
    this._short = _short;
    this._static = _static;
    this._super = _super;
    this._switch = _switch;
    this._synchronized = _synchronized;
    this._this = _this;
    this._throw = _throw;
    this._throws = _throws;
    this._transient = _transient;
    this._true = _true;
    this._try = _try;
    this._typeof = _typeof;
    this._var = _var;
    this._void = _void;
    this._volatile = _volatile;
    this._while = _while;
    this._with = _with;
    this._yield = _yield;
  }

  static decode(data) {
    let v_abstract = data["abstract"];

    if (v_abstract !== null && v_abstract !== undefined) {
      if (typeof v_abstract !== "string") {
        throw Error("expected string");
      }
    } else {
      v_abstract = null;
    }

    let v_await = data["await"];

    if (v_await !== null && v_await !== undefined) {
      if (typeof v_await !== "string") {
        throw Error("expected string");
      }
    } else {
      v_await = null;
    }

    let v_boolean = data["boolean"];

    if (v_boolean !== null && v_boolean !== undefined) {
      if (typeof v_boolean !== "string") {
        throw Error("expected string");
      }
    } else {
      v_boolean = null;
    }

    let v_break = data["break"];

    if (v_break !== null && v_break !== undefined) {
      if (typeof v_break !== "string") {
        throw Error("expected string");
      }
    } else {
      v_break = null;
    }

    let v_byte = data["byte"];

    if (v_byte !== null && v_byte !== undefined) {
      if (typeof v_byte !== "string") {
        throw Error("expected string");
      }
    } else {
      v_byte = null;
    }

    let v_case = data["case"];

    if (v_case !== null && v_case !== undefined) {
      if (typeof v_case !== "string") {
        throw Error("expected string");
      }
    } else {
      v_case = null;
    }

    let v_catch = data["catch"];

    if (v_catch !== null && v_catch !== undefined) {
      if (typeof v_catch !== "string") {
        throw Error("expected string");
      }
    } else {
      v_catch = null;
    }

    let v_char = data["char"];

    if (v_char !== null && v_char !== undefined) {
      if (typeof v_char !== "string") {
        throw Error("expected string");
      }
    } else {
      v_char = null;
    }

    let v_class = data["class"];

    if (v_class !== null && v_class !== undefined) {
      if (typeof v_class !== "string") {
        throw Error("expected string");
      }
    } else {
      v_class = null;
    }

    let v_const = data["const"];

    if (v_const !== null && v_const !== undefined) {
      if (typeof v_const !== "string") {
        throw Error("expected string");
      }
    } else {
      v_const = null;
    }

    let v_continue = data["continue"];

    if (v_continue !== null && v_continue !== undefined) {
      if (typeof v_continue !== "string") {
        throw Error("expected string");
      }
    } else {
      v_continue = null;
    }

    let v_debugger = data["debugger"];

    if (v_debugger !== null && v_debugger !== undefined) {
      if (typeof v_debugger !== "string") {
        throw Error("expected string");
      }
    } else {
      v_debugger = null;
    }

    let v_default = data["default"];

    if (v_default !== null && v_default !== undefined) {
      if (typeof v_default !== "string") {
        throw Error("expected string");
      }
    } else {
      v_default = null;
    }

    let v_delete = data["delete"];

    if (v_delete !== null && v_delete !== undefined) {
      if (typeof v_delete !== "string") {
        throw Error("expected string");
      }
    } else {
      v_delete = null;
    }

    let v_do = data["do"];

    if (v_do !== null && v_do !== undefined) {
      if (typeof v_do !== "string") {
        throw Error("expected string");
      }
    } else {
      v_do = null;
    }

    let v_double = data["double"];

    if (v_double !== null && v_double !== undefined) {
      if (typeof v_double !== "string") {
        throw Error("expected string");
      }
    } else {
      v_double = null;
    }

    let v_else = data["else"];

    if (v_else !== null && v_else !== undefined) {
      if (typeof v_else !== "string") {
        throw Error("expected string");
      }
    } else {
      v_else = null;
    }

    let v_enum = data["enum"];

    if (v_enum !== null && v_enum !== undefined) {
      if (typeof v_enum !== "string") {
        throw Error("expected string");
      }
    } else {
      v_enum = null;
    }

    let v_export = data["export"];

    if (v_export !== null && v_export !== undefined) {
      if (typeof v_export !== "string") {
        throw Error("expected string");
      }
    } else {
      v_export = null;
    }

    let v_extends = data["extends"];

    if (v_extends !== null && v_extends !== undefined) {
      if (typeof v_extends !== "string") {
        throw Error("expected string");
      }
    } else {
      v_extends = null;
    }

    let v_false = data["false"];

    if (v_false !== null && v_false !== undefined) {
      if (typeof v_false !== "string") {
        throw Error("expected string");
      }
    } else {
      v_false = null;
    }

    let v_final = data["final"];

    if (v_final !== null && v_final !== undefined) {
      if (typeof v_final !== "string") {
        throw Error("expected string");
      }
    } else {
      v_final = null;
    }

    let v_finally = data["finally"];

    if (v_finally !== null && v_finally !== undefined) {
      if (typeof v_finally !== "string") {
        throw Error("expected string");
      }
    } else {
      v_finally = null;
    }

    let v_float = data["float"];

    if (v_float !== null && v_float !== undefined) {
      if (typeof v_float !== "string") {
        throw Error("expected string");
      }
    } else {
      v_float = null;
    }

    let v_for = data["for"];

    if (v_for !== null && v_for !== undefined) {
      if (typeof v_for !== "string") {
        throw Error("expected string");
      }
    } else {
      v_for = null;
    }

    let v_function = data["function"];

    if (v_function !== null && v_function !== undefined) {
      if (typeof v_function !== "string") {
        throw Error("expected string");
      }
    } else {
      v_function = null;
    }

    let v_goto = data["goto"];

    if (v_goto !== null && v_goto !== undefined) {
      if (typeof v_goto !== "string") {
        throw Error("expected string");
      }
    } else {
      v_goto = null;
    }

    let v_if = data["if"];

    if (v_if !== null && v_if !== undefined) {
      if (typeof v_if !== "string") {
        throw Error("expected string");
      }
    } else {
      v_if = null;
    }

    let v_implements = data["implements"];

    if (v_implements !== null && v_implements !== undefined) {
      if (typeof v_implements !== "string") {
        throw Error("expected string");
      }
    } else {
      v_implements = null;
    }

    let v_import = data["import"];

    if (v_import !== null && v_import !== undefined) {
      if (typeof v_import !== "string") {
        throw Error("expected string");
      }
    } else {
      v_import = null;
    }

    let v_imported = data["imported"];

    if (v_imported !== null && v_imported !== undefined) {
      v_imported = t.decode(v_imported);
    } else {
      v_imported = null;
    }

    let v_in = data["in"];

    if (v_in !== null && v_in !== undefined) {
      if (typeof v_in !== "string") {
        throw Error("expected string");
      }
    } else {
      v_in = null;
    }

    let v_instanceof = data["instanceof"];

    if (v_instanceof !== null && v_instanceof !== undefined) {
      if (typeof v_instanceof !== "string") {
        throw Error("expected string");
      }
    } else {
      v_instanceof = null;
    }

    let v_int = data["int"];

    if (v_int !== null && v_int !== undefined) {
      if (typeof v_int !== "string") {
        throw Error("expected string");
      }
    } else {
      v_int = null;
    }

    let v_interface = data["interface"];

    if (v_interface !== null && v_interface !== undefined) {
      if (typeof v_interface !== "string") {
        throw Error("expected string");
      }
    } else {
      v_interface = null;
    }

    let v_let = data["let"];

    if (v_let !== null && v_let !== undefined) {
      if (typeof v_let !== "string") {
        throw Error("expected string");
      }
    } else {
      v_let = null;
    }

    let v_long = data["long"];

    if (v_long !== null && v_long !== undefined) {
      if (typeof v_long !== "string") {
        throw Error("expected string");
      }
    } else {
      v_long = null;
    }

    let v_native = data["native"];

    if (v_native !== null && v_native !== undefined) {
      if (typeof v_native !== "string") {
        throw Error("expected string");
      }
    } else {
      v_native = null;
    }

    let v_new = data["new"];

    if (v_new !== null && v_new !== undefined) {
      if (typeof v_new !== "string") {
        throw Error("expected string");
      }
    } else {
      v_new = null;
    }

    let v_package = data["package"];

    if (v_package !== null && v_package !== undefined) {
      if (typeof v_package !== "string") {
        throw Error("expected string");
      }
    } else {
      v_package = null;
    }

    let v_private = data["private"];

    if (v_private !== null && v_private !== undefined) {
      if (typeof v_private !== "string") {
        throw Error("expected string");
      }
    } else {
      v_private = null;
    }

    let v_protected = data["protected"];

    if (v_protected !== null && v_protected !== undefined) {
      if (typeof v_protected !== "string") {
        throw Error("expected string");
      }
    } else {
      v_protected = null;
    }

    let v_public = data["public"];

    if (v_public !== null && v_public !== undefined) {
      if (typeof v_public !== "string") {
        throw Error("expected string");
      }
    } else {
      v_public = null;
    }

    let v_return = data["return"];

    if (v_return !== null && v_return !== undefined) {
      if (typeof v_return !== "string") {
        throw Error("expected string");
      }
    } else {
      v_return = null;
    }

    let v_short = data["short"];

    if (v_short !== null && v_short !== undefined) {
      if (typeof v_short !== "string") {
        throw Error("expected string");
      }
    } else {
      v_short = null;
    }

    let v_static = data["static"];

    if (v_static !== null && v_static !== undefined) {
      if (typeof v_static !== "string") {
        throw Error("expected string");
      }
    } else {
      v_static = null;
    }

    let v_super = data["super"];

    if (v_super !== null && v_super !== undefined) {
      if (typeof v_super !== "string") {
        throw Error("expected string");
      }
    } else {
      v_super = null;
    }

    let v_switch = data["switch"];

    if (v_switch !== null && v_switch !== undefined) {
      if (typeof v_switch !== "string") {
        throw Error("expected string");
      }
    } else {
      v_switch = null;
    }

    let v_synchronized = data["synchronized"];

    if (v_synchronized !== null && v_synchronized !== undefined) {
      if (typeof v_synchronized !== "string") {
        throw Error("expected string");
      }
    } else {
      v_synchronized = null;
    }

    let v_this = data["this"];

    if (v_this !== null && v_this !== undefined) {
      if (typeof v_this !== "string") {
        throw Error("expected string");
      }
    } else {
      v_this = null;
    }

    let v_throw = data["throw"];

    if (v_throw !== null && v_throw !== undefined) {
      if (typeof v_throw !== "string") {
        throw Error("expected string");
      }
    } else {
      v_throw = null;
    }

    let v_throws = data["throws"];

    if (v_throws !== null && v_throws !== undefined) {
      if (typeof v_throws !== "string") {
        throw Error("expected string");
      }
    } else {
      v_throws = null;
    }

    let v_transient = data["transient"];

    if (v_transient !== null && v_transient !== undefined) {
      if (typeof v_transient !== "string") {
        throw Error("expected string");
      }
    } else {
      v_transient = null;
    }

    let v_true = data["true"];

    if (v_true !== null && v_true !== undefined) {
      if (typeof v_true !== "string") {
        throw Error("expected string");
      }
    } else {
      v_true = null;
    }

    let v_try = data["try"];

    if (v_try !== null && v_try !== undefined) {
      if (typeof v_try !== "string") {
        throw Error("expected string");
      }
    } else {
      v_try = null;
    }

    let v_typeof = data["typeof"];

    if (v_typeof !== null && v_typeof !== undefined) {
      if (typeof v_typeof !== "string") {
        throw Error("expected string");
      }
    } else {
      v_typeof = null;
    }

    let v_var = data["var"];

    if (v_var !== null && v_var !== undefined) {
      if (typeof v_var !== "string") {
        throw Error("expected string");
      }
    } else {
      v_var = null;
    }

    let v_void = data["void"];

    if (v_void !== null && v_void !== undefined) {
      if (typeof v_void !== "string") {
        throw Error("expected string");
      }
    } else {
      v_void = null;
    }

    let v_volatile = data["volatile"];

    if (v_volatile !== null && v_volatile !== undefined) {
      if (typeof v_volatile !== "string") {
        throw Error("expected string");
      }
    } else {
      v_volatile = null;
    }

    let v_while = data["while"];

    if (v_while !== null && v_while !== undefined) {
      if (typeof v_while !== "string") {
        throw Error("expected string");
      }
    } else {
      v_while = null;
    }

    let v_with = data["with"];

    if (v_with !== null && v_with !== undefined) {
      if (typeof v_with !== "string") {
        throw Error("expected string");
      }
    } else {
      v_with = null;
    }

    let v_yield = data["yield"];

    if (v_yield !== null && v_yield !== undefined) {
      if (typeof v_yield !== "string") {
        throw Error("expected string");
      }
    } else {
      v_yield = null;
    }

    return new Entry(v_abstract, v_await, v_boolean, v_break, v_byte, v_case, v_catch, v_char, v_class, v_const, v_continue, v_debugger, v_default, v_delete, v_do, v_double, v_else, v_enum, v_export, v_extends, v_false, v_final, v_finally, v_float, v_for, v_function, v_goto, v_if, v_implements, v_import, v_imported, v_in, v_instanceof, v_int, v_interface, v_let, v_long, v_native, v_new, v_package, v_private, v_protected, v_public, v_return, v_short, v_static, v_super, v_switch, v_synchronized, v_this, v_throw, v_throws, v_transient, v_true, v_try, v_typeof, v_var, v_void, v_volatile, v_while, v_with, v_yield);
  }

  encode() {
    const data = {};

    if (this._abstract !== null && this._abstract !== undefined) {
      data["abstract"] = this._abstract;
    }

    if (this._await !== null && this._await !== undefined) {
      data["await"] = this._await;
    }

    if (this._boolean !== null && this._boolean !== undefined) {
      data["boolean"] = this._boolean;
    }

    if (this._break !== null && this._break !== undefined) {
      data["break"] = this._break;
    }

    if (this._byte !== null && this._byte !== undefined) {
      data["byte"] = this._byte;
    }

    if (this._case !== null && this._case !== undefined) {
      data["case"] = this._case;
    }

    if (this._catch !== null && this._catch !== undefined) {
      data["catch"] = this._catch;
    }

    if (this._char !== null && this._char !== undefined) {
      data["char"] = this._char;
    }

    if (this._class !== null && this._class !== undefined) {
      data["class"] = this._class;
    }

    if (this._const !== null && this._const !== undefined) {
      data["const"] = this._const;
    }

    if (this._continue !== null && this._continue !== undefined) {
      data["continue"] = this._continue;
    }

    if (this._debugger !== null && this._debugger !== undefined) {
      data["debugger"] = this._debugger;
    }

    if (this._default !== null && this._default !== undefined) {
      data["default"] = this._default;
    }

    if (this._delete !== null && this._delete !== undefined) {
      data["delete"] = this._delete;
    }

    if (this._do !== null && this._do !== undefined) {
      data["do"] = this._do;
    }

    if (this._double !== null && this._double !== undefined) {
      data["double"] = this._double;
    }

    if (this._else !== null && this._else !== undefined) {
      data["else"] = this._else;
    }

    if (this._enum !== null && this._enum !== undefined) {
      data["enum"] = this._enum;
    }

    if (this._export !== null && this._export !== undefined) {
      data["export"] = this._export;
    }

    if (this._extends !== null && this._extends !== undefined) {
      data["extends"] = this._extends;
    }

    if (this._false !== null && this._false !== undefined) {
      data["false"] = this._false;
    }

    if (this._final !== null && this._final !== undefined) {
      data["final"] = this._final;
    }

    if (this._finally !== null && this._finally !== undefined) {
      data["finally"] = this._finally;
    }

    if (this._float !== null && this._float !== undefined) {
      data["float"] = this._float;
    }

    if (this._for !== null && this._for !== undefined) {
      data["for"] = this._for;
    }

    if (this._function !== null && this._function !== undefined) {
      data["function"] = this._function;
    }

    if (this._goto !== null && this._goto !== undefined) {
      data["goto"] = this._goto;
    }

    if (this._if !== null && this._if !== undefined) {
      data["if"] = this._if;
    }

    if (this._implements !== null && this._implements !== undefined) {
      data["implements"] = this._implements;
    }

    if (this._import !== null && this._import !== undefined) {
      data["import"] = this._import;
    }

    if (this.imported !== null && this.imported !== undefined) {
      data["imported"] = this.imported.encode();
    }

    if (this._in !== null && this._in !== undefined) {
      data["in"] = this._in;
    }

    if (this._instanceof !== null && this._instanceof !== undefined) {
      data["instanceof"] = this._instanceof;
    }

    if (this._int !== null && this._int !== undefined) {
      data["int"] = this._int;
    }

    if (this._interface !== null && this._interface !== undefined) {
      data["interface"] = this._interface;
    }

    if (this._let !== null && this._let !== undefined) {
      data["let"] = this._let;
    }

    if (this._long !== null && this._long !== undefined) {
      data["long"] = this._long;
    }

    if (this._native !== null && this._native !== undefined) {
      data["native"] = this._native;
    }

    if (this._new !== null && this._new !== undefined) {
      data["new"] = this._new;
    }

    if (this._package !== null && this._package !== undefined) {
      data["package"] = this._package;
    }

    if (this._private !== null && this._private !== undefined) {
      data["private"] = this._private;
    }

    if (this._protected !== null && this._protected !== undefined) {
      data["protected"] = this._protected;
    }

    if (this._public !== null && this._public !== undefined) {
      data["public"] = this._public;
    }

    if (this._return !== null && this._return !== undefined) {
      data["return"] = this._return;
    }

    if (this._short !== null && this._short !== undefined) {
      data["short"] = this._short;
    }

    if (this._static !== null && this._static !== undefined) {
      data["static"] = this._static;
    }

    if (this._super !== null && this._super !== undefined) {
      data["super"] = this._super;
    }

    if (this._switch !== null && this._switch !== undefined) {
      data["switch"] = this._switch;
    }

    if (this._synchronized !== null && this._synchronized !== undefined) {
      data["synchronized"] = this._synchronized;
    }

    if (this._this !== null && this._this !== undefined) {
      data["this"] = this._this;
    }

    if (this._throw !== null && this._throw !== undefined) {
      data["throw"] = this._throw;
    }

    if (this._throws !== null && this._throws !== undefined) {
      data["throws"] = this._throws;
    }

    if (this._transient !== null && this._transient !== undefined) {
      data["transient"] = this._transient;
    }

    if (this._true !== null && this._true !== undefined) {
      data["true"] = this._true;
    }

    if (this._try !== null && this._try !== undefined) {
      data["try"] = this._try;
    }

    if (this._typeof !== null && this._typeof !== undefined) {
      data["typeof"] = this._typeof;
    }

    if (this._var !== null && this._var !== undefined) {
      data["var"] = this._var;
    }

    if (this._void !== null && this._void !== undefined) {
      data["void"] = this._void;
    }

    if (this._volatile !== null && this._volatile !== undefined) {
      data["volatile"] = this._volatile;
    }

    if (this._while !== null && this._while !== undefined) {
      data["while"] = this._while;
    }

    if (this._with !== null && this._with !== undefined) {
      data["with"] = this._with;
    }

    if (this._yield !== null && this._yield !== undefined) {
      data["yield"] = this._yield;
    }

    return data;
  }
}
