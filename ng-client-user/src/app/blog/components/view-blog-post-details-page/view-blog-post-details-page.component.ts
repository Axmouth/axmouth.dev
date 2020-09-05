import { Component, OnInit } from '@angular/core';
import { BlogPostService } from '../../services/blog-post.service';
import { ActivatedRoute } from '@angular/router';
import { BlogPost } from '../../../models/api/blog-post';
import { BlogPostCommentService } from '../../services/blog-post-comment.service';
import { BlogPostComment } from 'src/app/models/api/blog-post-comment';

@Component({
  selector: 'app-view-blog-post-details-page',
  templateUrl: './view-blog-post-details-page.component.html',
  styleUrls: ['./view-blog-post-details-page.component.scss'],
})
export class ViewBlogPostDetailsPageComponent implements OnInit {
  post: BlogPost;
  commentsCount = 0;
  postId: string;
  postBodyData: any[];
  blogPostComments: BlogPostComment[] = [];
  commentPage: number;
  commentPageSize: number;

  constructor(
    private blogPostService: BlogPostService,
    private commentService: BlogPostCommentService,
    private route: ActivatedRoute,
  ) {}

  ngOnInit(): void {
    this.route.params.subscribe((params) => {
      this.postId = params.id;
      if (isNaN(+params.page) === false) {
        this.commentPage = +params.commentPage;
      }
      if (isNaN(+params.pageSize) === false) {
        this.commentPageSize = +params.commentPageSize;
      }
      this.initialiseState(); // reset and set based on new parameter this time
    });
  }

  initialiseState() {
    this.blogPostService.getPost(this.postId).subscribe((result) => {
      this.post = result.data;
      this.postBodyData = JSON.parse(result.data.body);
    });
    this.commentService
      .getAllCommentsByPost(this.postId, this.commentPage, this.commentPageSize)
      .subscribe((result) => {
        this.blogPostComments = result.data;
        this.commentsCount = result?.pagination?.totalResults;
      });
  }

  onCommentPosted() {
    this.initialiseState();
  }
}
