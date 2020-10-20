import { Injectable, Inject, OnDestroy, PLATFORM_ID } from '@angular/core';
import { AX_AUTH_OPTIONS } from '../auth-injection-token';
import { TokenService } from '../services/token.service';
import { HttpRequest, HttpHandler, HttpEvent } from '@angular/common/http';
import { AuthJWTToken } from '../internal/auth-jwt-token';
import { Subject, Observable } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { parse } from 'url';
import { isPlatformBrowser } from '@angular/common';
import { AuthModuleOptionsConfig } from '../auth-module-options-config';

@Injectable()
export class JwtInterceptor implements OnDestroy {
  ngUnsubscribe = new Subject<void>();
  headerName: string;
  authScheme: string;
  whitelistedDomains: Array<string | RegExp>;
  blacklistedRoutes: Array<string | RegExp>;
  throwNoTokenError: boolean;
  skipWhenExpired: boolean;
  token: AuthJWTToken;

  /**
   *
   */
  constructor(
    private tokenService: TokenService,
    @Inject(AX_AUTH_OPTIONS) config: AuthModuleOptionsConfig,
    @Inject(PLATFORM_ID) private platform: object,
  ) {
    tokenService
      .tokenChange()
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((token) => {
        if (token && token.getValue()) {
          this.token = new AuthJWTToken(token.getValue(), token.getCreatedAt());
        } else {
          this.token = null;
        }
      });
    this.headerName = config.headerName ?? 'Authorization';
    this.authScheme = config.authScheme ?? config.authScheme === '' ? config.authScheme : 'Bearer ';
    this.whitelistedDomains = config.whitelistedDomains || [];
    this.blacklistedRoutes = config.blacklistedRoutes || [];
    this.throwNoTokenError = config.throwNoTokenError || false;
    this.skipWhenExpired = config.skipWhenExpired;
  }

  isWhitelistedDomain(request: HttpRequest<any>): boolean {
    const requestUrl: any = parse(request.url, false, true);

    return (
      requestUrl.hostname === null ||
      this.whitelistedDomains.findIndex((domain) =>
        typeof domain === 'string'
          ? domain === requestUrl.hostname
          : domain instanceof RegExp
          ? domain.test(requestUrl.hostname)
          : false,
      ) > -1
    );
  }

  isBlacklistedRoute(request: HttpRequest<any>): boolean {
    const requestedUrl = parse(request.url, false, true);

    return (
      this.blacklistedRoutes.findIndex((route: string | RegExp) => {
        if (typeof route === 'string') {
          const parsedRoute = parse(route, false, true);
          return parsedRoute.hostname === requestedUrl.hostname && parsedRoute.path === requestedUrl.path;
        }

        if (route instanceof RegExp) {
          return route.test(request.url);
        }

        return false;
      }) > -1
    );
  }

  handleInterception(
    token: AuthJWTToken | null,
    request: HttpRequest<any>,
    next: HttpHandler,
  ): Observable<HttpEvent<any>> {
    let tokenIsExpired = false;

    if (!isPlatformBrowser(this.platform)) {
      tokenIsExpired = true;
    } else if (token) {
      tokenIsExpired = token.getTokenExpDate() < new Date();
    }

    if (token && tokenIsExpired && this.skipWhenExpired) {
      request = request.clone();
    } else if (token) {
      request = request.clone({
        setHeaders: {
          [this.headerName]: `${this.authScheme}${token.getValue()}`,
        },
      });
    }
    return next.handle(request);
  }

  intercept(request: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    if (!this.isWhitelistedDomain(request) || this.isBlacklistedRoute(request)) {
      return next.handle(request);
    }

    return this.handleInterception(this.token, request, next);
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
