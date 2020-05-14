import { Response as NativeResponse } from '../native';

export class Response {
  private resp: NativeResponse;
  private uidsMap: Map<string, string>;

  constructor(resp: NativeResponse) {
    this.resp = resp;
  }

  public getJson(): any {
    return this.resp.getJson();
  }

  public getUidsMap(): Map<string, string> {
    if (!this.uidsMap) {
      this.uidsMap = new Map(Object.entries(this.resp.getUidsMap()));
    }

    return this.uidsMap;
  }
}
