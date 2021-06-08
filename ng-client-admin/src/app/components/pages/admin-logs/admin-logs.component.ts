import { Component, OnDestroy, OnInit } from '@angular/core';
import { PageEvent } from '@angular/material/paginator';
import { Title } from '@angular/platform-browser';
import { ActivatedRoute, Router } from '@angular/router';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { AdminCategory } from 'src/app/admin-dashboard/definitions/admin-category';
import { AdminCategoryService } from 'src/app/admin-dashboard/services/admin-category.service';
import { AdminLog } from 'src/app/models/api/admin-log';
import { AdminLogsService } from 'src/app/services/admin-logs.service';

@Component({
  selector: 'app-admin-logs',
  templateUrl: './admin-logs.component.html',
  styleUrls: ['./admin-logs.component.scss'],
})
export class AdminLogsComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  resultNumber = 0;
  page: number;
  pageSize: number;
  sortType?: string;
  actionType?: string;
  loading = true;
  adminLogs: AdminLog[];

  constructor(
    private router: Router,
    private adminLogService: AdminLogsService,
    private route: ActivatedRoute,
    private title: Title,
  ) {}

  ngOnInit(): void {
    this.route.queryParams.pipe(takeUntil(this.ngUnsubscribe)).subscribe((qParams) => {
      if (isNaN(+qParams.page) === false) {
        this.page = +qParams.page ?? 1;
      } else {
        this.page = 1;
      }
      if (isNaN(+qParams.pageSize) === false) {
        this.pageSize = +qParams.pageSize ?? 25;
      } else {
        this.pageSize = 25;
      }
      this.sortType = qParams.sortType ?? 'ActionTimeDesc';
      this.actionType = qParams.action ?? undefined;
      this.initialiseState();
    });
  }

  initialiseState() {
    this.title.setTitle(`Loading Admin Logs | Axmouth's Website Admin Site`);
    this.loading = true;
    this.adminLogService
      .getAll({ sortType: this.sortType, action: this.actionType, pageSize: this.pageSize, page: this.page })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((res) => {
        this.adminLogs = res.data;
        this.resultNumber = res?.pagination?.totalResults;
        console.log(this.resultNumber);
        this.loading = false;
        this.title.setTitle(`Admin Logs | Axmouth's Website Admin Site`);
      });
  }

  handlePageEvent(event: PageEvent) {
    this.router.navigate([], {
      relativeTo: this.route,
      queryParams: { page: event.pageIndex + 1, pageSize: event.pageSize, sortType: this.sortType },
      queryParamsHandling: 'merge',
    });
  }

  sortValueChange(sortType: string) {
    this.router.navigate([], {
      relativeTo: this.route,
      queryParams: { sortType },
      queryParamsHandling: 'merge',
    });
  }

  actionValueChange(action: string) {
    this.router.navigate([], {
      relativeTo: this.route,
      queryParams: { action },
      queryParamsHandling: 'merge',
    });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
