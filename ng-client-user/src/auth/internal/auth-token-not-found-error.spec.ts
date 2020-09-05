import { AuthTokenNotFoundError } from './auth-token-not-found-error';

describe('AuthTokenNotFoundError', () => {
  it('should create an instance', () => {
    expect(new AuthTokenNotFoundError('Error error')).toBeTruthy();
  });
});
