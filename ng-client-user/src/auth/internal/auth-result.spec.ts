import { AuthResult } from './auth-result';

describe('AuthResult', () => {
  it('should create an instance', () => {
    expect(new AuthResult(true)).toBeTruthy();
  });
});
