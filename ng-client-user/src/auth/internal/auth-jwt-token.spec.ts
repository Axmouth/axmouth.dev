import { AuthJWTToken } from './auth-jwt-token';

describe('AuthJWTToken', () => {
  it('should create an instance', () => {
    expect(new AuthJWTToken('', '')).toBeTruthy();
  });
});
