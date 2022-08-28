/*
struct User<'a> {
    jwt: String,
}
*/

import jwt_decode from "jwt-decode";

class User {
  jwt: string;
  scope: string[];

  constructor(jwt: string) {
    this.jwt = jwt;

    let body: any = jwt_decode(jwt);
    this.scope = body.Scopes;
  }

  isAdmin() {
    return this.scope.includes("admin")
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
