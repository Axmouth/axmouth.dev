import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from '../../admin-dashboard/definitions/admin-model-field';

@Component({
  selector: 'app-unsigned-integer-field',
  templateUrl: './unsigned-integer-field.component.html',
  styleUrls: ['./unsigned-integer-field.component.scss'],
})
export class UnsignedIntegerFieldComponent implements OnInit {
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
