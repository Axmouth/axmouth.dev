import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HttpClientModule } from '@angular/common/http';
import { FlexLayoutModule } from '@angular/flex-layout';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatButtonToggleModule } from '@angular/material/button-toggle';
import { MatCardModule } from '@angular/material/card';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatDialogModule } from '@angular/material/dialog';
import { MatDividerModule } from '@angular/material/divider';
import { MatExpansionModule } from '@angular/material/expansion';
import { MatIconModule } from '@angular/material/icon';
import { MatInputModule } from '@angular/material/input';
import { MatListModule } from '@angular/material/list';
import { MatMenuModule } from '@angular/material/menu';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatSliderModule } from '@angular/material/slider';
import { MatToolbarModule } from '@angular/material/toolbar';
import { AdminDashboardRoutingModule } from './admin-dashboard-routing.module';
import { ViewAdminModelComponent } from './components/view-admin-model/view-admin-model.component';
import { ViewAdminCategoryComponent } from './components/view-admin-category/view-admin-category.component';
import { ViewEntityComponent } from './components/view-entity/view-entity.component';
import { AdminModelComponent } from './components/admin-model/admin-model.component';
import { AdminCategoryComponent } from './components/admin-category/admin-category.component';
import { CreateEntityComponent } from './components/create-entity/create-entity.component';
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
import { BooleanFieldComponent } from './field-components/boolean-field/boolean-field.component';
import { EnumFieldComponent } from './field-components/enum-field/enum-field.component';
import { MatPaginatorModule } from '@angular/material/paginator';
import { MatSelectModule } from '@angular/material/select';

@NgModule({
  declarations: [
    AdminModelComponent,
    AdminCategoryComponent,
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
    BooleanFieldComponent,
    EnumFieldComponent,
  ],
  imports: [
    AdminDashboardRoutingModule,
    CommonModule,
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
  ],
  exports: [
    AdminModelComponent,
    AdminCategoryComponent,
    AdminDashboardRoutingModule,
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
    BooleanFieldComponent,
    EnumFieldComponent,
  ],
})
export class AdminDashboardModule {}
