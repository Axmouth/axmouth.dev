import { DOCUMENT } from '@angular/common';
import { Inject, OnDestroy } from '@angular/core';
import { Component, OnInit } from '@angular/core';
import { FormGroup, FormControl, Validators } from '@angular/forms';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { CustomValidators } from 'src/app/shared/helpers/custom-validators';
import { AuthService } from 'src/auth';
import { IsBrowserService } from 'src/auth/helpers/services/is-browser.service';
import { AuthResult } from 'src/auth/internal/auth-result';
import { MismatchValidator } from '../register-page/register-page.component';
import { websiteUrl } from 'src/environments/environment';

@Component({
  selector: 'app-password-change-page',
  templateUrl: './password-change-page.component.html',
  styleUrls: ['./password-change-page.component.scss'],
})
export class PasswordChangePageComponent implements OnInit, OnDestroy {
  successMessages: string[];
  errors: string[] = [];
  result: AuthResult;
  loading = false;
  success: boolean;
  ngUnsubscribe = new Subject<void>();
  passwordResetForm = new FormGroup({
    newPassword: new FormControl(''),
    newPassword2: new FormControl(''),
  });

  constructor(
    private authService: AuthService,
    private isBrowserService: IsBrowserService,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.title.setTitle(`Change Password | Axmouth's Website`);
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
    this.setValidators();
    if (!this.isBrowserService.isInBrowser()) {
      return;
    }
  }

  onPasswordResetSubmit(): void {
    this.loading = true;
    this.authService
      .passwordReset({
        newPassword: this.passwordResetForm.get('newPassword').value,
        newPassword2: this.passwordResetForm.get('newPassword2').value,
      })
      .subscribe(
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

  private setValidators(): void {
    const formValidators = {
      password: Validators.compose([
        Validators.required,
        Validators.minLength(6),
        Validators.maxLength(35),
        // check whether the entered password has a number
        CustomValidators.patternValidator(/\d/, { hasNumber: true }),
        // check whether the entered password has upper case letter
        CustomValidators.patternValidator(/[A-Z]/, { hasUpperCase: true }),
        // check whether the entered password has a lower-case letter
        CustomValidators.patternValidator(/[a-z]/, { hasLowerCase: true }),
        // check whether the entered password has a special character
        CustomValidators.patternValidator(/[ !@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]/, { hasSpecialCharacters: true }),
      ]),
      password2: Validators.compose([
        Validators.required,
        MismatchValidator.mismatch(this.passwordResetForm.get('newPassword')),
      ]),
    };

    this.passwordResetForm.get('newPassword').setValidators(formValidators.password);
    this.passwordResetForm.get('newPassword2').setValidators(formValidators.password2);
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
