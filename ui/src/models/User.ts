/*
struct User<'a> {
    jwt: String,
}
*/
class User {
  jwt: string;

  constructor(jwt: string) {
    this.jwt = jwt;
  }

  toObject() {
    return {
      jwt: this.jwt,
    };
  }

  serialize() {
    return JSON.stringify(this.toObject());
  }

  static fromJSON(serialized: string): User {
    let user: User = JSON.parse(serialized);

    return new User(user.jwt);
  }
}

export { User };
