import { Injectable, Inject, OnDestroy } from '@angular/core';
import { TokenService } from './token.service';
import { map, switchMap, catchError, retry, take, takeUntil, timeout } from 'rxjs/operators';
import { Observable, of, Subject } from 'rxjs';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { ActivatedRoute } from '@angular/router';
import { AuthResult } from '../internal/auth-result';
import { AuthToken } from '../internal/auth-token';
import { AuthJWTToken, AuthCreateJWTToken } from '../internal/auth-jwt-token';
import { AuthIllegalTokenError } from '../internal/auth-illegal-token-error';
import { AX_AUTH_OPTIONS } from '../auth-injection-token';
import { AuthModuleOptionsConfig } from '../auth-module-options-config';
import { IsBrowserService } from 'src/auth/helpers/services/is-browser.service';

const tokenGetterDefault = (authRes: any) => {
  return authRes.data?.token;
};

@Injectable({
  providedIn: 'root',
})
export class AuthService implements OnDestroy {
  ngUnsubscribe = new Subject<void>();
  authenticating = false;
  authenticatingNotifier = new Subject<boolean>();
  config: AuthModuleOptionsConfig;
  authEndpointPrefix: string;
  waitingCount = 0;

  constructor(
    private tokenService: TokenService,
    private http: HttpClient,
    private route: ActivatedRoute,
    private isBrowserService: IsBrowserService,
    @Inject(AX_AUTH_OPTIONS) config: AuthModuleOptionsConfig,
  ) {
    this.authEndpointPrefix = config.authEndpointPrefix;
    this.config = config;
  }

