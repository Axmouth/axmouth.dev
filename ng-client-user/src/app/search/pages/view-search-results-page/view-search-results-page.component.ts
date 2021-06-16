import { DOCUMENT } from '@angular/common';
import { Component, Inject, OnInit, OnDestroy } from '@angular/core';
import { Title, Meta } from '@angular/platform-browser';
import { ActivatedRoute, Router } from '@angular/router';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { websiteUrl } from 'src/environments/environment';
import { SearchService } from 'src/app/search/services/search.service';
import { SearchItem } from 'src/app/models/api/search-item';
import { FormGroup, FormControl } from '@angular/forms';

@Component({
  selector: 'app-view-search-results-page',
  templateUrl: './view-search-results-page.component.html',
  styleUrls: ['./view-search-results-page.component.scss'],
})
export class ViewSearchResultsPageComponent implements OnInit, OnDestroy {
  searchForm = new FormGroup({
    searchText: new FormControl(''),
  });
  ngUnsubscribe = new Subject<void>();
  resultNumber = 0;
  searchText?: string;
  searchItemList: SearchItem[] = [];
  page?: number;
  pageSize?: number;
  type?: 'Project' | 'BlogPost' | 'Page' | 'ExternalLink';
  loading = true;

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private searchService: SearchService,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.route.params.pipe(takeUntil(this.ngUnsubscribe)).subscribe((params) => {
      this.searchText = params.searchText;
      this.searchForm.get('searchText').setValue(this.searchText);
      this.route.queryParams.pipe(takeUntil(this.ngUnsubscribe)).subscribe((qParams) => {
        if (isNaN(+qParams.page) === false) {
          this.page = +qParams.page ?? 1;
        } else {
          this.page = 1;
        }
        if (isNaN(+qParams.pageSize) === false) {
          this.pageSize = +qParams.pageSize ?? 5;
        } else {
          this.pageSize = 5;
        }
        this.type = qParams.type ?? undefined;
        this.initialiseState();
      });
    });
  }

  initialiseState() {
    this.title.setTitle(`Loading Search Items | Axmouth's Website`);
    this.loading = true;
    this.searchService
      .getAll(this.searchText !== '' ? this.searchText : undefined, this.type, this.page, this.pageSize)
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.searchItemList = result.data;
        this.resultNumber = result?.pagination?.totalResults;
        this.loading = false;
        this.title.setTitle(`Search: ${this.searchText} | Axmouth's Website`);
        this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
        this.meta.updateTag({
          property: `og:url`,
          content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
        });
        this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
        this.meta.updateTag({
          property: `twitter:url`,
          content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
        });
        this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
      });
  }

  onPageChange(): void {
    this.router.navigate([], {
      relativeTo: this.route,
      queryParams: { page: this.page, pageSize: this.pageSize, type: this.type },
      queryParamsHandling: 'merge',
    });
  }

  onSearchSubmit() {
    this.router.navigate(['/search', this.searchForm.get('searchText').value], {
      queryParams: {},
      queryParamsHandling: 'merge',
    });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
