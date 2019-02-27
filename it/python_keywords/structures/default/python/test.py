import _yield as t

class Entry:
  def __init__(self, _and, _as, _assert, _break, _class, _continue, _def, _del, _elif, _else, _except, _exec, _finally, _for, _from, _global, _if, _import, _imported, _in, _is, _lambda, _nonlocal, _not, _or, _pass, _print, _raise, _return, _try, _while, _with, _yield):
    self._and = _and
    self._as = _as
    self._assert = _assert
    self._break = _break
    self._class = _class
    self._continue = _continue
    self._def = _def
    self._del = _del
    self._elif = _elif
    self._else = _else
    self._except = _except
    self._exec = _exec
    self._finally = _finally
    self._for = _for
    self._from = _from
    self._global = _global
    self._if = _if
    self._import = _import
    self._imported = _imported
    self._in = _in
    self._is = _is
    self._lambda = _lambda
    self._nonlocal = _nonlocal
    self._not = _not
    self._or = _or
    self._pass = _pass
    self._print = _print
    self._raise = _raise
    self._return = _return
    self._try = _try
    self._while = _while
    self._with = _with
    self._yield = _yield

  @property
  def and(self):
    return self._and

  @property
  def as(self):
    return self._as

  @property
  def assert(self):
    return self._assert

  @property
  def break(self):
    return self._break

  @property
  def class(self):
    return self._class

  @property
  def continue(self):
    return self._continue

  @property
  def def(self):
    return self._def

  @property
  def del(self):
    return self._del

  @property
  def elif(self):
    return self._elif

  @property
  def else(self):
    return self._else

  @property
  def except(self):
    return self._except

  @property
  def exec(self):
    return self._exec

  @property
  def finally(self):
    return self._finally

  @property
  def for(self):
    return self._for

  @property
  def from(self):
    return self._from

  @property
  def global(self):
    return self._global

  @property
  def if(self):
    return self._if

  @property
  def import(self):
    return self._import

  @property
  def imported(self):
    return self._imported

  @property
  def in(self):
    return self._in

  @property
  def is(self):
    return self._is

  @property
  def lambda(self):
    return self._lambda

  @property
  def nonlocal(self):
    return self._nonlocal

  @property
  def not(self):
    return self._not

  @property
  def or(self):
    return self._or

  @property
  def pass(self):
    return self._pass

  @property
  def print(self):
    return self._print

  @property
  def raise(self):
    return self._raise

  @property
  def return(self):
    return self._return

  @property
  def try(self):
    return self._try

  @property
  def while(self):
    return self._while

  @property
  def with(self):
    return self._with

  @property
  def yield(self):
    return self._yield

  @staticmethod
  def decode(data):
    f_and = None

    if "and" in data:
      f_and = data["and"]

      if f_and is not None:
        if not isinstance(f_and, unicode):
          raise Exception("not a string")

    f_as = None

    if "as" in data:
      f_as = data["as"]

      if f_as is not None:
        if not isinstance(f_as, unicode):
          raise Exception("not a string")

    f_assert = None

    if "assert" in data:
      f_assert = data["assert"]

      if f_assert is not None:
        if not isinstance(f_assert, unicode):
          raise Exception("not a string")

    f_break = None

    if "break" in data:
      f_break = data["break"]

      if f_break is not None:
        if not isinstance(f_break, unicode):
          raise Exception("not a string")

    f_class = None

    if "class" in data:
      f_class = data["class"]

      if f_class is not None:
        if not isinstance(f_class, unicode):
          raise Exception("not a string")

    f_continue = None

    if "continue" in data:
      f_continue = data["continue"]

      if f_continue is not None:
        if not isinstance(f_continue, unicode):
          raise Exception("not a string")

    f_def = None

    if "def" in data:
      f_def = data["def"]

      if f_def is not None:
        if not isinstance(f_def, unicode):
          raise Exception("not a string")

    f_del = None

    if "del" in data:
      f_del = data["del"]

      if f_del is not None:
        if not isinstance(f_del, unicode):
          raise Exception("not a string")

    f_elif = None

    if "elif" in data:
      f_elif = data["elif"]

      if f_elif is not None:
        if not isinstance(f_elif, unicode):
          raise Exception("not a string")

    f_else = None

    if "else" in data:
      f_else = data["else"]

      if f_else is not None:
        if not isinstance(f_else, unicode):
          raise Exception("not a string")

    f_except = None

    if "except" in data:
      f_except = data["except"]

      if f_except is not None:
        if not isinstance(f_except, unicode):
          raise Exception("not a string")

    f_exec = None

    if "exec" in data:
      f_exec = data["exec"]

      if f_exec is not None:
        if not isinstance(f_exec, unicode):
          raise Exception("not a string")

    f_finally = None

    if "finally" in data:
      f_finally = data["finally"]

      if f_finally is not None:
        if not isinstance(f_finally, unicode):
          raise Exception("not a string")

    f_for = None

    if "for" in data:
      f_for = data["for"]

      if f_for is not None:
        if not isinstance(f_for, unicode):
          raise Exception("not a string")

    f_from = None

    if "from" in data:
      f_from = data["from"]

      if f_from is not None:
        if not isinstance(f_from, unicode):
          raise Exception("not a string")

    f_global = None

    if "global" in data:
      f_global = data["global"]

      if f_global is not None:
        if not isinstance(f_global, unicode):
          raise Exception("not a string")

    f_if = None

    if "if" in data:
      f_if = data["if"]

      if f_if is not None:
        if not isinstance(f_if, unicode):
          raise Exception("not a string")

    f_import = None

    if "import" in data:
      f_import = data["import"]

      if f_import is not None:
        if not isinstance(f_import, unicode):
          raise Exception("not a string")

    f_imported = None

    if "imported" in data:
      f_imported = data["imported"]

      if f_imported is not None:
        f_imported = t.Empty.decode(f_imported)

    f_in = None

    if "in" in data:
      f_in = data["in"]

      if f_in is not None:
        if not isinstance(f_in, unicode):
          raise Exception("not a string")

    f_is = None

    if "is" in data:
      f_is = data["is"]

      if f_is is not None:
        if not isinstance(f_is, unicode):
          raise Exception("not a string")

    f_lambda = None

    if "lambda" in data:
      f_lambda = data["lambda"]

      if f_lambda is not None:
        if not isinstance(f_lambda, unicode):
          raise Exception("not a string")

    f_nonlocal = None

    if "nonlocal" in data:
      f_nonlocal = data["nonlocal"]

      if f_nonlocal is not None:
        if not isinstance(f_nonlocal, unicode):
          raise Exception("not a string")

    f_not = None

    if "not" in data:
      f_not = data["not"]

      if f_not is not None:
        if not isinstance(f_not, unicode):
          raise Exception("not a string")

    f_or = None

    if "or" in data:
      f_or = data["or"]

      if f_or is not None:
        if not isinstance(f_or, unicode):
          raise Exception("not a string")

    f_pass = None

    if "pass" in data:
      f_pass = data["pass"]

      if f_pass is not None:
        if not isinstance(f_pass, unicode):
          raise Exception("not a string")

    f_print = None

    if "print" in data:
      f_print = data["print"]

      if f_print is not None:
        if not isinstance(f_print, unicode):
          raise Exception("not a string")

    f_raise = None

    if "raise" in data:
      f_raise = data["raise"]

      if f_raise is not None:
        if not isinstance(f_raise, unicode):
          raise Exception("not a string")

    f_return = None

    if "return" in data:
      f_return = data["return"]

      if f_return is not None:
        if not isinstance(f_return, unicode):
          raise Exception("not a string")

    f_try = None

    if "try" in data:
      f_try = data["try"]

      if f_try is not None:
        if not isinstance(f_try, unicode):
          raise Exception("not a string")

    f_while = None

    if "while" in data:
      f_while = data["while"]

      if f_while is not None:
        if not isinstance(f_while, unicode):
          raise Exception("not a string")

    f_with = None

    if "with" in data:
      f_with = data["with"]

      if f_with is not None:
        if not isinstance(f_with, unicode):
          raise Exception("not a string")

    f_yield = None

    if "yield" in data:
      f_yield = data["yield"]

      if f_yield is not None:
        if not isinstance(f_yield, unicode):
          raise Exception("not a string")

    return Entry(f_and, f_as, f_assert, f_break, f_class, f_continue, f_def, f_del, f_elif, f_else, f_except, f_exec, f_finally, f_for, f_from, f_global, f_if, f_import, f_imported, f_in, f_is, f_lambda, f_nonlocal, f_not, f_or, f_pass, f_print, f_raise, f_return, f_try, f_while, f_with, f_yield)

  def encode(self):
    data = dict()

    if self._and is not None:
      data["and"] = self._and

    if self._as is not None:
      data["as"] = self._as

    if self._assert is not None:
      data["assert"] = self._assert

    if self._break is not None:
      data["break"] = self._break

    if self._class is not None:
      data["class"] = self._class

    if self._continue is not None:
      data["continue"] = self._continue

    if self._def is not None:
      data["def"] = self._def

    if self._del is not None:
      data["del"] = self._del

    if self._elif is not None:
      data["elif"] = self._elif

    if self._else is not None:
      data["else"] = self._else

    if self._except is not None:
      data["except"] = self._except

    if self._exec is not None:
      data["exec"] = self._exec

    if self._finally is not None:
      data["finally"] = self._finally

    if self._for is not None:
      data["for"] = self._for

    if self._from is not None:
      data["from"] = self._from

    if self._global is not None:
      data["global"] = self._global

    if self._if is not None:
      data["if"] = self._if

    if self._import is not None:
      data["import"] = self._import

    if self._imported is not None:
      data["imported"] = self._imported.encode()

    if self._in is not None:
      data["in"] = self._in

    if self._is is not None:
      data["is"] = self._is

    if self._lambda is not None:
      data["lambda"] = self._lambda

    if self._nonlocal is not None:
      data["nonlocal"] = self._nonlocal

    if self._not is not None:
      data["not"] = self._not

    if self._or is not None:
      data["or"] = self._or

    if self._pass is not None:
      data["pass"] = self._pass

    if self._print is not None:
      data["print"] = self._print

    if self._raise is not None:
      data["raise"] = self._raise

    if self._return is not None:
      data["return"] = self._return

    if self._try is not None:
      data["try"] = self._try

    if self._while is not None:
      data["while"] = self._while

    if self._with is not None:
      data["with"] = self._with

    if self._yield is not None:
      data["yield"] = self._yield

    return data

  def __repr__(self):
    return "<Entry and:{!r}, as:{!r}, assert:{!r}, break:{!r}, class:{!r}, continue:{!r}, def:{!r}, del:{!r}, elif:{!r}, else:{!r}, except:{!r}, exec:{!r}, finally:{!r}, for:{!r}, from:{!r}, global:{!r}, if:{!r}, import:{!r}, imported:{!r}, in:{!r}, is:{!r}, lambda:{!r}, nonlocal:{!r}, not:{!r}, or:{!r}, pass:{!r}, print:{!r}, raise:{!r}, return:{!r}, try:{!r}, while:{!r}, with:{!r}, yield:{!r}>".format(self._and, self._as, self._assert, self._break, self._class, self._continue, self._def, self._del, self._elif, self._else, self._except, self._exec, self._finally, self._for, self._from, self._global, self._if, self._import, self._imported, self._in, self._is, self._lambda, self._nonlocal, self._not, self._or, self._pass, self._print, self._raise, self._return, self._try, self._while, self._with, self._yield)
