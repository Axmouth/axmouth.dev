import { Component, OnInit } from '@angular/core';
import { BlogPostService } from '../../services/blog-post.service';
import { BlogPost } from 'src/app/models/api/blog-post';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-view-blog-posts-page',
  templateUrl: './view-blog-posts-page.component.html',
  styleUrls: ['./view-blog-posts-page.component.scss'],
})
export class ViewBlogPostsPageComponent implements OnInit {
  resultNumber = 0;
  blogPostsList: BlogPost[] = [];
  page: number;
  pageSize: number;

  constructor(private blogPostService: BlogPostService, private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.subscribe((params) => {
      if (isNaN(+params.page) === false) {
        this.page = +params.page;
      }
      if (isNaN(+params.pageSize) === false) {
        this.pageSize = +params.pageSize;
      }
      this.initialiseState(); // reset and set based on new parameter this time
    });
  }

  initialiseState() {
    this.blogPostService.getAllPosts(this.page, this.pageSize).subscribe((result) => {
      this.blogPostsList = result.data;
      this.resultNumber = result?.pagination?.totalResults;
    });
  }
}
