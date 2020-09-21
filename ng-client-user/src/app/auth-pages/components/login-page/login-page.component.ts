import { Component, OnInit, OnDestroy } from '@angular/core';
import { FormGroup, FormControl } from '@angular/forms';
import { AuthService } from 'src/auth/services/auth.service';
import { RouteStateService } from 'src/app/shared/services/route-state.service';
import { Router } from '@angular/router';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { DOCUMENT } from '@angular/common';
import { Inject } from '@angular/core';

@Component({
  selector: 'app-login-page',
  templateUrl: './login-page.component.html',
  styleUrls: ['./login-page.component.scss'],
})
export class LoginPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  loginForm = new FormGroup({
    email: new FormControl(''),
    password: new FormControl(''),
  });
  errors = [];

  loginInProgress = false;

  constructor(
    private authService: AuthService,
    private router: Router,
    private routeStateService: RouteStateService,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.title.setTitle(`Login - Axmouth's Website`);
    this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `og:url`, content: this.doc.location.href });
    this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `twitter:url`, content: this.doc.location.href });
    this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
  }

  onSubmit() {
    this.loginInProgress = true;
    this.authService
      .authenticate({
        email: this.loginForm.get('email').value,
        password: this.loginForm.get('password').value,
      })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe(
        async (result) => {
          if (result.isSuccess()) {
            await this.router.navigateByUrl(this.routeStateService.getPreviousUrl());
          } else {
            this.errors = result.getResponse().error.errors;
          }
          this.loginInProgress = false;
        },
        (err) => {
          this.loginInProgress = false;
          console.log(err);
        },
      );
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
