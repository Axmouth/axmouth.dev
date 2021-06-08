import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/admin-dashboard/definitions/admin-model-field';

@Component({
  selector: 'app-unsigned-float-field',
  templateUrl: './unsigned-float-field.component.html',
  styleUrls: ['./unsigned-float-field.component.scss'],
})
export class UnsignedFloatFieldComponent implements OnInit {
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
