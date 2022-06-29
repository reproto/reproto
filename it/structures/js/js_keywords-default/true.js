export class Empty {
  constructor() {}

  static decode(data) {

    return new Empty();
  }

  encode() {
    const data = {};

    return data;
  }
}
