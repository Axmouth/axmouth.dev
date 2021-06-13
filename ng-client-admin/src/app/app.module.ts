import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { MatMenuModule } from '@angular/material/menu';
import { MatInputModule } from '@angular/material/input';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatExpansionModule } from '@angular/material/expansion';
import { MatListModule } from '@angular/material/list';
import { MatDividerModule } from '@angular/material/divider';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatSliderModule } from '@angular/material/slider';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonToggleModule } from '@angular/material/button-toggle';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatCardModule } from '@angular/material/card';
import { MatDialogModule } from '@angular/material/dialog';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatPaginatorModule } from '@angular/material/paginator';
import { MatSelectModule } from '@angular/material/select';
import { FlexLayoutModule } from '@angular/flex-layout';

import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { AuthModule } from '../auth/auth.module';
import { RoutingModule } from './app-routing.module';
import { apiRoot } from 'src/environments/environment';
import { NavbarComponent } from './components/navbar/navbar.component';
import { LoginPageComponent } from './components/login-page/login-page.component';
import { LogoutPageComponent } from './components/logout-page/logout-page.component';
import { NotFoundPageComponent } from './admin-dashboard/components/not-found-page/not-found-page.component';
import { HomeComponent } from './components/home/home.component';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http';
import { jwtWhitelist } from 'src/environments/environment';
import { AdminDashboardModule } from './admin-dashboard/admin-dashboard.module';
import { AdminLogPreviewComponent } from './components/admin-log-preview/admin-log-preview.component';
import { AdminLogsComponent } from './components/pages/admin-logs/admin-logs.component';
import {
  AdminLogDetailsComponent,
  ExampleDialogComponent,
} from './components/pages/admin-log-details/admin-log-details.component';
import { WarningDialogComponent } from './components/warning-dialog/warning-dialog.component';

@NgModule({
  declarations: [
    AppComponent,
    NavbarComponent,
    LoginPageComponent,
    LogoutPageComponent,
    NotFoundPageComponent,
    HomeComponent,
    AdminLogPreviewComponent,
    AdminLogsComponent,
    AdminLogDetailsComponent,
    WarningDialogComponent,
    ExampleDialogComponent,
  ],
  imports: [
    AdminDashboardModule,
    BrowserModule,
    BrowserAnimationsModule,
    RoutingModule,
    FormsModule,
    ReactiveFormsModule,
    HttpClientModule,
    FlexLayoutModule,
    MatCardModule,
    MatButtonModule,
    MatButtonToggleModule,
    MatDialogModule,
    MatToolbarModule,
    MatCheckboxModule,
    MatInputModule,
    MatSliderModule,
    MatMenuModule,
    MatIconModule,
    MatProgressBarModule,
    MatDividerModule,
    MatListModule,
    MatProgressSpinnerModule,
    MatExpansionModule,
    MatPaginatorModule,
    MatSelectModule,
    AuthModule.forRoot({
      config: {
        authEndpointPrefix: `${apiRoot}/auth/`,
        whitelistedDomains: jwtWhitelist,
        blacklistedRoutes: [],
        jwtTokenKey: 'auth_app_token_admin',
        loginEndpoint: 'admin-login',
        logoutEndpoint: 'admin-logout',
        // authScheme: ""
      },
    }),
  ],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
