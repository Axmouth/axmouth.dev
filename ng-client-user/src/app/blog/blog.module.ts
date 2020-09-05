import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { BlogRoutingModule } from './blog-routing.module';
import { ViewBlogPostsPageComponent } from './components/view-blog-posts-page/view-blog-posts-page.component';
import { ViewBlogPostDetailsPageComponent } from './components/view-blog-post-details-page/view-blog-post-details-page.component';
import { BlogMainComponent } from './components/blog-main/blog-main.component';
import { BlogSearchSideWidgetComponent } from './components/blog-search-side-widget/blog-search-side-widget.component';
import { BlogCategorySideWidgetComponent } from './components/blog-category-side-widget/blog-category-side-widget.component';
import { SharedModule } from '../shared/shared.module';
import { ViewBlogPostsCategoryPageComponent } from './components/view-blog-posts-category-page/view-blog-posts-category-page.component';
import { BlogPostPreviewComponent } from './components/blog-post-preview/blog-post-preview.component';
import { ViewBlogPostSearchPageComponent } from './components/view-blog-post-search-page/view-blog-post-search-page.component';
import { BlogPostCommentsListComponent } from './components/blog-post-comments-list/blog-post-comments-list.component';
import { BlogPostCommentComponent } from './components/blog-post-comment/blog-post-comment.component';
import { BlogPostService } from './services/blog-post.service';
import { BlogPostCommentService } from './services/blog-post-comment.service';
import { CommentsTreeComponent } from './components/comments-tree/comments-tree.component';
import { CommentFormComponent } from './components/comment-form/comment-form.component';
import { CommentPreviewComponent } from './components/comment-preview/comment-preview.component';
import { CommentPostedComponent } from './components/comment-posted/comment-posted.component';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';
import { CommentPostedPageComponent } from './components/comment-posted-page/comment-posted-page.component';
import { CommentPreviewPageComponent } from './components/comment-preview-page/comment-preview-page.component';

@NgModule({
  declarations: [
    ViewBlogPostsPageComponent,
    ViewBlogPostDetailsPageComponent,
    BlogMainComponent,
    BlogSearchSideWidgetComponent,
    BlogCategorySideWidgetComponent,
    ViewBlogPostsCategoryPageComponent,
    BlogPostPreviewComponent,
    ViewBlogPostSearchPageComponent,
    BlogPostCommentsListComponent,
    BlogPostCommentComponent,
    CommentsTreeComponent,
    CommentFormComponent,
    CommentPreviewComponent,
    CommentPostedComponent,
    CommentPostedPageComponent,
    CommentPreviewPageComponent,
  ],
  imports: [CommonModule, BlogRoutingModule, SharedModule, ReactiveFormsModule, FormsModule],
  providers: [BlogPostService, BlogPostCommentService],
})
export class BlogModule {}
