import { Component, OnInit, Input } from '@angular/core';
import { BlogPost } from '../../../models/api/blog-post';

@Component({
  selector: 'app-blog-post-preview',
  templateUrl: './blog-post-preview.component.html',
  styleUrls: ['./blog-post-preview.component.scss'],
})
export class BlogPostPreviewComponent implements OnInit {
  @Input()
  post: BlogPost;

  constructor() {}

  ngOnInit(): void {}
}
