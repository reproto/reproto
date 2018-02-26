import * as t from "yield.js";

export class Entry {
  constructor(as, and, assert, _break, _class, _continue, def, del, elif, _else, except, exec, _finally, _for, from, global, _if, _import, _in, is, lambda, nonlocal, not, or, pass, print, raise, _return, _try, _while, _with, _yield, imported) {
    this.as = as;
    this.and = and;
    this.assert = assert;
    this._break = _break;
    this._class = _class;
    this._continue = _continue;
    this.def = def;
    this.del = del;
    this.elif = elif;
    this._else = _else;
    this.except = except;
    this.exec = exec;
    this._finally = _finally;
    this._for = _for;
    this.from = from;
    this.global = global;
    this._if = _if;
    this._import = _import;
    this._in = _in;
    this.is = is;
    this.lambda = lambda;
    this.nonlocal = nonlocal;
    this.not = not;
    this.or = or;
    this.pass = pass;
    this.print = print;
    this.raise = raise;
    this._return = _return;
    this._try = _try;
    this._while = _while;
    this._with = _with;
    this._yield = _yield;
    this.imported = imported;
  }

  static decode(data) {
    let v_as = data["as"];

    if (v_as !== null && v_as !== undefined) {
      v_as = v_as;
    } else {
      v_as = null;
    }

    let v_and = data["and"];

    if (v_and !== null && v_and !== undefined) {
      v_and = v_and;
    } else {
      v_and = null;
    }

    let v_assert = data["assert"];

    if (v_assert !== null && v_assert !== undefined) {
      v_assert = v_assert;
    } else {
      v_assert = null;
    }

    let v_break = data["break"];

    if (v_break !== null && v_break !== undefined) {
      v_break = v_break;
    } else {
      v_break = null;
    }

    let v_class = data["class"];

    if (v_class !== null && v_class !== undefined) {
      v_class = v_class;
    } else {
      v_class = null;
    }

    let v_continue = data["continue"];

    if (v_continue !== null && v_continue !== undefined) {
      v_continue = v_continue;
    } else {
      v_continue = null;
    }

    let v_def = data["def"];

    if (v_def !== null && v_def !== undefined) {
      v_def = v_def;
    } else {
      v_def = null;
    }

    let v_del = data["del"];

    if (v_del !== null && v_del !== undefined) {
      v_del = v_del;
    } else {
      v_del = null;
    }

    let v_elif = data["elif"];

    if (v_elif !== null && v_elif !== undefined) {
      v_elif = v_elif;
    } else {
      v_elif = null;
    }

    let v_else = data["else"];

    if (v_else !== null && v_else !== undefined) {
      v_else = v_else;
    } else {
      v_else = null;
    }

    let v_except = data["except"];

    if (v_except !== null && v_except !== undefined) {
      v_except = v_except;
    } else {
      v_except = null;
    }

    let v_exec = data["exec"];

    if (v_exec !== null && v_exec !== undefined) {
      v_exec = v_exec;
    } else {
      v_exec = null;
    }

    let v_finally = data["finally"];

    if (v_finally !== null && v_finally !== undefined) {
      v_finally = v_finally;
    } else {
      v_finally = null;
    }

    let v_for = data["for"];

    if (v_for !== null && v_for !== undefined) {
      v_for = v_for;
    } else {
      v_for = null;
    }

    let v_from = data["from"];

    if (v_from !== null && v_from !== undefined) {
      v_from = v_from;
    } else {
      v_from = null;
    }

    let v_global = data["global"];

    if (v_global !== null && v_global !== undefined) {
      v_global = v_global;
    } else {
      v_global = null;
    }

    let v_if = data["if"];

    if (v_if !== null && v_if !== undefined) {
      v_if = v_if;
    } else {
      v_if = null;
    }

    let v_import = data["import"];

    if (v_import !== null && v_import !== undefined) {
      v_import = v_import;
    } else {
      v_import = null;
    }

    let v_in = data["in"];

    if (v_in !== null && v_in !== undefined) {
      v_in = v_in;
    } else {
      v_in = null;
    }

    let v_is = data["is"];

    if (v_is !== null && v_is !== undefined) {
      v_is = v_is;
    } else {
      v_is = null;
    }

    let v_lambda = data["lambda"];

    if (v_lambda !== null && v_lambda !== undefined) {
      v_lambda = v_lambda;
    } else {
      v_lambda = null;
    }

    let v_nonlocal = data["nonlocal"];

    if (v_nonlocal !== null && v_nonlocal !== undefined) {
      v_nonlocal = v_nonlocal;
    } else {
      v_nonlocal = null;
    }

    let v_not = data["not"];

    if (v_not !== null && v_not !== undefined) {
      v_not = v_not;
    } else {
      v_not = null;
    }

    let v_or = data["or"];

    if (v_or !== null && v_or !== undefined) {
      v_or = v_or;
    } else {
      v_or = null;
    }

    let v_pass = data["pass"];

    if (v_pass !== null && v_pass !== undefined) {
      v_pass = v_pass;
    } else {
      v_pass = null;
    }

    let v_print = data["print"];

    if (v_print !== null && v_print !== undefined) {
      v_print = v_print;
    } else {
      v_print = null;
    }

    let v_raise = data["raise"];

    if (v_raise !== null && v_raise !== undefined) {
      v_raise = v_raise;
    } else {
      v_raise = null;
    }

    let v_return = data["return"];

    if (v_return !== null && v_return !== undefined) {
      v_return = v_return;
    } else {
      v_return = null;
    }

    let v_try = data["try"];

    if (v_try !== null && v_try !== undefined) {
      v_try = v_try;
    } else {
      v_try = null;
    }

    let v_while = data["while"];

    if (v_while !== null && v_while !== undefined) {
      v_while = v_while;
    } else {
      v_while = null;
    }

    let v_with = data["with"];

    if (v_with !== null && v_with !== undefined) {
      v_with = v_with;
    } else {
      v_with = null;
    }

    let v_yield = data["yield"];

    if (v_yield !== null && v_yield !== undefined) {
      v_yield = v_yield;
    } else {
      v_yield = null;
    }

    let v_imported = data["imported"];

    if (v_imported !== null && v_imported !== undefined) {
      v_imported = t.Empty.decode(v_imported);
    } else {
      v_imported = null;
    }

    return new Entry(v_as, v_and, v_assert, v_break, v_class, v_continue, v_def, v_del, v_elif, v_else, v_except, v_exec, v_finally, v_for, v_from, v_global, v_if, v_import, v_in, v_is, v_lambda, v_nonlocal, v_not, v_or, v_pass, v_print, v_raise, v_return, v_try, v_while, v_with, v_yield, v_imported);
  }

