import { Component, OnInit, OnDestroy } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { AdminCategory } from 'src/app/admin-dashboard/definitions/admin-category';
import { AdminCategoryService } from 'src/app/admin-dashboard/services/admin-category.service';
import { AdminLogsService } from '../../services/admin-logs.service';
import { AdminLog } from '../../models/api/admin-log';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss'],
})
export class HomeComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  categories: AdminCategory[];
  adminLogs: AdminLog[];

  constructor(
    private categoryService: AdminCategoryService,
    private adminLogService: AdminLogsService,
    private title: Title,
  ) {}

  ngOnInit(): void {
    this.title.setTitle(`Index | Axmouth's Website Admin Site`);
    this.categories = this.categoryService.getAll();
    this.adminLogService
      .getAll({ sortType: 'ActionTimeDesc' })
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((res) => {
        this.adminLogs = res.data;
      });
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
