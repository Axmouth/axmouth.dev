import { Component, OnInit, OnDestroy, Inject } from '@angular/core';
import { AdminModel } from 'src/app/admin-dashboard/definitions/admin-model';
import { AdminModelService } from 'src/app/admin-dashboard/services/admin-model.service';
import { ActivatedRoute, Router } from '@angular/router';
import { RestApiService } from 'src/app/admin-dashboard/services/rest-api.service';
import { ModelValuesService } from 'src/app/admin-dashboard/services/model-values.service';
import { Title } from '@angular/platform-browser';
import { takeUntil } from 'rxjs/operators';
import { Subject } from 'rxjs';
import { MatSnackBar } from '@angular/material/snack-bar';
import { MatDialog, MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'app-view-entity',
  templateUrl: './view-entity.component.html',
  styleUrls: ['./view-entity.component.scss'],
})
export class ViewEntityComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  model: AdminModel;
  modelName: string;
  categoryName: string;
  id: string;
  entity: object;
  idField: string;
  displayField: string;
  editing = true;

  constructor(
    private modelService: AdminModelService,
    private route: ActivatedRoute,
    private router: Router,
    private apiService: RestApiService,
    private modelValueService: ModelValuesService,
    private title: Title,
    public dialog: MatDialog,
    private snackBar: MatSnackBar,
  ) {}

  ngOnInit(): void {
    const params = this.route.snapshot.paramMap;
    this.categoryName = params.get('categoryName');
    this.modelName = params.get('modelName');
    this.id = params.get('id');
    this.model = this.modelService.getByModelName(this.modelName);

    this.displayField = this.model.displayField || 'id';
    this.idField = this.model.idField || 'id';

    this.apiService
      .getAll<{ data: object }>(this.model.endpoint, {})
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe((result) => {
        this.entity = result.data;
        this.title.setTitle(`${this.modelName} Entity: ${this.id} | Axmouth's Website Admin Site`);
      });
  }

  onSaveClick() {
    this.dialog.open(ExampleDialogComponent, {
      data: {
        title: 'Revert warning',
        body: 'Are you sure you want to save the changes to this object?',
        okText: 'Ok',
        cancelText: 'Cancel',
        okClicked: () => {
          this.modelValueService
            .sendUpdateRequest(this.model.endpoint, this.id)
            .pipe(takeUntil(this.ngUnsubscribe))
            .subscribe(
              (response) => {
                this.snackBar.open(`Changes to ${this.modelName} saved successfully!`, `❌`, { duration: 3000 });
              },
              (err) => {
                console.log(err);
                this.snackBar.open(`Failed to save changes to ${this.modelName}..`, `❌`, { duration: 3000 });
              },
            );
        },
      },
    });
  }

  onRemoveClick() {
    this.dialog.open(ExampleDialogComponent, {
      data: {
        title: 'Delete warning',
        body: 'Are you sure you want to delete this object?',
        okText: 'Ok',
        cancelText: 'Cancel',
        okClicked: () => {
          this.apiService
            .delete(this.model.endpoint, this.id, {})
            .pipe(takeUntil(this.ngUnsubscribe))
            .subscribe(
              (response) => {
                this.router.navigate(['categories', this.categoryName, 'models', this.modelName]);
                this.snackBar.open(`Removed from ${this.modelName} successfully!`, `❌`, { duration: 3000 });
              },
              (err) => {
                console.log(err);
                this.snackBar.open(`Failed to remove from ${this.modelName}..`, `❌`, { duration: 3000 });
              },
            );
        },
      },
    });
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
  templateUrl: '../warning-dialog/warning-dialog.component.html',
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
