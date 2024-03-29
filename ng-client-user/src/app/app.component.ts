import { DOCUMENT } from '@angular/common';
import { Inject } from '@angular/core';
import { Component, OnInit, OnDestroy } from '@angular/core';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { AuthService } from 'src/auth/services/auth.service';
import { websiteUrl } from 'src/environments/environment';
import { PageViewService } from './shared/services/page-view.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  loggedIn = false;
  displayName: string;
  isMenuCollapsed = true;

  constructor(
    private authService: AuthService,
    private pageViewService: PageViewService,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit() {
    this.authService
      .isAuthenticatedOrRefresh()
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.loggedIn = result;
        if (result === true) {
          this.authService.getUsername().subscribe((name) => {
            this.displayName = name;
          });
          this.authService.getToken().subscribe((token) => console.log);
        }
      });
    this.authService
      .onAuthenticationChange()
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.loggedIn = result;
        if (result === true) {
          this.authService.getUsername().subscribe((name) => {
            this.displayName = name;
          });
        }
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
