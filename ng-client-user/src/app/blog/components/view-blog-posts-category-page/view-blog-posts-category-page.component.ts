import { Component, OnInit } from '@angular/core';
import { BlogPost } from '../../../models/api/blog-post';
import { BlogPostService } from '../../services/blog-post.service';
import { ActivatedRoute } from '@angular/router';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-view-blog-posts-category-page',
  templateUrl: './view-blog-posts-category-page.component.html',
  styleUrls: ['./view-blog-posts-category-page.component.scss'],
})
export class ViewBlogPostsCategoryPageComponent implements OnInit {
  categoryName: string;
  resultNumber = 0;
  blogPostsList: BlogPost[] = [];
  page: number;
  pageSize: number;
  loading = true;

  constructor(private blogPostService: BlogPostService, private route: ActivatedRoute, private title: Title) {}

  ngOnInit(): void {
    this.route.params.subscribe((params) => {
      this.categoryName = params.categoryName;
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
    this.title.setTitle(`axmouth.dev - Loading Blog Posts, Category: ${this.categoryName}`);
    this.loading = true;
    this.blogPostService.getAllPostsByCategory(this.categoryName, this.page, this.pageSize).subscribe((result) => {
      this.blogPostsList = result.data;
      this.resultNumber = result?.pagination?.totalResults;
      this.loading = false;
      this.title.setTitle(`axmouth.dev - Blog Posts Index, Category: ${this.categoryName}`);
    });
  }
}
