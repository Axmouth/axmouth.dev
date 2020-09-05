import { Component, OnInit } from '@angular/core';
import { HCAPTCHA_SITE_KEY } from 'src/environments/environment';
import { FormControl, FormGroup } from '@angular/forms';
import { ContactService } from '../../services/contact.service';

@Component({
  selector: 'app-contact-page',
  templateUrl: './contact-page.component.html',
  styleUrls: ['./contact-page.component.scss'],
})
export class ContactPageComponent implements OnInit {
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

  constructor(private contactService: ContactService) {}

  ngOnInit(): void {}

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
      .subscribe((result) => {
        //
      });
  }
}
