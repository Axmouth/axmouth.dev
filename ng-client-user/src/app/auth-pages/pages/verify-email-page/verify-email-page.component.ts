import { DOCUMENT } from '@angular/common';
import { Component, Inject, OnInit, OnDestroy } from '@angular/core';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { AuthService } from 'src/auth';
import { IsBrowserService } from 'src/auth/helpers/services/is-browser.service';
import { AuthResult } from 'src/auth/internal/auth-result';
import { websiteUrl } from 'src/environments/environment';

@Component({
  selector: 'app-verify-email-page',
  templateUrl: './verify-email-page.component.html',
  styleUrls: ['./verify-email-page.component.scss'],
})
export class VerifyEmailPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  result: AuthResult;
  errors: string[] = [];
  loading = true;
  success: boolean;
  successMessages: string[];

  constructor(
    private authService: AuthService,
    private isBrowserService: IsBrowserService,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.title.setTitle(`Verify Email | Axmouth's Website`);
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
    this.authService.verifyEmail().subscribe(
      (result) => {
        this.result = result;
        if (result.isSuccess()) {
          this.success = true;
          this.errors = [];
          this.successMessages = result.getMessages();
        } else {
          this.success = false;
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
