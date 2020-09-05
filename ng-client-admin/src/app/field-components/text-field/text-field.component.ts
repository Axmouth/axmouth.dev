import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/models/definitions/admin-model-field';

@Component({
  selector: 'app-text-field',
  templateUrl: './text-field.component.html',
  styleUrls: ['./text-field.component.scss'],
})
export class TextFieldComponent implements OnInit {
  @Input()
  content: any;
  @Input()
  fieldOptions: AdminModelField;
  @Output()
  contentChange: EventEmitter<any>;

  constructor() {
    this.contentChange = new EventEmitter();
  }

  ngOnInit(): void {}
}
