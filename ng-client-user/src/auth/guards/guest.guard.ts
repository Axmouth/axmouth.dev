import { Injectable, OnDestroy } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, RouterStateSnapshot, UrlTree, Router } from '@angular/router';
import { Observable, Subject, of } from 'rxjs';
import { AuthService } from '..';
import { map, takeUntil } from 'rxjs/operators';
// import { IsBrowserService } from 'src/app/helpers/services/is-browser.service';

@Injectable({
  providedIn: 'root',
})
export class GuestGuard implements CanActivate, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  constructor(private router: Router, private authService: AuthService) {}
  canActivate(
    next: ActivatedRouteSnapshot,
    state: RouterStateSnapshot,
  ): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {
    return this.authService
      .isAuthenticatedOrRefresh()
      .pipe(
        map((auth) => {
          if (auth) {
            this.router.navigate(['']);
          }
          return !auth;
        }),
      )
      .pipe(takeUntil(this.ngUnsubscribe));
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
