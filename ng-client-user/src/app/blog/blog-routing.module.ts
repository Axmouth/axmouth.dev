import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { ViewBlogPostsPageComponent } from './components/view-blog-posts-page/view-blog-posts-page.component';
import { ViewBlogPostDetailsPageComponent } from './components/view-blog-post-details-page/view-blog-post-details-page.component';
import { BlogMainComponent } from './components/blog-main/blog-main.component';
import { NotFoundPageComponent } from '../shared/components/not-found-page/not-found-page.component';
import { ViewBlogPostsCategoryPageComponent } from './components/view-blog-posts-category-page/view-blog-posts-category-page.component';
import { ViewBlogPostSearchPageComponent } from './components/view-blog-post-search-page/view-blog-post-search-page.component';

const routes: Routes = [
  {
    path: '',
    component: BlogMainComponent,

    children: [
      { path: '', component: ViewBlogPostsPageComponent, pathMatch: 'full' },
      { path: 'category/:categoryName', component: ViewBlogPostsCategoryPageComponent, pathMatch: 'full' },
      { path: 'search/:searchQuery', component: ViewBlogPostSearchPageComponent, pathMatch: 'full' },
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
