import { Component, OnInit } from '@angular/core';
import { HomePageLink } from 'src/app/models/api/home-page-link';
import { LinkService } from '../../services/link.service';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-links-side-widget',
  templateUrl: './links-side-widget.component.html',
  styleUrls: ['./links-side-widget.component.scss'],
})
export class LinksSideWidgetComponent implements OnInit {
  linksList: HomePageLink[] = LinkService.getAllLinksFromCache()?.data;
  resultNumber = 0;
  page: number;
  pageSize: number;

  constructor(private linkService: LinkService, private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.subscribe((params) => {
      if (isNaN(+params.page) === false) {
        this.page = +params.linkPage;
      }
      if (isNaN(+params.pageSize) === false) {
        this.pageSize = +params.linkPageSize;
      }
      this.initialiseState(); // reset and set based on new parameter this time
    });
  }

  initialiseState() {
    this.linkService.getAllLinks(this.page, this.pageSize).subscribe((result) => {
      this.linksList = result.data;
      this.resultNumber = result?.pagination?.totalResults;
    });
  }
}
