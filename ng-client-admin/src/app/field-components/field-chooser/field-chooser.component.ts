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

  constructor() {
    this.contentChange = new EventEmitter();
  }

  ngOnInit(): void {}

  onContentChanged(content: any): void {
    this.content = content;
    this.contentChange.emit(this.content);
  }
}
