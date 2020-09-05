import { AuthIllegalJWTTokenError } from './auth-illegal-jwt-token-error';

describe('AuthIllegalJWTTokenError', () => {
  it('should create an instance', () => {
    expect(new AuthIllegalJWTTokenError('Error eror')).toBeTruthy();
  });
});
