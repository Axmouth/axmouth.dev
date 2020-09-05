import { Component, OnInit } from '@angular/core';
import { BlogPost } from '../../../models/api/blog-post';

@Component({
  selector: 'app-view-blog-post-search-page',
  templateUrl: './view-blog-post-search-page.component.html',
  styleUrls: ['./view-blog-post-search-page.component.scss'],
})
export class ViewBlogPostSearchPageComponent implements OnInit {
  resultNumber = 0;
  searchQuery: string;
  blogPostsList: BlogPost[] = [];

  constructor() {}

  ngOnInit(): void {}
}
