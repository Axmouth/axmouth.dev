import { AuthTokenClass } from './auth-token-class';

export abstract class AuthToken {
  protected payload: any = null;

  abstract getValue(): string;
  abstract isValid(): boolean;
  // the strategy name used to acquire this token (needed for refreshing token)
  abstract getCreatedAt(): Date;
  abstract toString(): string;

  getName(): string {
    return (this.constructor as AuthTokenClass).NAME;
  }

  getPayload(): any {
    return this.payload;
  }
}