  encode() {
    const data = {};

    if (this.as !== null && this.as !== undefined) {
      data["as"] = this.as;
    }

    if (this.and !== null && this.and !== undefined) {
      data["and"] = this.and;
    }

    if (this.assert !== null && this.assert !== undefined) {
      data["assert"] = this.assert;
    }

    if (this._break !== null && this._break !== undefined) {
      data["break"] = this._break;
    }

    if (this._class !== null && this._class !== undefined) {
      data["class"] = this._class;
    }

    if (this._continue !== null && this._continue !== undefined) {
      data["continue"] = this._continue;
    }

    if (this.def !== null && this.def !== undefined) {
      data["def"] = this.def;
    }

    if (this.del !== null && this.del !== undefined) {
      data["del"] = this.del;
    }

    if (this.elif !== null && this.elif !== undefined) {
      data["elif"] = this.elif;
    }

    if (this._else !== null && this._else !== undefined) {
      data["else"] = this._else;
    }

    if (this.except !== null && this.except !== undefined) {
      data["except"] = this.except;
    }

    if (this.exec !== null && this.exec !== undefined) {
      data["exec"] = this.exec;
    }

    if (this._finally !== null && this._finally !== undefined) {
      data["finally"] = this._finally;
    }

    if (this._for !== null && this._for !== undefined) {
      data["for"] = this._for;
    }

    if (this.from !== null && this.from !== undefined) {
      data["from"] = this.from;
    }

    if (this.global !== null && this.global !== undefined) {
      data["global"] = this.global;
    }

    if (this._if !== null && this._if !== undefined) {
      data["if"] = this._if;
    }

    if (this._import !== null && this._import !== undefined) {
      data["import"] = this._import;
    }

    if (this._in !== null && this._in !== undefined) {
      data["in"] = this._in;
    }

    if (this.is !== null && this.is !== undefined) {
      data["is"] = this.is;
    }

    if (this.lambda !== null && this.lambda !== undefined) {
      data["lambda"] = this.lambda;
    }

    if (this.nonlocal !== null && this.nonlocal !== undefined) {
      data["nonlocal"] = this.nonlocal;
    }

    if (this.not !== null && this.not !== undefined) {
      data["not"] = this.not;
    }

    if (this.or !== null && this.or !== undefined) {
      data["or"] = this.or;
    }

    if (this.pass !== null && this.pass !== undefined) {
      data["pass"] = this.pass;
    }

    if (this.print !== null && this.print !== undefined) {
      data["print"] = this.print;
    }

    if (this.raise !== null && this.raise !== undefined) {
      data["raise"] = this.raise;
    }

    if (this._return !== null && this._return !== undefined) {
      data["return"] = this._return;
    }

    if (this._try !== null && this._try !== undefined) {
      data["try"] = this._try;
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

    if (this.imported !== null && this.imported !== undefined) {
      data["imported"] = this.imported.encode();
    }

    return data;
  }
}
