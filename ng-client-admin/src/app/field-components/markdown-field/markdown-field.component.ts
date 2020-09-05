import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/models/definitions/admin-model-field';

@Component({
  selector: 'app-markdown-field',
  templateUrl: './markdown-field.component.html',
  styleUrls: ['./markdown-field.component.scss'],
})
export class MarkdownFieldComponent implements OnInit {
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
