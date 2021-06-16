import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { NotFoundPageComponent } from 'src/app/shared/components/not-found-page/not-found-page.component';
import { ViewSearchResultsPageComponent } from 'src/app/search/pages/view-search-results-page/view-search-results-page.component';

const routes: Routes = [
  { path: ':searchText', component: ViewSearchResultsPageComponent, pathMatch: 'full' },
  { path: '', component: ViewSearchResultsPageComponent, pathMatch: 'full' },
  { path: '**', component: NotFoundPageComponent, pathMatch: 'full' },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class SearchRoutingModule {}
