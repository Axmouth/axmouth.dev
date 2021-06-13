import { Component, Inject, OnDestroy, OnInit } from '@angular/core';
import { MatDialog, MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { Title } from '@angular/platform-browser';
import { ActivatedRoute } from '@angular/router';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { AdminModel } from 'src/app/admin-dashboard/definitions/admin-model';
import { AdminModelService } from 'src/app/admin-dashboard/services/admin-model.service';
import { RestApiService } from 'src/app/admin-dashboard/services/rest-api.service';
import { AdminLog } from 'src/app/models/api/admin-log';
import { AdminLogsService } from 'src/app/services/admin-logs.service';
import { apiRoot } from 'src/environments/environment';

@Component({
  selector: 'app-admin-log-details',
  templateUrl: './admin-log-details.component.html',
  styleUrls: ['./admin-log-details.component.scss'],
})
export class AdminLogDetailsComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  adminLog: AdminLog;
  adminLogId: string;
  model: AdminModel;
  oldData: any;
  newData: any;
  viewEditorMode = false;

  constructor(
    private modelService: AdminModelService,
    private adminLogService: AdminLogsService,
    private apiService: RestApiService,
    private route: ActivatedRoute,
    private title: Title,
    public dialog: MatDialog,
  ) {}

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
        this.model = this.modelService.getByModelId(res.data.model);
        console.log(res);
        console.log(this.model);
        this.newData = JSON.parse(res.data.newData);
        this.oldData = JSON.parse(res.data.oldData);
      });
  }

  onRecreateClick() {
    this.dialog.open(ExampleDialogComponent, {
      data: {
        title: 'Create warning',
        body: 'Are you sure you want to recreate this object?',
        okText: 'Ok',
        cancelText: 'Cancel',
        okClicked: () => {
          const root = apiRoot;
          this.apiService
            .create(`${root}${this.adminLog.baseLink}`, JSON.parse(this.adminLog.oldData), {})
            .pipe(takeUntil(this.ngUnsubscribe))
            .subscribe();
        },
      },
    });
  }

  onDeleteClick() {
    this.dialog.open(ExampleDialogComponent, {
      data: {
        title: 'Delete warning',
        body: 'Are you sure you want to delete this object?',
        okText: 'Ok',
        cancelText: 'Cancel',
        okClicked: () => {
          const root = apiRoot;
          this.apiService
            .delete(`${root}${this.adminLog.baseLink}`, this.adminLog.objectId, {})
            .pipe(takeUntil(this.ngUnsubscribe))
            .subscribe();
        },
      },
    });
  }

  onRevertUpdateClick() {
    this.dialog.open(ExampleDialogComponent, {
      data: {
        title: 'Revert warning',
        body: 'Are you sure you want to revert this object?',
        okText: 'Ok',
        cancelText: 'Cancel',
        okClicked: () => {
          const root = apiRoot;
          this.apiService
            .patchUpdate(
              `${root}${this.adminLog.baseLink}`,
              this.adminLog.objectId,
              JSON.parse(this.adminLog.oldData),
              {},
            )
            .pipe(takeUntil(this.ngUnsubscribe))
            .subscribe();
        },
      },
    });
  }

  onDialogClick(ok: boolean) {
    console.log('onDialogClick');
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}

export interface DialogData<T> {
  title: string;
  body: string;
  okText: string;
  cancelText: string;
  okClicked: () => void;
}

@Component({
  selector: 'app-example-dialog',
  templateUrl: '../../warning-dialog/warning-dialog.component.html',
})
export class ExampleDialogComponent {
  constructor(
    public dialogRef: MatDialogRef<ExampleDialogComponent>,
    @Inject(MAT_DIALOG_DATA) public data: DialogData<void>,
  ) {}

  onCancelClick(): void {
    this.dialogRef.close();
  }

  onOkClick(): void {
    this.dialogRef.close();
    this.data.okClicked();
  }
}
