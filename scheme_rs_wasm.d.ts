/* tslint:disable */
/* eslint-disable */
export function evaluate(input: string): string;
export function read_eval_print(input: string): string;
export class Port {
  private constructor();
  free(): void;
}
export class Thing {
  private constructor();
  free(): void;
  static new(js_signal: Function): Thing;
  eval(input: string): string;
  read_port(port: string): string;
}
