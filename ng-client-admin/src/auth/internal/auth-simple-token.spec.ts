import { AuthSimpleToken } from './auth-simple-token';

describe('AuthSimpleToken', () => {
  it('should create an instance', () => {
    expect(new AuthSimpleToken('', '')).toBeTruthy();
  });
});
