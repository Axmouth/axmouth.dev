import { Component, Inject, OnInit } from '@angular/core';
import { AdminModel } from 'src/app/admin-dashboard/definitions/admin-model';
import { AdminModelService } from 'src/app/admin-dashboard/services/admin-model.service';
import { ActivatedRoute, Router } from '@angular/router';
import { ModelValuesService } from 'src/app/admin-dashboard/services/model-values.service';
import { Title } from '@angular/platform-browser';
import { MatSnackBar } from '@angular/material/snack-bar';
import { MatDialog, MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'app-create-entity',
  templateUrl: './create-entity.component.html',
  styleUrls: ['./create-entity.component.scss'],
})
export class CreateEntityComponent implements OnInit {
  model: AdminModel;
  modelName: string;
  categoryName: string;

  constructor(
    private modelService: AdminModelService,
    private route: ActivatedRoute,
    private router: Router,
    private modelValuesService: ModelValuesService,
    private title: Title,
    public dialog: MatDialog,
    private snackBar: MatSnackBar,
  ) {}

  ngOnInit(): void {
    const params = this.route.snapshot.paramMap;
    this.modelName = params.get('modelName');
    this.categoryName = params.get('categoryName');
    this.model = this.modelService.getByModelName(this.modelName);
    this.title.setTitle(`Create a ${this.modelName} | Axmouth's Website Admin Site`);
  }

  onSaveClick() {
    this.dialog.open(ExampleDialogComponent, {
      data: {
        title: 'Create warning',
        body: 'Are you sure you want to create this object?',
        okText: 'Ok',
        cancelText: 'Cancel',
        okClicked: () => {
          this.modelValuesService.sendCreateRequest(this.model.endpoint).subscribe(
            (response) => {
              this.router.navigate(['categories', this.categoryName, 'models', this.modelName, { duration: 3000 }]);
              this.snackBar.open(`Successfully added to ${this.modelName}!`, `❌`);
            },
            (err) => {
              console.log(err);
              this.snackBar.open(`Failed to add to ${this.modelName}..`, `❌`, { duration: 3000 });
            },
          );
        },
      },
    });
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
