import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { ContactRoutingModule } from './contact-routing.module';
import { ContactPageComponent } from './components/contact-page/contact-page.component';
import { ContactSuccessPageComponent } from './components/contact-success-page/contact-success-page.component';
import { SharedModule } from '../shared/shared.module';
import { ContactMainComponent } from './components/contact-main/contact-main.component';
import { ContactService } from './services/contact.service';
import { HCaptchaDirective } from '../shared/directives/h-captcha.directive';
import { ReactiveFormsModule } from '@angular/forms';

@NgModule({
  declarations: [ContactPageComponent, ContactSuccessPageComponent, ContactMainComponent],
  imports: [CommonModule, ContactRoutingModule, SharedModule, ReactiveFormsModule],
  providers: [ContactService],
})
export class ContactModule {}
