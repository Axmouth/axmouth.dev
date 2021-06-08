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
import { AdminModelComponent } from './admin-dashboard/components/admin-model/admin-model.component';
import { AdminCategoryComponent } from './admin-dashboard/components/admin-category/admin-category.component';
import { NavbarComponent } from './components/navbar/navbar.component';
import { LoginPageComponent } from './components/login-page/login-page.component';
import { LogoutPageComponent } from './components/logout-page/logout-page.component';
import { NotFoundPageComponent } from './admin-dashboard/components/not-found-page/not-found-page.component';
import { HomeComponent } from './components/home/home.component';
import { ViewAdminModelComponent } from './admin-dashboard/components/view-admin-model/view-admin-model.component';
import { ViewAdminCategoryComponent } from './admin-dashboard/components/view-admin-category/view-admin-category.component';
import { ViewEntityComponent } from './admin-dashboard/components/view-entity/view-entity.component';
import { CreateEntityComponent } from './admin-dashboard/components/create-entity/create-entity.component';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http';
import { TitleFieldComponent } from './field-components/title-field/title-field.component';
import { TextFieldComponent } from './field-components/text-field/text-field.component';
import { IntegerFieldComponent } from './field-components/integer-field/integer-field.component';
import { FloatFieldComponent } from './field-components/float-field/float-field.component';
import { UnsignedFloatFieldComponent } from './field-components/unsigned-float-field/unsigned-float-field.component';
import { UnsignedIntegerFieldComponent } from './field-components/unsigned-integer-field/unsigned-integer-field.component';
import { HtmlFieldComponent } from './field-components/html-field/html-field.component';
import { MarkdownFieldComponent } from './field-components/markdown-field/markdown-field.component';
import { NameListFieldComponent } from './field-components/name-list-field/name-list-field.component';
import { ImageFieldComponent } from './field-components/image-field/image-field.component';
import { LinkFieldComponent } from './field-components/link-field/link-field.component';
import { FieldChooserComponent } from './field-components/field-chooser/field-chooser.component';
import { EditorjsFieldComponent } from './field-components/editorjs-field/editorjs-field.component';
import { DateFieldComponent } from './field-components/date-field/date-field.component';
import { AddToListDialogComponent } from './field-components/add-to-list-dialog/add-to-list-dialog.component';
import { EditEntityComponent } from './admin-dashboard/components/edit-entity/edit-entity.component';
import { BooleanFieldComponent } from './field-components/boolean-field/boolean-field.component';
import { EnumFieldComponent } from './field-components/enum-field/enum-field.component';
import { jwtWhitelist } from 'src/environments/environment';
import { AdminDashboardModule } from './admin-dashboard/admin-dashboard.module';
import { AdminLogPreviewComponent } from './components/admin-log-preview/admin-log-preview.component';
import { AdminLogsComponent } from './components/pages/admin-logs/admin-logs.component';
import { AdminLogDetailsComponent } from './components/pages/admin-log-details/admin-log-details.component';

@NgModule({
  declarations: [
    AppComponent,
    AdminModelComponent,
    AdminCategoryComponent,
    NavbarComponent,
    LoginPageComponent,
    LogoutPageComponent,
    NotFoundPageComponent,
    HomeComponent,
    ViewAdminModelComponent,
    ViewAdminCategoryComponent,
    ViewEntityComponent,
    CreateEntityComponent,
    TitleFieldComponent,
    TextFieldComponent,
    IntegerFieldComponent,
    FloatFieldComponent,
    UnsignedFloatFieldComponent,
    UnsignedIntegerFieldComponent,
    HtmlFieldComponent,
    MarkdownFieldComponent,
    NameListFieldComponent,
    ImageFieldComponent,
    LinkFieldComponent,
    FieldChooserComponent,
    EditorjsFieldComponent,
    DateFieldComponent,
    AddToListDialogComponent,
    EditEntityComponent,
    BooleanFieldComponent,
    EnumFieldComponent,
    AdminLogPreviewComponent,
    AdminLogsComponent,
    AdminLogDetailsComponent,
  ],
  imports: [
    BrowserModule,
    BrowserAnimationsModule,
    RoutingModule,
    FormsModule,
    ReactiveFormsModule,
    HttpClientModule,
    FlexLayoutModule,
    MatDialogModule,
    MatToolbarModule,
    MatCheckboxModule,
    MatInputModule,
    MatSliderModule,
    MatMenuModule,
    MatIconModule,
    MatButtonModule,
    MatButtonToggleModule,
    MatProgressBarModule,
    MatDividerModule,
    MatCardModule,
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
