/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {string}
*/
export function evaluate(input: string): string;
/**
* @param {string} input
* @returns {string}
*/
export function read_eval_print(input: string): string;
/**
*/
export class Port {
  free(): void;
}
/**
*/
export class Thing {
  free(): void;
/**
* @param {Function} js_signal
* @returns {Thing}
*/
  static new(js_signal: Function): Thing;
/**
* @param {string} input
* @returns {string}
*/
  eval(input: string): string;
/**
* @param {string} port
* @returns {string}
*/
  read_port(port: string): string;
}
