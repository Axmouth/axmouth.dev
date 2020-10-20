import { AuthTokenNotFoundError } from './auth-token-not-found-error';
import { AuthToken } from './auth-token';

/**
 * Wrapper for simple (text) token
 */
export class AuthSimpleToken extends AuthToken {
  constructor(protected readonly token: any, protected createdAt?: Date) {
    super();
    try {
      this.parsePayload();
    } catch (err) {
      if (!(err instanceof AuthTokenNotFoundError)) {
        // token is present but has got a problem, including illegal
        throw err;
      }
    }
    this.createdAt = this.prepareCreatedAt(createdAt);
  }

  protected parsePayload(): any {
    this.payload = null;
  }

  protected prepareCreatedAt(date: Date): Date {
    return date ? date : new Date();
  }

  /**
   * Returns the token's creation date
   */
  getCreatedAt(): Date {
    return this.createdAt;
  }

  /**
   * Returns the token value
   * @returns string
   */
  getValue(): string {
    return this.token;
  }

  /**
   * Is non empty and valid
   */
  isValid(): boolean {
    return !!this.getValue();
  }

  /**
   * Validate value and convert to string, if value is not valid return empty string
   */
  toString(): string {
    return !!this.token ? this.token : '';
  }
}
