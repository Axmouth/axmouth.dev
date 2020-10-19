import { DOCUMENT } from '@angular/common';
import { Component, Inject, OnInit, OnDestroy } from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { pipe, Subject } from 'rxjs';
import { map, switchMap, takeUntil } from 'rxjs/operators';
import { Profile } from 'src/app/models/api/profile';
import { Response } from 'src/app/models/api/response';
import { AuthService } from 'src/auth';
import { IsBrowserService } from 'src/auth/helpers/services/is-browser.service';

@Component({
  selector: 'app-settings-page',
  templateUrl: './settings-page.component.html',
  styleUrls: ['./settings-page.component.scss'],
})
export class SettingsPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  sendAccountVerificationEmailClickInProgress = false;
  requestPasswordResetInProgress = false;
  profileData: Profile;

  constructor(
    private authService: AuthService,
    private isBrowserService: IsBrowserService,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.title.setTitle(`Settings | Axmouth's Website`);
    this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `og:url`, content: this.doc.location.href });
    this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `twitter:url`, content: this.doc.location.href });
    this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
    if (!this.isBrowserService.isInBrowser()) {
      return;
    }
    this.initialise();
  }

  initialise(): void {
    this.authService.getProfile<Response<Profile>>().subscribe((profile) => {
      this.profileData = profile.data;
    });
  }

  onSendAccountVerificationEmailClick(): void {
    this.sendAccountVerificationEmailClickInProgress = true;
    this.authService
      .requestVerificationEmail({ email: this.profileData.email })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((_) => {
        this.sendAccountVerificationEmailClickInProgress = false;
      });
  }

  onResetYourPasswordClick(): void {
    this.requestPasswordResetInProgress = true;
    this.authService
      .requestPasswordReset({ email: this.profileData.email })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((_) => {
        this.requestPasswordResetInProgress = false;
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
