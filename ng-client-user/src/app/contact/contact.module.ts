import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { ContactRoutingModule } from './contact-routing.module';
import { ContactPageComponent } from './pages/contact-page/contact-page.component';
import { ContactSuccessPageComponent } from './pages/contact-success-page/contact-success-page.component';
import { SharedModule } from '../shared/shared.module';
import { ContactMainComponent } from './components/contact-main/contact-main.component';
import { ContactService } from './services/contact.service';
import { HCaptchaDirective } from '../shared/directives/h-captcha.directive';
import { ReactiveFormsModule } from '@angular/forms';
import { NgbModule } from '@ng-bootstrap/ng-bootstrap';

@NgModule({
  declarations: [ContactPageComponent, ContactSuccessPageComponent, ContactMainComponent],
  imports: [CommonModule, ContactRoutingModule, SharedModule, ReactiveFormsModule, NgbModule],
  providers: [ContactService],
})
export class ContactModule {}
