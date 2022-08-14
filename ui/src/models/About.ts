/*
struct About<'a> {
    api_version: String,
    servername: String,
    hostname: String,
    software: Vec<&'a str>,
}
*/
class About {
  api_version: string;
  servername: string;
  hostname: string;
  software: string[];

  constructor(
    api_version: string,
    servername: string,
    hostname: string,
    software: string[]
  ) {
    this.api_version = api_version;
    this.servername = servername;
    this.hostname = hostname;
    this.software = software;
  }

  toObject() {
    return {
      api_version: this.api_version,
      servername: this.servername,
      hostname: this.hostname,
      software: this.software,
    };
  }

  serialize() {
    return JSON.stringify(this.toObject());
  }

  static fromJSON(serialized: string): About {
    let about: ReturnType<About["toObject"]> = JSON.parse(serialized);

    return new About(
      about.api_version,
      about.servername,
      about.hostname,
      about.software
    );
  }
}

export { About };
