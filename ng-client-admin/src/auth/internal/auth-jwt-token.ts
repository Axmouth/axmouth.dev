import { AuthTokenNotFoundError } from './auth-token-not-found-error';
import { AuthSimpleToken } from './auth-simple-token';
import { AuthIllegalJWTTokenError } from './auth-illegal-jwt-token-error';
import { AuthEmptyTokenError } from './auth-empty-token-error';

/**
 * Wrapper for JWT token with additional methods.
 */
export class AuthJWTToken extends AuthSimpleToken {
  static NAME = 'axmouth.dev:auth:jwt:token';

  /**
   * for JWT token, the iat (issued at) field of the token payload contains the creation Date
   */
  protected prepareCreatedAt(date: Date) {
    const decoded = this.getPayload();
    return decoded && decoded.iat ? new Date(Number(decoded.iat) * 1000) : super.prepareCreatedAt(date);
  }

  /**
   * Returns payload object
   * @returns any
   */
  protected parsePayload(): void {
    if (!this.token) {
      throw new AuthTokenNotFoundError('Token not found. ');
    }
    this.payload = decodeJwtPayload(this.token);
  }

  /**
   * Returns expiration date
   * @returns Date
   */
  getTokenExpDate(): Date {
    const decoded = this.getPayload();
    if (decoded && !decoded.hasOwnProperty('exp')) {
      return null;
    }
    const date = new Date(0);
    date.setUTCSeconds(decoded.exp); // 'cause jwt token are set in seconds
    return date;
  }

  /**
   * Is data expired
   */
  isValid(): boolean {
    return super.isValid() && (!this.getTokenExpDate() || new Date() < this.getTokenExpDate());
  }
}

export function AuthCreateJWTToken(token: any, ownerStrategyName: string, createdAt?: Date) {
  return new AuthJWTToken(token, ownerStrategyName, createdAt);
}

export function decodeJwtPayload(payload: string): any {
  if (payload.length === 0) {
    throw new AuthEmptyTokenError('Cannot extract from an empty payload.');
  }

  const parts = payload.split('.');

  if (parts.length !== 3) {
    throw new AuthIllegalJWTTokenError(
      `The payload ${payload} is not valid JWT payload and must consist of three parts.`,
    );
  }

  let decoded: string;
  try {
    decoded = urlBase64Decode(parts[1]);
  } catch (e) {
    throw new AuthIllegalJWTTokenError(`The payload ${payload} is not valid JWT payload and cannot be parsed.`);
  }

  if (!decoded) {
    throw new AuthIllegalJWTTokenError(`The payload ${payload} is not valid JWT payload and cannot be decoded.`);
  }
  return JSON.parse(decoded);
}

export function urlBase64Decode(str: string): string {
  let output = str.replace(/-/g, '+').replace(/_/g, '/');
  switch (output.length % 4) {
    case 0: {
      break;
    }
    case 2: {
      output += '==';
      break;
    }
    case 3: {
      output += '=';
      break;
    }
    default: {
      throw new Error('Illegal base64url string!');
    }
  }
  return b64DecodeUnicode(output);
}

export function b64decode(str: string): string {
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=';
  let output = '';

  str = String(str).replace(/=+$/, '');

  if (str.length % 4 === 1) {
    throw new Error(`'atob' failed: The string to be decoded is not correctly encoded.`);
  }

  for (
    // initialize result and counters
    let bc = 0, bs: any, buffer: any, idx = 0;
    // get next character
    // tslint:disable-next-line:no-conditional-assignment
    (buffer = str.charAt(idx++));
    // character found in table? initialize bit storage and add its ascii value;
    // tslint:disable-next-line:no-bitwise
    ~buffer &&
    // tslint:disable-next-line:no-conditional-assignment
    ((bs = bc % 4 ? bs * 64 + buffer : buffer),
    // and if not first of each 4 characters,
    // convert the first 8 bits to one ascii character
    bc++ % 4)
      ? // tslint:disable-next-line:no-bitwise
        (output += String.fromCharCode(255 & (bs >> ((-2 * bc) & 6))))
      : 0
  ) {
    // try to find character in table (0-63, not found => -1)
    buffer = chars.indexOf(buffer);
  }
  return output;
}

// https://developer.mozilla.org/en/docs/Web/API/WindowBase64/Base64_encoding_and_decoding#The_Unicode_Problem
export function b64DecodeUnicode(str: any) {
  return decodeURIComponent(
    Array.prototype.map
      .call(b64decode(str), (c: any) => {
        return '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2);
      })
      .join(''),
  );
}
