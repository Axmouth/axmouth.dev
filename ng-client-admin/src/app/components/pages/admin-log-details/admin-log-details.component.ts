import { Component, OnDestroy, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ActivatedRoute } from '@angular/router';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { AdminLog } from 'src/app/models/api/admin-log';
import { AdminLogsService } from 'src/app/services/admin-logs.service';

@Component({
  selector: 'app-admin-log-details',
  templateUrl: './admin-log-details.component.html',
  styleUrls: ['./admin-log-details.component.scss'],
})
export class AdminLogDetailsComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  adminLog: AdminLog;
  adminLogId: string;

  constructor(private adminLogService: AdminLogsService, private route: ActivatedRoute, private title: Title) {}

  ngOnInit(): void {
    this.route.params.pipe(takeUntil(this.ngUnsubscribe)).subscribe((params) => {
      this.adminLogId = params.adminLogId;
      this.initialiseState(); // reset and set based on new parameter this time
    });
  }

  initialiseState() {
    this.title.setTitle(`Loading Admin Logs | Axmouth's Website Admin Site`);
    this.adminLogService
      .get(this.adminLogId, {})
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((res) => {
        this.adminLog = res.data;
        this.title.setTitle(`Admin Logs | Axmouth's Website Admin Site`);
      });
  }

  onRevertClick() {
    if (this.adminLog.action === 'Create') {
    } else if (this.adminLog.action === 'Update') {
    } else if (this.adminLog.action === 'Delete') {
    }
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
