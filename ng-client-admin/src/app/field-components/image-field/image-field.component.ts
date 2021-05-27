import { Component, OnInit, Input, Output, EventEmitter, ElementRef, ViewChild } from '@angular/core';
import { AdminModelField } from 'src/app/models/definitions/admin-model-field';
import { BehaviorSubject, Observable } from 'rxjs';
import { ModelValuesService } from 'src/app/services/model-values.service';
import { UploadService } from '../../services/upload.service';
import { apiRoot } from 'src/environments/environment';

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
  @ViewChild('FileSelectInputDialog')
  FileSelectInputDialog: ElementRef;
  imageUploading = false;

  constructor(private modelValuesService: ModelValuesService, private uploadService: UploadService) {
    this.contentChange = new EventEmitter();
  }

  ngOnInit(): void {
    this.subject = this.modelValuesService.addField(this.fieldOptions.identifier, undefined);
    if (this.content) {
      this.subject.next(this.content);
    }
  }

  onChange(text: string) {
    console.log(text);
    this.content = text;
    this.subject.next(text);
    this.contentChange.next(text);
  }

  onOpenImageClick() {
    const e: HTMLElement = this.FileSelectInputDialog.nativeElement;
    e.click();
  }

  uploadImage(file: File): Observable<{ success: number; file: { url: string } }> {
    const url = `${apiRoot}/files/upload/image`;
    const result = this.uploadService.uploadFile<{ success: number; file: { url: string } }>(url, 'image', file);
    return result;
  }

  onFileAdded() {
    const files: { [key: string]: File } = this.FileSelectInputDialog.nativeElement.files;
    for (const file in files) {
      if (!isNaN(parseInt(file, 10))) {
        this.imageUploading = true;
        this.uploadImage(files[file]).subscribe((uploadResult) => {
          this.content = uploadResult.file.url;
          this.subject.next(this.content);
          this.imageUploading = false;
        });
      }
    }
  }
}
