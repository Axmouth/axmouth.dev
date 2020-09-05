import { AuthEmptyTokenError } from './auth-empty-token-error';

describe('AuthEmptyTokenError', () => {
  it('should create an instance', () => {
    expect(new AuthEmptyTokenError('Error error')).toBeTruthy();
  });
});
