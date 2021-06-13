import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { CreateEntityComponent } from './components/create-entity/create-entity.component';
import { NotFoundPageComponent } from './components/not-found-page/not-found-page.component';
import { ViewAdminCategoryComponent } from './components/view-admin-category/view-admin-category.component';
import { ViewAdminModelComponent } from './components/view-admin-model/view-admin-model.component';
import { ViewEntityComponent } from './components/view-entity/view-entity.component';

const routes: Routes = [
  { path: 'categories/:categoryName', component: ViewAdminCategoryComponent },
  { path: 'categories/:categoryName/models/:modelName', component: ViewAdminModelComponent },
  {
    path: 'categories/:categoryName/models/:modelName/add',
    component: CreateEntityComponent,
  },
  { path: 'categories/:categoryName/models/:modelName/:id', component: ViewEntityComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class AdminDashboardRoutingModule {}
