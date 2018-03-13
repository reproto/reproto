import * as lower_camel from "lower_camel.js";
import * as lower_snake from "lower_snake.js";
import * as upper_camel from "upper_camel.js";
import * as upper_snake from "upper_snake.js";

export class Entry {
  constructor(lower_camel, lower_snake, upper_camel, upper_snake) {
    this.lower_camel = lower_camel;
    this.lower_snake = lower_snake;
    this.upper_camel = upper_camel;
    this.upper_snake = upper_snake;
  }

  static decode(data) {
    let v_lower_camel = data["lower_camel"];

    if (v_lower_camel !== null && v_lower_camel !== undefined) {
      v_lower_camel = lower_camel.Value.decode(v_lower_camel);
    } else {
      v_lower_camel = null;
    }

    let v_lower_snake = data["lower_snake"];

    if (v_lower_snake !== null && v_lower_snake !== undefined) {
      v_lower_snake = lower_snake.Value.decode(v_lower_snake);
    } else {
      v_lower_snake = null;
    }

    let v_upper_camel = data["upper_camel"];

    if (v_upper_camel !== null && v_upper_camel !== undefined) {
      v_upper_camel = upper_camel.Value.decode(v_upper_camel);
    } else {
      v_upper_camel = null;
    }

    let v_upper_snake = data["upper_snake"];

    if (v_upper_snake !== null && v_upper_snake !== undefined) {
      v_upper_snake = upper_snake.Value.decode(v_upper_snake);
    } else {
      v_upper_snake = null;
    }

    return new Entry(v_lower_camel, v_lower_snake, v_upper_camel, v_upper_snake);
  }

  encode() {
    const data = {};

    if (this.lower_camel !== null && this.lower_camel !== undefined) {
      data["lower_camel"] = this.lower_camel.encode();
    }

    if (this.lower_snake !== null && this.lower_snake !== undefined) {
      data["lower_snake"] = this.lower_snake.encode();
    }

    if (this.upper_camel !== null && this.upper_camel !== undefined) {
      data["upper_camel"] = this.upper_camel.encode();
    }

    if (this.upper_snake !== null && this.upper_snake !== undefined) {
      data["upper_snake"] = this.upper_snake.encode();
    }

    return data;
  }
}
