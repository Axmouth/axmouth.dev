import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { ViewBlogPostsPageComponent } from './pages/view-blog-posts-page/view-blog-posts-page.component';
import { ViewBlogPostDetailsPageComponent } from './pages/view-blog-post-details-page/view-blog-post-details-page.component';
import { BlogMainComponent } from './components/blog-main/blog-main.component';
import { NotFoundPageComponent } from '../shared/components/not-found-page/not-found-page.component';
import { ViewBlogPostsCategoryPageComponent } from './pages/view-blog-posts-category-page/view-blog-posts-category-page.component';

const routes: Routes = [
  {
    path: '',
    component: BlogMainComponent,

    children: [
      { path: '', component: ViewBlogPostsPageComponent, pathMatch: 'full' },
      { path: 'category/:categoryName', component: ViewBlogPostsCategoryPageComponent, pathMatch: 'full' },
      { path: ':id', component: ViewBlogPostDetailsPageComponent, pathMatch: 'full' },
      { path: '**', component: NotFoundPageComponent, pathMatch: 'full' },
    ],
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class BlogRoutingModule {}
