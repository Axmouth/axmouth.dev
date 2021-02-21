import { Component, OnInit, OnDestroy } from '@angular/core';
import { BlogPostCategoryService } from '../../services/blog-post-category.service';
import { ActivatedRoute } from '@angular/router';
import { takeUntil } from 'rxjs/operators';
import { Subject } from 'rxjs';
import { BlogPostCategory } from 'src/app/models/api/blog-post-category';

@Component({
  selector: 'app-blog-category-side-widget',
  templateUrl: './blog-category-side-widget.component.html',
  styleUrls: ['./blog-category-side-widget.component.scss'],
})
export class BlogCategorySideWidgetComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  blogCategoriesList: BlogPostCategory[];
  resultNumber = 0;
  page: number;
  pageSize: number;

  constructor(private categoryService: BlogPostCategoryService, private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.pipe(takeUntil(this.ngUnsubscribe)).subscribe((params) => {
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
    this.categoryService
      .getAllCategories(this.page, this.pageSize)
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.blogCategoriesList = result.data;
        this.resultNumber = result?.pagination?.totalResults;
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
