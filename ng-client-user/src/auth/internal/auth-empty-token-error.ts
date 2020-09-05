import { AuthIllegalTokenError } from './auth-illegal-token-error';

export class AuthEmptyTokenError extends AuthIllegalTokenError {
  constructor(message: string) {
    super(message);
    Object.setPrototypeOf(this, new.target.prototype);
  }
}
