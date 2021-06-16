import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { SearchResultPreviewComponent } from 'src/app/search/components/search-result-preview/search-result-preview.component';
import { ViewSearchResultsPageComponent } from 'src/app/search/pages/view-search-results-page/view-search-results-page.component';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';
import { NgbModule } from '@ng-bootstrap/ng-bootstrap';
import { SharedModule } from 'src/app/shared/shared.module';
import { SearchRoutingModule } from 'src/app/search/search-routing.module';

@NgModule({
  declarations: [SearchResultPreviewComponent, ViewSearchResultsPageComponent],
  imports: [CommonModule, SearchRoutingModule, SharedModule, ReactiveFormsModule, FormsModule, NgbModule],
})
export class SearchModule {}
