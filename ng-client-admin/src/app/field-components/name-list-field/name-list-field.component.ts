import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/models/definitions/admin-model-field';
import { ModelValuesService } from '../../services/model-values.service';
import { MatDialog } from '@angular/material/dialog';
import { AddToListDialogComponent } from '../add-to-list-dialog/add-to-list-dialog.component';
import { BehaviorSubject } from 'rxjs';

@Component({
  selector: 'app-name-list-field',
  templateUrl: './name-list-field.component.html',
  styleUrls: ['./name-list-field.component.scss'],
})
export class NameListFieldComponent implements OnInit {
  @Input()
  content: string[];
  @Input()
  fieldOptions: AdminModelField;
  @Output()
  contentChange: EventEmitter<any>;
  contentSelected: string[] = [];
  subject: BehaviorSubject<string[]>;

  constructor(public dialog: MatDialog, private modelValuesService: ModelValuesService) {
    this.contentChange = new EventEmitter();
  }

  ngOnInit(): void {
    this.subject = this.modelValuesService.addField(this.fieldOptions.identifier, undefined);
    if (this.content) {
      this.subject.next(this.content);
    }
  }

  onAddClick(): void {
    const dialogRef = this.dialog.open(AddToListDialogComponent, {
      width: '250px',
      data: { input: '' },
    });

    dialogRef.afterClosed().subscribe((result) => {
      console.log('The dialog was closed');
      if (!result) {
        return;
      }
      // this.animal = result;
      if (this.content === null || this.content === undefined) {
        this.content = [];
      }
      this.content.push(result);
      this.subject.next(this.content);
      this.contentChange.next(this.content);
    });
  }

  onRemoveClick(): void {
    if (!this.content) {
      return;
    }
    for (const selected of this.contentSelected) {
      const index = this.content.indexOf(selected, 0);
      this.content.splice(index, 1);
      this.contentChange.next(this.content);
    }
  }
}
