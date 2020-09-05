import { AuthIllegalTokenError } from './auth-illegal-token-error';

export class AuthIllegalJWTTokenError extends AuthIllegalTokenError {
  constructor(message: string) {
    super(message);
    Object.setPrototypeOf(this, new.target.prototype);
  }
}
