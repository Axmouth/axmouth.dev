import { AuthToken } from './auth-token';

export interface AuthTokenClass<T = AuthToken> {
  NAME: string;
  new (raw: any, strategyName: string, expDate?: Date): T;
}
