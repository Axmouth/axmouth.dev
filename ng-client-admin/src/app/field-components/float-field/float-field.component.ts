import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/models/definitions/admin-model-field';

@Component({
  selector: 'app-float-field',
  templateUrl: './float-field.component.html',
  styleUrls: ['./float-field.component.scss'],
})
export class FloatFieldComponent implements OnInit {
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
