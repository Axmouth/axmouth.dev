import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/models/definitions/admin-model-field';
import { BehaviorSubject } from 'rxjs';
import { ModelValuesService } from 'src/app/services/model-values.service';

@Component({
  selector: 'app-image-field',
  templateUrl: './image-field.component.html',
  styleUrls: ['./image-field.component.scss'],
})
export class ImageFieldComponent implements OnInit {
  @Input()
  content: any;
  @Input()
  fieldOptions: AdminModelField;
  @Output()
  contentChange: EventEmitter<any>;
  subject: BehaviorSubject<string>;

  constructor(private modelValuesService: ModelValuesService) {
    this.contentChange = new EventEmitter();
  }

  ngOnInit(): void {
    this.subject = this.modelValuesService.addField(this.fieldOptions.identifier, null);
    if (this.content === undefined) {
      this.content = null;
    }
    if (this.content) {
      this.subject.next(this.content);
    }
  }

  onChange(text: string) {
    console.log(text);
    this.subject.next(text);
  }
}
