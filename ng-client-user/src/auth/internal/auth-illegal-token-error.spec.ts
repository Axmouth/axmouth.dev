import { AuthIllegalTokenError } from './auth-illegal-token-error';

describe('AuthIllegalTokenError', () => {
  it('should create an instance', () => {
    expect(new AuthIllegalTokenError('Error error')).toBeTruthy();
  });
});
