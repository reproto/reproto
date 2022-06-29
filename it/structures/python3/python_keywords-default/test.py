from _yield import Empty as t

class Entry:
  def __init__(self, _and, _as, _assert, _break, _class, _continue, _def, _del, _elif, _else, _except, _exec, _finally, _for, _from, _global, _if, _import, imported, _in, _is, _lambda, _nonlocal, _not, _or, _pass, _print, _raise, _return, _try, _while, _with, _yield):
    self.__and = _and
    self.__as = _as
    self.__assert = _assert
    self.__break = _break
    self.__class = _class
    self.__continue = _continue
    self.__def = _def
    self.__del = _del
    self.__elif = _elif
    self.__else = _else
    self.__except = _except
    self.__exec = _exec
    self.__finally = _finally
    self.__for = _for
    self.__from = _from
    self.__global = _global
    self.__if = _if
    self.__import = _import
    self.__imported = imported
    self.__in = _in
    self.__is = _is
    self.__lambda = _lambda
    self.__nonlocal = _nonlocal
    self.__not = _not
    self.__or = _or
    self.__pass = _pass
    self.__print = _print
    self.__raise = _raise
    self.__return = _return
    self.__try = _try
    self.__while = _while
    self.__with = _with
    self.__yield = _yield

  @property
  def _and(self):
    return self.__and

  @_and.setter
  def _and(self, _and):
    self.__and = _and

  @property
  def _as(self):
    return self.__as

  @_as.setter
  def _as(self, _as):
    self.__as = _as

  @property
  def _assert(self):
    return self.__assert

  @_assert.setter
  def _assert(self, _assert):
    self.__assert = _assert

  @property
  def _break(self):
    return self.__break

  @_break.setter
  def _break(self, _break):
    self.__break = _break

  @property
  def _class(self):
    return self.__class

  @_class.setter
  def _class(self, _class):
    self.__class = _class

  @property
  def _continue(self):
    return self.__continue

  @_continue.setter
  def _continue(self, _continue):
    self.__continue = _continue

  @property
  def _def(self):
    return self.__def

  @_def.setter
  def _def(self, _def):
    self.__def = _def

  @property
  def _del(self):
    return self.__del

  @_del.setter
  def _del(self, _del):
    self.__del = _del

  @property
  def _elif(self):
    return self.__elif

  @_elif.setter
  def _elif(self, _elif):
    self.__elif = _elif

  @property
  def _else(self):
    return self.__else

  @_else.setter
  def _else(self, _else):
    self.__else = _else

  @property
  def _except(self):
    return self.__except

  @_except.setter
  def _except(self, _except):
    self.__except = _except

  @property
  def _exec(self):
    return self.__exec

  @_exec.setter
  def _exec(self, _exec):
    self.__exec = _exec

  @property
  def _finally(self):
    return self.__finally

  @_finally.setter
  def _finally(self, _finally):
    self.__finally = _finally

  @property
  def _for(self):
    return self.__for

  @_for.setter
  def _for(self, _for):
    self.__for = _for

  @property
  def _from(self):
    return self.__from

  @_from.setter
  def _from(self, _from):
    self.__from = _from

  @property
  def _global(self):
    return self.__global

  @_global.setter
  def _global(self, _global):
    self.__global = _global

  @property
  def _if(self):
    return self.__if

  @_if.setter
  def _if(self, _if):
    self.__if = _if

  @property
  def _import(self):
    return self.__import

  @_import.setter
  def _import(self, _import):
    self.__import = _import

  @property
  def imported(self):
    return self.__imported

  @imported.setter
  def imported(self, imported):
    self.__imported = imported

  @property
  def _in(self):
    return self.__in

  @_in.setter
  def _in(self, _in):
    self.__in = _in

  @property
  def _is(self):
    return self.__is

  @_is.setter
  def _is(self, _is):
    self.__is = _is

  @property
  def _lambda(self):
    return self.__lambda

  @_lambda.setter
  def _lambda(self, _lambda):
    self.__lambda = _lambda

  @property
  def _nonlocal(self):
    return self.__nonlocal

  @_nonlocal.setter
  def _nonlocal(self, _nonlocal):
    self.__nonlocal = _nonlocal

  @property
  def _not(self):
    return self.__not

  @_not.setter
  def _not(self, _not):
    self.__not = _not

  @property
  def _or(self):
    return self.__or

  @_or.setter
  def _or(self, _or):
    self.__or = _or

  @property
  def _pass(self):
    return self.__pass

  @_pass.setter
  def _pass(self, _pass):
    self.__pass = _pass

  @property
  def _print(self):
    return self.__print

  @_print.setter
  def _print(self, _print):
    self.__print = _print

  @property
  def _raise(self):
    return self.__raise

  @_raise.setter
  def _raise(self, _raise):
    self.__raise = _raise

  @property
  def _return(self):
    return self.__return

  @_return.setter
  def _return(self, _return):
    self.__return = _return

  @property
  def _try(self):
    return self.__try

  @_try.setter
  def _try(self, _try):
    self.__try = _try

  @property
  def _while(self):
    return self.__while

  @_while.setter
  def _while(self, _while):
    self.__while = _while

  @property
  def _with(self):
    return self.__with

  @_with.setter
  def _with(self, _with):
    self.__with = _with

  @property
  def _yield(self):
    return self.__yield

  @_yield.setter
  def _yield(self, _yield):
    self.__yield = _yield

  @staticmethod
  def decode(data):
    f_and = None

    if "and" in data:
      f_and = data["and"]

      if f_and is not None:
        if not isinstance(f_and, str):
          raise Exception("not a string")

    f_as = None

    if "as" in data:
      f_as = data["as"]

      if f_as is not None:
        if not isinstance(f_as, str):
          raise Exception("not a string")

    f_assert = None

    if "assert" in data:
      f_assert = data["assert"]

      if f_assert is not None:
        if not isinstance(f_assert, str):
          raise Exception("not a string")

    f_break = None

    if "break" in data:
      f_break = data["break"]

      if f_break is not None:
        if not isinstance(f_break, str):
          raise Exception("not a string")

    f_class = None

    if "class" in data:
      f_class = data["class"]

      if f_class is not None:
        if not isinstance(f_class, str):
          raise Exception("not a string")

    f_continue = None

    if "continue" in data:
      f_continue = data["continue"]

      if f_continue is not None:
        if not isinstance(f_continue, str):
          raise Exception("not a string")

    f_def = None

    if "def" in data:
      f_def = data["def"]

      if f_def is not None:
        if not isinstance(f_def, str):
          raise Exception("not a string")

    f_del = None

    if "del" in data:
      f_del = data["del"]

      if f_del is not None:
        if not isinstance(f_del, str):
          raise Exception("not a string")

    f_elif = None

    if "elif" in data:
      f_elif = data["elif"]

      if f_elif is not None:
        if not isinstance(f_elif, str):
          raise Exception("not a string")

    f_else = None

    if "else" in data:
      f_else = data["else"]

      if f_else is not None:
        if not isinstance(f_else, str):
          raise Exception("not a string")

    f_except = None

    if "except" in data:
      f_except = data["except"]

      if f_except is not None:
        if not isinstance(f_except, str):
          raise Exception("not a string")

    f_exec = None

    if "exec" in data:
      f_exec = data["exec"]

      if f_exec is not None:
        if not isinstance(f_exec, str):
          raise Exception("not a string")

    f_finally = None

    if "finally" in data:
      f_finally = data["finally"]

      if f_finally is not None:
        if not isinstance(f_finally, str):
          raise Exception("not a string")

    f_for = None

    if "for" in data:
      f_for = data["for"]

      if f_for is not None:
        if not isinstance(f_for, str):
          raise Exception("not a string")

    f_from = None

    if "from" in data:
      f_from = data["from"]

      if f_from is not None:
        if not isinstance(f_from, str):
          raise Exception("not a string")

    f_global = None

    if "global" in data:
      f_global = data["global"]

      if f_global is not None:
        if not isinstance(f_global, str):
          raise Exception("not a string")

    f_if = None

    if "if" in data:
      f_if = data["if"]

      if f_if is not None:
        if not isinstance(f_if, str):
          raise Exception("not a string")

    f_import = None

    if "import" in data:
      f_import = data["import"]

      if f_import is not None:
        if not isinstance(f_import, str):
          raise Exception("not a string")

    f_imported = None

    if "imported" in data:
      f_imported = data["imported"]

      if f_imported is not None:
        f_imported = t.decode(f_imported)

    f_in = None

    if "in" in data:
      f_in = data["in"]

      if f_in is not None:
        if not isinstance(f_in, str):
          raise Exception("not a string")

    f_is = None

    if "is" in data:
      f_is = data["is"]

      if f_is is not None:
        if not isinstance(f_is, str):
          raise Exception("not a string")

    f_lambda = None

    if "lambda" in data:
      f_lambda = data["lambda"]

      if f_lambda is not None:
        if not isinstance(f_lambda, str):
          raise Exception("not a string")

    f_nonlocal = None

    if "nonlocal" in data:
      f_nonlocal = data["nonlocal"]

      if f_nonlocal is not None:
        if not isinstance(f_nonlocal, str):
          raise Exception("not a string")

    f_not = None

    if "not" in data:
      f_not = data["not"]

      if f_not is not None:
        if not isinstance(f_not, str):
          raise Exception("not a string")

    f_or = None

    if "or" in data:
      f_or = data["or"]

      if f_or is not None:
        if not isinstance(f_or, str):
          raise Exception("not a string")

    f_pass = None

    if "pass" in data:
      f_pass = data["pass"]

      if f_pass is not None:
        if not isinstance(f_pass, str):
          raise Exception("not a string")

    f_print = None

    if "print" in data:
      f_print = data["print"]

      if f_print is not None:
        if not isinstance(f_print, str):
          raise Exception("not a string")

    f_raise = None

    if "raise" in data:
      f_raise = data["raise"]

      if f_raise is not None:
        if not isinstance(f_raise, str):
          raise Exception("not a string")

    f_return = None

    if "return" in data:
      f_return = data["return"]

      if f_return is not None:
        if not isinstance(f_return, str):
          raise Exception("not a string")

    f_try = None

    if "try" in data:
      f_try = data["try"]

      if f_try is not None:
        if not isinstance(f_try, str):
          raise Exception("not a string")

    f_while = None

    if "while" in data:
      f_while = data["while"]

      if f_while is not None:
        if not isinstance(f_while, str):
          raise Exception("not a string")

    f_with = None

    if "with" in data:
      f_with = data["with"]

      if f_with is not None:
        if not isinstance(f_with, str):
          raise Exception("not a string")

    f_yield = None

    if "yield" in data:
      f_yield = data["yield"]

      if f_yield is not None:
        if not isinstance(f_yield, str):
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

    if self.imported is not None:
      data["imported"] = self.imported.encode()

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
    return "<Entry and:{!r}, as:{!r}, assert:{!r}, break:{!r}, class:{!r}, continue:{!r}, def:{!r}, del:{!r}, elif:{!r}, else:{!r}, except:{!r}, exec:{!r}, finally:{!r}, for:{!r}, from:{!r}, global:{!r}, if:{!r}, import:{!r}, imported:{!r}, in:{!r}, is:{!r}, lambda:{!r}, nonlocal:{!r}, not:{!r}, or:{!r}, pass:{!r}, print:{!r}, raise:{!r}, return:{!r}, try:{!r}, while:{!r}, with:{!r}, yield:{!r}>".format(self._and, self._as, self._assert, self._break, self._class, self._continue, self._def, self._del, self._elif, self._else, self._except, self._exec, self._finally, self._for, self._from, self._global, self._if, self._import, self.imported, self._in, self._is, self._lambda, self._nonlocal, self._not, self._or, self._pass, self._print, self._raise, self._return, self._try, self._while, self._with, self._yield)
