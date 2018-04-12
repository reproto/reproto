import _yield as t

class Entry:
  def __init__(self, _and, _as, _assert, _break, _class, _continue, _def, _del, _elif, _else, _except, _exec, _finally, _for, _from, _global, _if, _import, imported, _in, _is, _lambda, _nonlocal, _not, _or, _pass, _print, _raise, _return, _try, _while, _with, _yield):
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
    self.imported = imported
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

  def get_and(self):
    return self._and

  def get_as(self):
    return self._as

  def get_assert(self):
    return self._assert

  def get_break(self):
    return self._break

  def get_class(self):
    return self._class

  def get_continue(self):
    return self._continue

  def get_def(self):
    return self._def

  def get_del(self):
    return self._del

  def get_elif(self):
    return self._elif

  def get_else(self):
    return self._else

  def get_except(self):
    return self._except

  def get_exec(self):
    return self._exec

  def get_finally(self):
    return self._finally

  def get_for(self):
    return self._for

  def get_from(self):
    return self._from

  def get_global(self):
    return self._global

  def get_if(self):
    return self._if

  def get_import(self):
    return self._import

  def get_imported(self):
    return self.imported

  def get_in(self):
    return self._in

  def get_is(self):
    return self._is

  def get_lambda(self):
    return self._lambda

  def get_nonlocal(self):
    return self._nonlocal

  def get_not(self):
    return self._not

  def get_or(self):
    return self._or

  def get_pass(self):
    return self._pass

  def get_print(self):
    return self._print

  def get_raise(self):
    return self._raise

  def get_return(self):
    return self._return

  def get_try(self):
    return self._try

  def get_while(self):
    return self._while

  def get_with(self):
    return self._with

  def get_yield(self):
    return self._yield

  @staticmethod
  def decode(data):
    f__and = None

    if "and" in data:
      f__and = data["and"]

      if f__and is not None:
        if not isinstance(f__and, unicode):
          raise Exception("not a string")

    f__as = None

    if "as" in data:
      f__as = data["as"]

      if f__as is not None:
        if not isinstance(f__as, unicode):
          raise Exception("not a string")

    f__assert = None

    if "assert" in data:
      f__assert = data["assert"]

      if f__assert is not None:
        if not isinstance(f__assert, unicode):
          raise Exception("not a string")

    f__break = None

    if "break" in data:
      f__break = data["break"]

      if f__break is not None:
        if not isinstance(f__break, unicode):
          raise Exception("not a string")

    f__class = None

    if "class" in data:
      f__class = data["class"]

      if f__class is not None:
        if not isinstance(f__class, unicode):
          raise Exception("not a string")

    f__continue = None

    if "continue" in data:
      f__continue = data["continue"]

      if f__continue is not None:
        if not isinstance(f__continue, unicode):
          raise Exception("not a string")

    f__def = None

    if "def" in data:
      f__def = data["def"]

      if f__def is not None:
        if not isinstance(f__def, unicode):
          raise Exception("not a string")

    f__del = None

    if "del" in data:
      f__del = data["del"]

      if f__del is not None:
        if not isinstance(f__del, unicode):
          raise Exception("not a string")

    f__elif = None

    if "elif" in data:
      f__elif = data["elif"]

      if f__elif is not None:
        if not isinstance(f__elif, unicode):
          raise Exception("not a string")

    f__else = None

    if "else" in data:
      f__else = data["else"]

      if f__else is not None:
        if not isinstance(f__else, unicode):
          raise Exception("not a string")

    f__except = None

    if "except" in data:
      f__except = data["except"]

      if f__except is not None:
        if not isinstance(f__except, unicode):
          raise Exception("not a string")

    f__exec = None

    if "exec" in data:
      f__exec = data["exec"]

      if f__exec is not None:
        if not isinstance(f__exec, unicode):
          raise Exception("not a string")

    f__finally = None

    if "finally" in data:
      f__finally = data["finally"]

      if f__finally is not None:
        if not isinstance(f__finally, unicode):
          raise Exception("not a string")

    f__for = None

    if "for" in data:
      f__for = data["for"]

      if f__for is not None:
        if not isinstance(f__for, unicode):
          raise Exception("not a string")

    f__from = None

    if "from" in data:
      f__from = data["from"]

      if f__from is not None:
        if not isinstance(f__from, unicode):
          raise Exception("not a string")

    f__global = None

    if "global" in data:
      f__global = data["global"]

      if f__global is not None:
        if not isinstance(f__global, unicode):
          raise Exception("not a string")

    f__if = None

    if "if" in data:
      f__if = data["if"]

      if f__if is not None:
        if not isinstance(f__if, unicode):
          raise Exception("not a string")

    f__import = None

    if "import" in data:
      f__import = data["import"]

      if f__import is not None:
        if not isinstance(f__import, unicode):
          raise Exception("not a string")

    f_imported = None

    if "imported" in data:
      f_imported = data["imported"]

      if f_imported is not None:
        f_imported = t.Empty.decode(f_imported)

    f__in = None

    if "in" in data:
      f__in = data["in"]

      if f__in is not None:
        if not isinstance(f__in, unicode):
          raise Exception("not a string")

    f__is = None

    if "is" in data:
      f__is = data["is"]

      if f__is is not None:
        if not isinstance(f__is, unicode):
          raise Exception("not a string")

    f__lambda = None

    if "lambda" in data:
      f__lambda = data["lambda"]

      if f__lambda is not None:
        if not isinstance(f__lambda, unicode):
          raise Exception("not a string")

    f__nonlocal = None

    if "nonlocal" in data:
      f__nonlocal = data["nonlocal"]

      if f__nonlocal is not None:
        if not isinstance(f__nonlocal, unicode):
          raise Exception("not a string")

    f__not = None

    if "not" in data:
      f__not = data["not"]

      if f__not is not None:
        if not isinstance(f__not, unicode):
          raise Exception("not a string")

    f__or = None

    if "or" in data:
      f__or = data["or"]

      if f__or is not None:
        if not isinstance(f__or, unicode):
          raise Exception("not a string")

    f__pass = None

    if "pass" in data:
      f__pass = data["pass"]

      if f__pass is not None:
        if not isinstance(f__pass, unicode):
          raise Exception("not a string")

    f__print = None

    if "print" in data:
      f__print = data["print"]

      if f__print is not None:
        if not isinstance(f__print, unicode):
          raise Exception("not a string")

    f__raise = None

    if "raise" in data:
      f__raise = data["raise"]

      if f__raise is not None:
        if not isinstance(f__raise, unicode):
          raise Exception("not a string")

    f__return = None

    if "return" in data:
      f__return = data["return"]

      if f__return is not None:
        if not isinstance(f__return, unicode):
          raise Exception("not a string")

    f__try = None

    if "try" in data:
      f__try = data["try"]

      if f__try is not None:
        if not isinstance(f__try, unicode):
          raise Exception("not a string")

    f__while = None

    if "while" in data:
      f__while = data["while"]

      if f__while is not None:
        if not isinstance(f__while, unicode):
          raise Exception("not a string")

    f__with = None

    if "with" in data:
      f__with = data["with"]

      if f__with is not None:
        if not isinstance(f__with, unicode):
          raise Exception("not a string")

    f__yield = None

    if "yield" in data:
      f__yield = data["yield"]

      if f__yield is not None:
        if not isinstance(f__yield, unicode):
          raise Exception("not a string")

    return Entry(f__and, f__as, f__assert, f__break, f__class, f__continue, f__def, f__del, f__elif, f__else, f__except, f__exec, f__finally, f__for, f__from, f__global, f__if, f__import, f_imported, f__in, f__is, f__lambda, f__nonlocal, f__not, f__or, f__pass, f__print, f__raise, f__return, f__try, f__while, f__with, f__yield)

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
