public struct Test_Entry {
  let as_: String?
  let associatedtype_: String?
  let associativity_: String?
  let break_: String?
  let case_: String?
  let catch_: String?
  let class_: String?
  let continue_: String?
  let convenience_: String?
  let default_: String?
  let defer_: String?
  let deinit_: String?
  let do_: String?
  let dynamic_: String?
  let else_: String?
  let enum_: String?
  let extension_: String?
  let fallthrough_: String?
  let false_: String?
  let fileprivate_: String?
  let final_: String?
  let for_: String?
  let func_: String?
  let get_: String?
  let guard_: String?
  let if_: String?
  let import_: String?
  let in_: String?
  let indirect_: String?
  let infix_: String?
  let init_: String?
  let inout_: String?
  let internal_: String?
  let is_: String?
  let lazy_: String?
  let left_: String?
  let let_: String?
  let mutating_: String?
  let nil_: String?
  let none_: String?
  let nonmutating_: String?
  let open_: String?
  let operator_: String?
  let optional_: String?
  let override_: String?
  let postfix_: String?
  let precedence_: String?
  let prefix_: String?
  let private_: String?
  let protocol_: String?
  let public_: String?
  let repeat_: String?
  let required_: String?
  let rethrows_: String?
  let return_: String?
  let right_: String?
  let self_: String?
  let set_: String?
  let static_: String?
  let struct_: String?
  let subscript_: String?
  let super_: String?
  let switch_: String?
  let throw_: String?
  let throws_: String?
  let true_: String?
  let try_: String?
  let typealias_: String?
  let unowned_: String?
  let var_: String?
  let weak_: String?
  let where_: String?
  let while_: String?
}

