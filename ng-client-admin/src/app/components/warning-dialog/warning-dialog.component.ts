import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
@Component({
  selector: 'app-warning-dialog',
  template: '',
})
export class WarningDialogComponent implements OnInit {
  constructor(public dialog: MatDialog) {}
  ngOnInit(): void {}
}