  /**
   * Retrieves the logged in user's username
   * It is assumed it stored under sub inside the token
   *
   */
  getUsername(): Observable<string> {
    return this.tokenService
      .get()
      .pipe(
        map((token) => {
          if (!this.isBrowserService.isInBrowser()) {
            return null;
          }
          const payload = token.getPayload();
          if (payload) {
            const userNameKey = this.config.userNameJwtKey ?? 'sub';
            return payload[userNameKey];
          }
          return null;
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Retrieves the logged in user's email
   * It is assumed it stored under email inside the token
   *
   */
  getEmail(): Observable<string> {
    return this.tokenService
      .get()
      .pipe(
        map((token) => {
          if (!this.isBrowserService.isInBrowser()) {
            return null;
          }
          const payload = token.getPayload();
          if (payload) {
            const emailKey = this.config.emailJwtKey ?? 'email';
            return payload[emailKey];
          }
          return null;
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Retrieves the logged in user's email
   * It is assumed it stored under email inside the token
   *
   */
  getProfile<T>(): Observable<T> {
    const url = `${this.authEndpointPrefix}${this.config.profileEndpoint ?? 'profile'}`;
    const result = this.baseApiRequestWithAuth<T>(this.http, url, {}, this.config.profileMethod ?? 'get', undefined)
      .pipe(
        map((res) => {
          return res;
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
    return result;
  }

  /**
   * Authenticates
   * Stores received token in the token storage
   *
   * Example:
   * authenticate('{email: 'email@example.com', password: 'test'})
   * authenticate( {userName: 'email@example.com', password: 'test'})
   * authenticate( {userName: 'username', password: 'test'})
   *
   */
  authenticate(data?: any): Observable<AuthResult> {
    const url = `${this.authEndpointPrefix}${this.config.loginEndpoint ?? 'login'}`;

    const result = this.baseApiRequest(this.http, url, {}, this.config.loginMethod ?? 'post', data).pipe(
      map((res) => {
        const tokenGetter = this.config?.loginTokengetter ?? tokenGetterDefault;
        return new AuthResult(
          true,
          res,
          true,
          [], // ['Login/Email combination is not correct, please try again.'],
          ['You have been successfully logged in.'],
          this.createToken(tokenGetter(res), true),
        );
      }),
      catchError((res) => {
        return this.handleResponseError(res);
      }),
    );
    return result
      .pipe(
        switchMap((authResult: AuthResult) => {
          return this.processResultToken(authResult);
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Sign outs
   * Removes token from the token storage
   *
   * Example:
   * logout('email')
   *
   */
  logout(): Observable<AuthResult> {
    const url = `${this.authEndpointPrefix}${this.config.logoutEndpoint ?? 'logout'}`;
    const result = of({}).pipe(
      switchMap((res: any) => {
        if (!url) {
          return of(res);
        }
        return this.baseApiRequest(this.http, url, {}, this.config.logoutMethod ?? 'delete', undefined);
      }),
      map((res) => {
        return new AuthResult(
          true,
          res,
          true,
          [], // ['Something went wrong, please try again.'],
          ['You have been successfully logged out.'],
        );
      }),
      catchError((res) => {
        return this.handleResponseError(res);
      }),
    );
    return result
      .pipe(
        switchMap((authResult: AuthResult) => {
          if (authResult?.isSuccess()) {
          }
          if (authResult?.getResponse()?.status === 404 || authResult?.getResponse()?.status === '404') {
            return of(authResult);
          }
          return this.tokenService.clear().pipe(map(() => authResult));
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Registers
   * Stores received token in the token storage
   *
   * Example:
   * register('email', {email: 'email@example.com', name: 'Some Name', password: 'test'})
   *
   */
  register(data?: any): Observable<AuthResult> {
    const url = `${this.authEndpointPrefix}${this.config.registerEndpoint ?? 'register'}`;
    const result = this.baseApiRequest(this.http, url, {}, this.config.registerMethod ?? 'post', data).pipe(
      map((res) => {
        const tokenGetter = this.config?.registerTokengetter ?? tokenGetterDefault;
        return new AuthResult(
          true,
          res,
          true,
          [], // ['Something went wrong, please try again.'],
          ['You have been successfully registered.'],
          this.createToken(tokenGetter(res), true),
        );
      }),
      catchError((res) => {
        return this.handleResponseError(res).pipe(takeUntil(this.ngUnsubscribe));
      }),
    );
    return result
      .pipe(
        switchMap((authResult: AuthResult) => {
          return this.processResultToken(authResult).pipe(takeUntil(this.ngUnsubscribe));
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Returns true if auth token is present in the token storage
   */
  isAuthenticated(): Observable<boolean> {
    if (!this.isBrowserService.isInBrowser()) {
      return of(false);
    }
    return this.getToken()
      .pipe(map((token: AuthToken) => token.isValid()))
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Returns true if valid auth token is present in the token storage.
   * If not, calls refreshToken, and returns isAuthenticated() if success, false otherwise
   */
  isAuthenticatedOrRefresh(callback$?: Observable<any>): Observable<boolean> {
    if (!this.isBrowserService.isInBrowser()) {
      return of(false);
    }
    if (this.authenticating) {
      // check if auth request is in progress and do nothing then
      return this.authenticatingNotifier
        .pipe(take(1))
        .pipe(takeUntil(this.ngUnsubscribe))
        .pipe(timeout(1000))
        .pipe(
          catchError((err, caught) => {
            this.authenticating = false;
            this.authenticatingNotifier.next(false);
            return of(false);
          }),
        );
    }
    this.authenticating = true;
    return this.getToken()
      .pipe(
        switchMap((token) => {
          if (token.getValue() && !token.isValid()) {
            return this.refreshToken(token, callback$).pipe(
              switchMap((res) => {
                if (res === null) {
                  // For the case where there is an auth request in progress. Keep the status quo
                  return of(true);
                }
                if (res.isSuccess()) {
                  return this.isAuthenticated();
                } else {
                  if (
                    res.getResponse().status === 404 ||
                    res.getResponse().status === '404' ||
                    res.getResponse().status >= 500 ||
                    res.getResponse().status >= '500'
                  ) {
                    return this.isAuthenticated();
                  }
                  return of(false);
                }
              }),
            );
          } else {
            return of(token.isValid());
          }
        }),
      )
      .pipe(
        map((t) => {
          this.authenticating = false;
          this.authenticatingNotifier.next(t);
          return t;
        }),
      );
  }

  /**
   * Returns authentication status stream
   */
  onAuthenticationChange(): Observable<boolean> {
    if (!this.isBrowserService.isInBrowser()) {
      return of(false).pipe(takeUntil(this.ngUnsubscribe));
    }
    return this.onTokenChange()
      .pipe(map((token: AuthToken) => token.isValid()))
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Sends a refresh token request
   * Stores received token in the token storage
   *
   * Example:
   * refreshToken({token: token})
   *
   */
  refreshToken(data?: any, callback$?: Observable<any>): Observable<AuthResult> {
    const url = `${this.authEndpointPrefix}${this.config.refreshEndpoint ?? 'refresh'}`;
    const refresh$ = this.baseApiRequest(this.http, url, {}, this.config.refreshMethod ?? 'post', data)
      .pipe(
        map((res) => {
          const tokenGetter = this.config?.refreshTokengetter ?? tokenGetterDefault;
          const token = AuthCreateJWTToken(tokenGetter(res));
          const authResult = new AuthResult(
            true,
            res,
            true,
            [],
            ['Your token has been successfully refreshed.'],
            token,
          );
          return authResult;
        }),
        catchError((res) => {
          return this.handleResponseError(res);
        }),
      )
      .pipe(
        switchMap((result: AuthResult) => {
          return this.processResultToken(result);
        }),
      );
    if (callback$ === undefined) {
      callback$ = of(null);
    }
    return callback$
      .pipe(
        switchMap((_) => {
          return refresh$;
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  protected handleResponseError(res: any): Observable<AuthResult> {
    return of(new AuthResult(false, res, false, ''));
  }

  /**
   * Retrieves current authenticated token stored
   */
  getToken(): Observable<any> {
    return this.tokenService.get().pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Returns tokens stream
   */
  onTokenChange(): Observable<AuthToken> {
    return this.tokenService.tokenChange().pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Sends forgot password request
   *
   * Example:
   * requestPasswordReset({email: 'email@example.com'})
   * requestPasswordReset({userName: 'username'})
   *
   */
  requestPasswordReset(data?: any): Observable<AuthResult> {
    const url = `${this.authEndpointPrefix}${this.config.requestPasswordResetEndpoint ?? 'request-password-reset'}`;
    return this.baseApiRequestWithAuth(this.http, url, {}, this.config.requestPasswordResetMethod ?? 'post', data)
      .pipe(
        map((res) => {
          return new AuthResult(true, res, true, [], ['Reset password instructions have been sent to your email!']);
        }),
        catchError((res) => {
          return this.handleResponseError(res);
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Tries to reset password
   *
   * Example:
   * passwordReset({newPassword: 'test'})
   *
   */
  passwordReset(data?: any): Observable<AuthResult> {
    const url = `${this.authEndpointPrefix}${this.config.passwordResetEndpoint ?? 'password-reset'}`;
    const tokenQueryKey = this.config?.passwordResetConfig?.tokenQueryKey ?? 'reset_password_token';
    const userNameQueryKey = this.config?.passwordResetConfig?.userNameQueryKey ?? 'user_name';
    const emailQueryKey = this.config?.passwordResetConfig?.emailQueryKey ?? 'email';
    const tokenKey = this.config?.passwordResetConfig?.tokenKey ?? 'token';
    const userNameKey = this.config?.passwordResetConfig?.userNameKey ?? 'userName';
    const emailKey = this.config?.passwordResetConfig?.emailKey ?? 'email';
    data[tokenKey] = this.route.snapshot.queryParams[tokenQueryKey];
    if (this.route.snapshot.queryParams[userNameQueryKey]) {
      data[userNameKey] = this.route.snapshot.queryParams[userNameQueryKey];
    }
    if (this.route.snapshot.queryParams[emailQueryKey]) {
      data[emailKey] = this.route.snapshot.queryParams[emailQueryKey];
    }
    return this.baseApiRequestWithAuth(this.http, url, {}, this.config.passwordResetMethod ?? 'post', data)
      .pipe(
        map((res) => {
          return new AuthResult(
            true,
            res,
            true,
            [], // ['Something went wrong, please try again.'],
            ['Your password has been successfully changed!'],
          );
        }),
        catchError((res) => {
          return this.handleResponseError(res);
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Uses an email verification token to confirm you own the email address you used
   *
   * Example:
   * verifyEmail()
   *
   */
  verifyEmail(): Observable<AuthResult> {
    const data = {};
    const url = `${this.authEndpointPrefix}${this.config.verifyEmailEndpoint ?? 'verify-email'}`;
    const tokenQueryKey = this.config?.verifyEmailConfig?.tokenQueryKey ?? 'email_confirm_token';
    const userNameQueryKey = this.config?.verifyEmailConfig?.userNameQueryKey ?? 'user_name';
    const emailQueryKey = this.config?.verifyEmailConfig?.emailQueryKey ?? 'email';
    const tokenKey = this.config?.verifyEmailConfig?.tokenKey ?? 'token';
    const userNameKey = this.config?.verifyEmailConfig?.userNameKey ?? 'userName';
    const emailKey = this.config?.verifyEmailConfig?.emailKey ?? 'email';
    data[tokenKey] = this.route.snapshot.queryParams[tokenQueryKey];
    if (this.route.snapshot.queryParams[userNameQueryKey]) {
      data[userNameKey] = this.route.snapshot.queryParams[userNameQueryKey];
    }
    if (this.route.snapshot.queryParams[emailQueryKey]) {
      data[emailKey] = this.route.snapshot.queryParams[emailQueryKey];
    }
    return this.baseApiRequestWithAuth(this.http, url, {}, this.config.verifyEmailMethod ?? 'post', data)
      .pipe(
        map((res) => {
          return new AuthResult(
            true,
            res,
            true,
            [], // ['Something went wrong, please try again.'],
            ['Your Email has been successfully verified!'],
          );
        }),
        catchError((res) => {
          return this.handleResponseError(res);
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  /**
   * Requests an email for email verification
   *
   * Example:
   * verifyEmail({email: 'user@example.com'})
   *
   */
  requestVerificationEmail(data?: any): Observable<AuthResult> {
    const url = `${this.authEndpointPrefix}${
      this.config.requestVerificationEmailEndpoint ?? 'request-verification-email'
    }`;
    return this.baseApiRequestWithAuth(this.http, url, {}, this.config.requestVerificationEmailMethod ?? 'post', data)
      .pipe(
        map((res) => {
          return new AuthResult(true, res, true, [], ['Your verification Email has been successfully sent!']);
        }),
        catchError((res) => {
          return this.handleResponseError(res);
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  private processResultToken(result: AuthResult): Observable<AuthResult> {
    if (result.isSuccess() && result.getToken()) {
      return this.tokenService
        .set(result.getToken())
        .pipe(
          map((token: AuthToken) => {
            return result;
          }),
        )
        .pipe(takeUntil(this.ngUnsubscribe));
    }

    return of(result).pipe(takeUntil(this.ngUnsubscribe));
  }

  createToken(value: any, failWhenInvalidToken?: boolean): AuthJWTToken {
    const token = AuthCreateJWTToken(value);
    // At this point, AuthCreateToken failed with AuthIllegalTokenError which MUST be intercepted
    // Or token is created. It MAY be created even if backend did not return any token, in this case it is !Valid
    if (failWhenInvalidToken && !token.isValid()) {
      // If we require a valid token (i.e. isValid), then we MUST throw AuthIllegalTokenError so that the we
      // intercept it
      throw new AuthIllegalTokenError('Token is empty or invalid.');
    }
    return token;
  }

  private paramsToQuery(params: any): string {
    return Object.keys(params)
      .map((key) => {
        if (Array.isArray(params[key])) {
          return params[key]
            .map((value: string | number | boolean) => {
              if (value === undefined || value === null) {
                return '';
              }
              return `${encodeURIComponent(key)}=${encodeURIComponent(value)}`;
            })
            .join('&');
        }
        if (params[key] === undefined || params[key] === null) {
          return '';
        }
        return `${encodeURIComponent(key)}=${encodeURIComponent(params[key])}`;
      })
      .filter((s) => s !== '')
      .join('&');
  }

  private baseApiRequest<T>(
    http: HttpClient,
    url: string,
    queryParams: any,
    method: HttpMethod,
    body: any,
  ): Observable<T> {
    const headers = new HttpHeaders();
    headers.append('Content-Type', 'application/json');
    const queryString = this.paramsToQuery(queryParams);
    let newUrl = url;
    if (queryString && queryString.length > 0) {
      newUrl = `${newUrl}?${queryString}`;
    }
    return http
      .request<T>(method, newUrl, { body, headers, withCredentials: true, observe: 'response' })
      .pipe(retry(2))
      .pipe(map((result) => result.body));
  }

  private baseApiRequestWithAuth<T>(
    http: HttpClient,
    url: string,
    queryParams: any,
    method: HttpMethod,
    body: any,
  ): Observable<T> {
    return this.isAuthenticatedOrRefresh().pipe(
      switchMap(() => {
        return this.baseApiRequest<T>(http, url, queryParams, method, body);
      }),
    );
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}

type HttpMethod = 'get' | 'post' | 'put' | 'patch' | 'delete';
