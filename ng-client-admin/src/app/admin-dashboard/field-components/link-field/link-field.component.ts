import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/admin-dashboard/definitions/admin-model-field';

@Component({
  selector: 'app-link-field',
  templateUrl: './link-field.component.html',
  styleUrls: ['./link-field.component.scss'],
})
export class LinkFieldComponent implements OnInit {
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
