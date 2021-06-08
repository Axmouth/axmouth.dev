import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/admin-dashboard/definitions/admin-model-field';

@Component({
  selector: 'app-integer-field',
  templateUrl: './integer-field.component.html',
  styleUrls: ['./integer-field.component.scss'],
})
export class IntegerFieldComponent implements OnInit {
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
