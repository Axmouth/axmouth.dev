import { Component, OnInit } from '@angular/core';
import { BlogPostCategoryService } from '../../services/blog-post-category.service';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-blog-category-side-widget',
  templateUrl: './blog-category-side-widget.component.html',
  styleUrls: ['./blog-category-side-widget.component.scss'],
})
export class BlogCategorySideWidgetComponent implements OnInit {
  blogCategoriesList: any[] = [];
  resultNumber = 0;
  page: number;
  pageSize: number;

  constructor(private categoryService: BlogPostCategoryService, private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.subscribe((params) => {
      if (isNaN(+params.page) === false) {
        this.page = +params.categoryPage;
      }
      if (isNaN(+params.pageSize) === false) {
        this.pageSize = +params.categoryPageSize;
      }
      this.initialiseState(); // reset and set based on new parameter this time
    });
  }

  initialiseState() {
    this.categoryService.getAllCategories(this.page, this.pageSize).subscribe((result) => {
      this.blogCategoriesList = result.data;
      this.resultNumber = result?.pagination?.totalResults;
    });
  }
}
