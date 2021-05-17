import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { FieldType, AdminModelField } from 'src/app/models/definitions/admin-model-field';

@Component({
  selector: 'app-field-chooser',
  templateUrl: './field-chooser.component.html',
  styleUrls: ['./field-chooser.component.scss'],
})
export class FieldChooserComponent implements OnInit {
  @Input()
  fieldOptions: AdminModelField;
  @Input()
  content: any;
  @Output()
  contentChange: EventEmitter<any>;

  isNull = false;
  isSkipped = false;

  constructor() {
    this.contentChange = new EventEmitter();
  }

  ngOnInit(): void {
    if (this.content === null) {
      this.isNull = true;
    }
    if (this.content === undefined) {
      this.isSkipped = true;
    }
  }

  onNullifyChanged(value: boolean) {
    this.isNull = value;
    if (value === true) {
      this.contentChange.emit(null);
    } else {
      this.contentChange.emit(this.content);
    }
  }

  onSkipChanged(value: boolean) {
    this.isSkipped = value;
    if (value === true) {
      this.contentChange.emit(undefined);
    } else {
      this.contentChange.emit(this.content);
    }
  }

  onContentChanged(content: any): void {
    this.content = content;
    this.contentChange.emit(this.content);
  }
}
