import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { HomePageRoutingModule } from './home-page-routing.module';
import { HomePageComponent } from './components/home-page/home-page.component';
import { HomeMainComponent } from './components/home-main/home-main.component';
import { SharedModule } from '../shared/shared.module';
import { IntroductionSectionComponent } from './components/introduction-section/introduction-section.component';
import { MyPageComponent } from './components/my-page/my-page.component';

@NgModule({
  declarations: [HomePageComponent, HomeMainComponent, IntroductionSectionComponent, MyPageComponent],
  imports: [CommonModule, HomePageRoutingModule, SharedModule],
})
export class HomePageModule {}
