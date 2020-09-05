import { Component, OnInit, Output, EventEmitter, Input } from '@angular/core';
import { BlogPostComment } from 'src/app/models/api/blog-post-comment';
import { FormGroup, FormControl } from '@angular/forms';
import { BlogPostCommentService } from 'src/app/blog/services/blog-post-comment.service';
import { ActivatedRoute } from '@angular/router';
import { AuthService } from 'src/auth/services/auth.service';

@Component({
  selector: 'app-comment-form',
  templateUrl: './comment-form.component.html',
  styleUrls: ['./comment-form.component.scss'],
})
export class CommentFormComponent implements OnInit {
  @Output()
  commentPosted: EventEmitter<BlogPostComment> = new EventEmitter<any>();
  commentForm = new FormGroup({
    body: new FormControl(''),
  });
  postId: number;
  loggedIn = false;

  constructor(
    private commentService: BlogPostCommentService,
    private route: ActivatedRoute,
    private authService: AuthService,
  ) {}

  ngOnInit(): void {
    if (!isNaN(+this.route.snapshot.paramMap.get('id'))) {
      this.postId = +this.route.snapshot.paramMap.get('id');
    }
    this.authService.isAuthenticatedOrRefresh().subscribe((result) => {
      this.loggedIn = result;
      if (result === false) {
        this.commentForm.disable();
      }
    });
  }

  OnSubmit() {
    this.commentService.createComment(this.commentForm.get('body').value, this.postId).subscribe((result) => {
      this.commentPosted.emit(null);
      this.commentForm.reset();
    });
  }

  OnPreview() {}
}
