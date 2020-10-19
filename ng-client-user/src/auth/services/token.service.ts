import { Injectable, Inject, PLATFORM_ID, OnDestroy } from '@angular/core';
import { BehaviorSubject, Observable, of, Subject } from 'rxjs';
import { filter, share, takeUntil } from 'rxjs/operators';
import { TokenPack } from '../internal/token-pack';
import { AuthToken } from '../internal/auth-token';
import { isPlatformBrowser } from '@angular/common';
import { AuthCreateJWTToken } from '../internal/auth-jwt-token';
import { AX_AUTH_OPTIONS } from '../auth-injection-token';
import { AuthModuleOptionsConfig } from '../auth-module-options-config';

@Injectable({
  providedIn: 'root',
})
export class TokenService implements OnDestroy {
  ngUnsubscribe = new Subject<void>();
  protected token$: BehaviorSubject<AuthToken> = new BehaviorSubject(null);

  protected key: string;

  constructor(@Inject(PLATFORM_ID) private platform: object, @Inject(AX_AUTH_OPTIONS) config: AuthModuleOptionsConfig) {
    if (isPlatformBrowser(platform)) {
      this.key = config.jwtTokenKey ?? 'auth_app_token';
      this.publishStoredToken();
    }
  }

  /**
   * Returns observable of current token
   */
  get(): Observable<AuthToken> {
    // const token = this.tokenStorage.get();
    if (!isPlatformBrowser(this.platform)) {
      return of(this.unwrap('')).pipe(takeUntil(this.ngUnsubscribe));
    }
    const raw = localStorage.getItem(this.key);
    const token = this.unwrap(raw);
    return of(token).pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Sets a token into the storage. This method is used by the AuthService automatically.
   *
   */
  set(token: AuthToken): Observable<null> {
    if (!isPlatformBrowser(this.platform)) {
      return of(null).pipe(takeUntil(this.ngUnsubscribe));
    }
    const raw = this.wrap(token);
    localStorage.setItem(this.key, raw);
    this.publishStoredToken();
    return of(null).pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Removes the token and published token value
   *
   */
  clear(): Observable<null> {
    // this.tokenStorage.clear();
    if (!isPlatformBrowser(this.platform)) {
      return of(null).pipe(takeUntil(this.ngUnsubscribe));
    }
    localStorage.removeItem(this.key);
    this.publishStoredToken();
    return of(null).pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Publishes token when it changes.
   */
  tokenChange(): Observable<AuthToken> {
    return this.token$
      .pipe(
        filter((value) => !!value),
        share(),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  protected publishStoredToken(): void {
    if (!isPlatformBrowser(this.platform)) {
      return;
    }
    const raw = localStorage.getItem(this.key);
    const token = this.unwrap(raw);
    this.token$.next(token);
  }

  protected wrap(token: AuthToken): string {
    return JSON.stringify({
      name: token.getName(),
      createdAt: token.getCreatedAt().getTime(),
      value: token.toString(),
    });
  }

  protected unwrap(value: string): AuthToken {
    // let tokenClass: AuthTokenClass = this.fallbackClass;
    let tokenValue = '';
    let tokenOwnerStrategyName = '';
    let tokenCreatedAt: Date = null;

    const tokenPack: TokenPack = this.parseTokenPack(value);
    if (tokenPack) {
      // tokenClass = this.getClassByName(tokenPack.name) || this.fallbackClass;
      tokenValue = tokenPack.value;
      tokenOwnerStrategyName = tokenPack.ownerStrategyName;
      tokenCreatedAt = new Date(Number(tokenPack.createdAt));
    }

    return AuthCreateJWTToken(tokenValue, tokenCreatedAt);
  }

  protected parseTokenPack(value: string): TokenPack {
    try {
      return JSON.parse(value);
    } catch (e) {}
    return null;
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
