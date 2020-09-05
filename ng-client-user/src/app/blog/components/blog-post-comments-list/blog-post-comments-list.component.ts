import { Component, OnInit, Input } from '@angular/core';
import { BlogPostComment } from 'src/app/models/api/blog-post-comment';
import { BlogPostCommentService } from '../../services/blog-post-comment.service';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-blog-post-comments-list',
  templateUrl: './blog-post-comments-list.component.html',
  styleUrls: ['./blog-post-comments-list.component.scss'],
})
export class BlogPostCommentsListComponent implements OnInit {
  @Input()
  blogPostComments: BlogPostComment[] = [];
  resultNumber = 0;
  page: number;
  pageSize: number;
  postId: string;

  constructor(private commentService: BlogPostCommentService, private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.subscribe((params) => {
      this.initialiseState(); // reset and set based on new parameter this time
    });
  }

  initialiseState() {}
}