public extension Test_Entry {
  static func decode(json: Any) throws -> Test_Entry {
    let json = try decode_value(json as? [String: Any])

    var as_: String? = Optional.none

    if let value = json["as"] {
      as_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "as"))
    }

    var associatedtype_: String? = Optional.none

    if let value = json["associatedtype"] {
      associatedtype_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "associatedtype"))
    }

    var associativity_: String? = Optional.none

    if let value = json["associativity"] {
      associativity_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "associativity"))
    }

    var break_: String? = Optional.none

    if let value = json["break"] {
      break_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "break"))
    }

    var case_: String? = Optional.none

    if let value = json["case"] {
      case_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "case"))
    }

    var catch_: String? = Optional.none

    if let value = json["catch"] {
      catch_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "catch"))
    }

    var class_: String? = Optional.none

    if let value = json["class"] {
      class_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "class"))
    }

    var continue_: String? = Optional.none

    if let value = json["continue"] {
      continue_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "continue"))
    }

    var convenience_: String? = Optional.none

    if let value = json["convenience"] {
      convenience_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "convenience"))
    }

    var default_: String? = Optional.none

    if let value = json["default"] {
      default_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "default"))
    }

    var defer_: String? = Optional.none

    if let value = json["defer"] {
      defer_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "defer"))
    }

    var deinit_: String? = Optional.none

    if let value = json["deinit"] {
      deinit_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "deinit"))
    }

    var do_: String? = Optional.none

    if let value = json["do"] {
      do_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "do"))
    }

    var dynamic_: String? = Optional.none

    if let value = json["dynamic"] {
      dynamic_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "dynamic"))
    }

    var else_: String? = Optional.none

    if let value = json["else"] {
      else_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "else"))
    }

    var enum_: String? = Optional.none

    if let value = json["enum"] {
      enum_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "enum"))
    }

    var extension_: String? = Optional.none

    if let value = json["extension"] {
      extension_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "extension"))
    }

    var fallthrough_: String? = Optional.none

    if let value = json["fallthrough"] {
      fallthrough_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "fallthrough"))
    }

    var false_: String? = Optional.none

    if let value = json["false"] {
      false_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "false"))
    }

    var fileprivate_: String? = Optional.none

    if let value = json["fileprivate"] {
      fileprivate_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "fileprivate"))
    }

    var final_: String? = Optional.none

    if let value = json["final"] {
      final_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "final"))
    }

    var for_: String? = Optional.none

    if let value = json["for"] {
      for_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "for"))
    }

    var func_: String? = Optional.none

    if let value = json["func"] {
      func_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "func"))
    }

    var get_: String? = Optional.none

    if let value = json["get"] {
      get_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "get"))
    }

    var guard_: String? = Optional.none

    if let value = json["guard"] {
      guard_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "guard"))
    }

    var if_: String? = Optional.none

    if let value = json["if"] {
      if_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "if"))
    }

    var import_: String? = Optional.none

    if let value = json["import"] {
      import_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "import"))
    }

    var in_: String? = Optional.none

    if let value = json["in"] {
      in_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "in"))
    }

    var indirect_: String? = Optional.none

    if let value = json["indirect"] {
      indirect_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "indirect"))
    }

    var infix_: String? = Optional.none

    if let value = json["infix"] {
      infix_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "infix"))
    }

    var init_: String? = Optional.none

    if let value = json["init"] {
      init_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "init"))
    }

    var inout_: String? = Optional.none

    if let value = json["inout"] {
      inout_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "inout"))
    }

    var internal_: String? = Optional.none

    if let value = json["internal"] {
      internal_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "internal"))
    }

    var is_: String? = Optional.none

    if let value = json["is"] {
      is_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "is"))
    }

    var lazy_: String? = Optional.none

    if let value = json["lazy"] {
      lazy_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "lazy"))
    }

    var left_: String? = Optional.none

    if let value = json["left"] {
      left_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "left"))
    }

    var let_: String? = Optional.none

    if let value = json["let"] {
      let_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "let"))
    }

    var mutating_: String? = Optional.none

    if let value = json["mutating"] {
      mutating_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "mutating"))
    }

    var nil_: String? = Optional.none

    if let value = json["nil"] {
      nil_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "nil"))
    }

    var none_: String? = Optional.none

    if let value = json["none"] {
      none_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "none"))
    }

    var nonmutating_: String? = Optional.none

    if let value = json["nonmutating"] {
      nonmutating_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "nonmutating"))
    }

    var open_: String? = Optional.none

    if let value = json["open"] {
      open_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "open"))
    }

    var operator_: String? = Optional.none

    if let value = json["operator"] {
      operator_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "operator"))
    }

    var optional_: String? = Optional.none

    if let value = json["optional"] {
      optional_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "optional"))
    }

    var override_: String? = Optional.none

    if let value = json["override"] {
      override_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "override"))
    }

    var postfix_: String? = Optional.none

    if let value = json["postfix"] {
      postfix_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "postfix"))
    }

    var precedence_: String? = Optional.none

    if let value = json["precedence"] {
      precedence_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "precedence"))
    }

    var prefix_: String? = Optional.none

    if let value = json["prefix"] {
      prefix_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "prefix"))
    }

    var private_: String? = Optional.none

    if let value = json["private"] {
      private_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "private"))
    }

    var protocol_: String? = Optional.none

    if let value = json["protocol"] {
      protocol_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "protocol"))
    }

    var public_: String? = Optional.none

    if let value = json["public"] {
      public_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "public"))
    }

    var repeat_: String? = Optional.none

    if let value = json["repeat"] {
      repeat_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "repeat"))
    }

    var required_: String? = Optional.none

    if let value = json["required"] {
      required_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "required"))
    }

    var rethrows_: String? = Optional.none

    if let value = json["rethrows"] {
      rethrows_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "rethrows"))
    }

    var return_: String? = Optional.none

    if let value = json["return"] {
      return_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "return"))
    }

    var right_: String? = Optional.none

    if let value = json["right"] {
      right_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "right"))
    }

    var self_: String? = Optional.none

    if let value = json["self"] {
      self_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "self"))
    }

    var set_: String? = Optional.none

    if let value = json["set"] {
      set_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "set"))
    }

    var static_: String? = Optional.none

    if let value = json["static"] {
      static_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "static"))
    }

    var struct_: String? = Optional.none

    if let value = json["struct"] {
      struct_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "struct"))
    }

    var subscript_: String? = Optional.none

    if let value = json["subscript"] {
      subscript_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "subscript"))
    }

    var super_: String? = Optional.none

    if let value = json["super"] {
      super_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "super"))
    }

    var switch_: String? = Optional.none

    if let value = json["switch"] {
      switch_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "switch"))
    }

    var throw_: String? = Optional.none

    if let value = json["throw"] {
      throw_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "throw"))
    }

    var throws_: String? = Optional.none

    if let value = json["throws"] {
      throws_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "throws"))
    }

    var true_: String? = Optional.none

    if let value = json["true"] {
      true_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "true"))
    }

    var try_: String? = Optional.none

    if let value = json["try"] {
      try_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "try"))
    }

    var typealias_: String? = Optional.none

    if let value = json["typealias"] {
      typealias_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "typealias"))
    }

    var unowned_: String? = Optional.none

    if let value = json["unowned"] {
      unowned_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "unowned"))
    }

    var var_: String? = Optional.none

    if let value = json["var"] {
      var_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "var"))
    }

    var weak_: String? = Optional.none

    if let value = json["weak"] {
      weak_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "weak"))
    }

    var where_: String? = Optional.none

    if let value = json["where"] {
      where_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "where"))
    }

    var while_: String? = Optional.none

    if let value = json["while"] {
      while_ = Optional.some(try decode_name(unbox(value, as: String.self), name: "while"))
    }

    return Test_Entry(as_: as_, associatedtype_: associatedtype_, associativity_: associativity_, break_: break_, case_: case_, catch_: catch_, class_: class_, continue_: continue_, convenience_: convenience_, default_: default_, defer_: defer_, deinit_: deinit_, do_: do_, dynamic_: dynamic_, else_: else_, enum_: enum_, extension_: extension_, fallthrough_: fallthrough_, false_: false_, fileprivate_: fileprivate_, final_: final_, for_: for_, func_: func_, get_: get_, guard_: guard_, if_: if_, import_: import_, in_: in_, indirect_: indirect_, infix_: infix_, init_: init_, inout_: inout_, internal_: internal_, is_: is_, lazy_: lazy_, left_: left_, let_: let_, mutating_: mutating_, nil_: nil_, none_: none_, nonmutating_: nonmutating_, open_: open_, operator_: operator_, optional_: optional_, override_: override_, postfix_: postfix_, precedence_: precedence_, prefix_: prefix_, private_: private_, protocol_: protocol_, public_: public_, repeat_: repeat_, required_: required_, rethrows_: rethrows_, return_: return_, right_: right_, self_: self_, set_: set_, static_: static_, struct_: struct_, subscript_: subscript_, super_: super_, switch_: switch_, throw_: throw_, throws_: throws_, true_: true_, try_: try_, typealias_: typealias_, unowned_: unowned_, var_: var_, weak_: weak_, where_: where_, while_: while_)
  }

  func encode() throws -> [String: Any] {
    var json = [String: Any]()

    if let value = self.as_ {
      json["as"] = value
    }
    if let value = self.associatedtype_ {
      json["associatedtype"] = value
    }
    if let value = self.associativity_ {
      json["associativity"] = value
    }
    if let value = self.break_ {
      json["break"] = value
    }
    if let value = self.case_ {
      json["case"] = value
    }
    if let value = self.catch_ {
      json["catch"] = value
    }
    if let value = self.class_ {
      json["class"] = value
    }
    if let value = self.continue_ {
      json["continue"] = value
    }
    if let value = self.convenience_ {
      json["convenience"] = value
    }
    if let value = self.default_ {
      json["default"] = value
    }
    if let value = self.defer_ {
      json["defer"] = value
    }
    if let value = self.deinit_ {
      json["deinit"] = value
    }
    if let value = self.do_ {
      json["do"] = value
    }
    if let value = self.dynamic_ {
      json["dynamic"] = value
    }
    if let value = self.else_ {
      json["else"] = value
    }
    if let value = self.enum_ {
      json["enum"] = value
    }
    if let value = self.extension_ {
      json["extension"] = value
    }
    if let value = self.fallthrough_ {
      json["fallthrough"] = value
    }
    if let value = self.false_ {
      json["false"] = value
    }
    if let value = self.fileprivate_ {
      json["fileprivate"] = value
    }
    if let value = self.final_ {
      json["final"] = value
    }
    if let value = self.for_ {
      json["for"] = value
    }
    if let value = self.func_ {
      json["func"] = value
    }
    if let value = self.get_ {
      json["get"] = value
    }
    if let value = self.guard_ {
      json["guard"] = value
    }
    if let value = self.if_ {
      json["if"] = value
    }
    if let value = self.import_ {
      json["import"] = value
    }
    if let value = self.in_ {
      json["in"] = value
    }
    if let value = self.indirect_ {
      json["indirect"] = value
    }
    if let value = self.infix_ {
      json["infix"] = value
    }
    if let value = self.init_ {
      json["init"] = value
    }
    if let value = self.inout_ {
      json["inout"] = value
    }
    if let value = self.internal_ {
      json["internal"] = value
    }
    if let value = self.is_ {
      json["is"] = value
    }
    if let value = self.lazy_ {
      json["lazy"] = value
    }
    if let value = self.left_ {
      json["left"] = value
    }
    if let value = self.let_ {
      json["let"] = value
    }
    if let value = self.mutating_ {
      json["mutating"] = value
    }
    if let value = self.nil_ {
      json["nil"] = value
    }
    if let value = self.none_ {
      json["none"] = value
    }
    if let value = self.nonmutating_ {
      json["nonmutating"] = value
    }
    if let value = self.open_ {
      json["open"] = value
    }
    if let value = self.operator_ {
      json["operator"] = value
    }
    if let value = self.optional_ {
      json["optional"] = value
    }
    if let value = self.override_ {
      json["override"] = value
    }
    if let value = self.postfix_ {
      json["postfix"] = value
    }
    if let value = self.precedence_ {
      json["precedence"] = value
    }
    if let value = self.prefix_ {
      json["prefix"] = value
    }
    if let value = self.private_ {
      json["private"] = value
    }
    if let value = self.protocol_ {
      json["protocol"] = value
    }
    if let value = self.public_ {
      json["public"] = value
    }
    if let value = self.repeat_ {
      json["repeat"] = value
    }
    if let value = self.required_ {
      json["required"] = value
    }
    if let value = self.rethrows_ {
      json["rethrows"] = value
    }
    if let value = self.return_ {
      json["return"] = value
    }
    if let value = self.right_ {
      json["right"] = value
    }
    if let value = self.self_ {
      json["self"] = value
    }
    if let value = self.set_ {
      json["set"] = value
    }
    if let value = self.static_ {
      json["static"] = value
    }
    if let value = self.struct_ {
      json["struct"] = value
    }
    if let value = self.subscript_ {
      json["subscript"] = value
    }
    if let value = self.super_ {
      json["super"] = value
    }
    if let value = self.switch_ {
      json["switch"] = value
    }
    if let value = self.throw_ {
      json["throw"] = value
    }
    if let value = self.throws_ {
      json["throws"] = value
    }
    if let value = self.true_ {
      json["true"] = value
    }
    if let value = self.try_ {
      json["try"] = value
    }
    if let value = self.typealias_ {
      json["typealias"] = value
    }
    if let value = self.unowned_ {
      json["unowned"] = value
    }
    if let value = self.var_ {
      json["var"] = value
    }
    if let value = self.weak_ {
      json["weak"] = value
    }
    if let value = self.where_ {
      json["where"] = value
    }
    if let value = self.while_ {
      json["while"] = value
    }

    return json
  }
}
