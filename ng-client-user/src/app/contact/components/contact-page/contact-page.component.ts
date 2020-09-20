import { Component, OnInit, OnDestroy } from '@angular/core';
import { HCAPTCHA_SITE_KEY } from 'src/environments/environment';
import { FormControl, FormGroup } from '@angular/forms';
import { ContactService } from '../../services/contact.service';
import { Router } from '@angular/router';
import { catchError, takeUntil } from 'rxjs/operators';
import { Response } from '../../../models/api/response';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';

@Component({
  selector: 'app-contact-page',
  templateUrl: './contact-page.component.html',
  styleUrls: ['./contact-page.component.scss'],
})
export class ContactPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  contactForm = new FormGroup({
    hcaptcha: new FormControl(''),
    message: new FormControl(''),
    subject: new FormControl(''),
    from_email: new FormControl(''),
  });
  HCAPTCHA_SITE_KEY = HCAPTCHA_SITE_KEY;
  captchaConfig = {
    size: 'compact',
  };
  captchaDone = false;

  captchaToken: string;
  errors: string[];

  constructor(
    private contactService: ContactService,
    private router: Router,
    private title: Title,
    private meta: Meta,
  ) {}

  ngOnInit(): void {
    this.title.setTitle(`Contact Me - Axmouth's Website`);
    this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `og:url`, content: window.location.href });
    this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
    this.meta.updateTag({ property: `twitter:url`, content: window.location.href });
    this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
  }

  onCaptchaResponse(token: string) {
    this.captchaDone = true;
    this.captchaToken = token;
    console.log('CaptchaResponse : ' + token);
  }

  onCaptchaExpired() {
    this.captchaDone = false;
    console.log('Captcha Expired');
  }

  onSubmit() {
    this.contactService
      .sendContactEmail(
        this.contactForm.get('subject').value,
        this.contactForm.get('from_email').value,
        this.contactForm.get('message').value,
        this.contactForm.get('hcaptcha').value,
      )
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe(
        (result) => {
          if (result.success === true) {
            this.router.navigateByUrl('/contact/success');
          } else {
            this.errors = result.errors;
          }
        },
        (err) => {
          console.log(err);
        },
      );
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
