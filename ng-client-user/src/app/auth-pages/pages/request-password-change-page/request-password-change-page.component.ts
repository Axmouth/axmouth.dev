import { DOCUMENT } from '@angular/common';
import { Inject, OnDestroy } from '@angular/core';
import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl, Validators } from '@angular/forms';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { AuthService } from 'src/auth';
import { IsBrowserService } from 'src/auth/helpers/services/is-browser.service';
import { AuthResult } from 'src/auth/internal/auth-result';
import { websiteUrl } from 'src/environments/environment';

@Component({
  selector: 'app-request-password-change-page',
  templateUrl: './request-password-change-page.component.html',
  styleUrls: ['./request-password-change-page.component.scss'],
})
export class RequestPasswordChangePageComponent implements OnInit, OnDestroy {
  successMessages: string[];
  errors: string[] = [];
  result: AuthResult;
  loading = false;
  success: boolean;
  ngUnsubscribe = new Subject<void>();
  requestPasswordResetForm = new FormGroup({
    email: new FormControl('', [Validators.required, Validators.email]),
  });
  constructor(
    private authService: AuthService,
    private isBrowserService: IsBrowserService,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.title.setTitle(`Request Password Reset | Axmouth's Website`);
    this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
    this.meta.updateTag({
      property: `og:url`,
      content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
    });
    this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
    this.meta.updateTag({
      property: `twitter:url`,
      content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
    });
    this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
    if (!this.isBrowserService.isInBrowser()) {
      return;
    }
  }

  onRequestPasswordResetSubmit(): void {
    this.authService
      .requestPasswordReset({ email: this.requestPasswordResetForm.get('email').value })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe(
        (result) => {
          this.result = result;
          if (result.isSuccess()) {
            this.success = true;
            this.errors = [];
            this.successMessages = result.getMessages();
          } else {
            this.success = false;
            console.log(result.getResponse());
            console.log(result.getErrors());
            this.errors = result.getResponse().error.errors;
          }
          this.loading = false;
        },
        (err) => {
          console.log(err);
          this.loading = false;
        },
      );
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
