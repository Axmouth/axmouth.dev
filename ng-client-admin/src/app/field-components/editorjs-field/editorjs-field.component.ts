import { Component, OnInit, AfterViewInit, ViewChild, ElementRef, Input, Output, EventEmitter } from '@angular/core';
import { AdminModelField } from 'src/app/models/definitions/admin-model-field';
import { of, BehaviorSubject } from 'rxjs';
import EditorJS, { API, OutputData } from '@editorjs/editorjs';
import * as Header from '@editorjs/header';
import * as List from '@editorjs/list';
import * as SimpleImage from '@editorjs/simple-image';
import * as ImageTool from '@editorjs/image';
import * as InlineCode from '@editorjs/inline-code';
import * as Link from '@editorjs/link';
import * as Embed from '@editorjs/embed';
import * as Paragraph from '@editorjs/paragraph';
import * as Quote from '@editorjs/quote';
import * as Warning from '@editorjs/warning';
import * as RawTool from '@editorjs/raw';
import * as Delimiter from '@editorjs/delimiter';
import * as CheckList from '@editorjs/checklist';
import * as UnderLine from '@editorjs/underline';
import * as Table from '@editorjs/table';
import * as Personality from '@editorjs/personality';
import * as Marker from '@editorjs/marker';
import * as Code from '@editorjs/code';
import { ModelValuesService } from '../../services/model-values.service';
import { apiRoot } from 'src/environments/environment';
import { UploadService } from '../../services/upload.service';

const defaultData = '{"time":1597135214246,"blocks":[],"version":"2.18.0"}';

@Component({
  selector: 'app-editorjs-field',
  templateUrl: './editorjs-field.component.html',
  styleUrls: ['./editorjs-field.component.scss'],
})
export class EditorjsFieldComponent implements OnInit, AfterViewInit {
  @ViewChild('editorJs') el: ElementRef;
  @ViewChild('editorJs2') el2: ElementRef;
  @Input()
  content: any;
  @Input()
  fieldOptions: AdminModelField;
  @Output()
  contentChange: EventEmitter<any> = new EventEmitter();
  editor: EditorJS;
  subject: BehaviorSubject<string>;

  constructor(private modelValuesService: ModelValuesService, private uploadService: UploadService) {}

  ngOnInit(): void {
    this.subject = this.modelValuesService.addField(this.fieldOptions.identifier, null);
    console.log(this.content);
    if (this.content) {
      this.subject.next(this.content);
    }
  }

  ngAfterViewInit() {
    const uploadService = this.uploadService;
    const editor = new EditorJS({
      onChange: async (api: API) => {
        const data: OutputData = await this.editor.save();
        this.subject.next(JSON.stringify(data));
      },
      onReady: async () => {
        editor.render(JSON.parse(this.content ?? defaultData));
      },
      holder: this.el.nativeElement,
      readOnly: false,
      tools: {
        header: Header,
        list: {
          class: List,
          inlineToolbar: true,
        },
        image: {
          class: ImageTool,
          config: {
            uploader: {
              async uploadByFile(file: File): Promise<{ success: number; file: { url: string } }> {
                const url = `${apiRoot}/files/upload/editorjs`;
                return uploadService
                  .uploadFile<{ success: number; file: { url: string } }>(url, 'image', file)
                  .toPromise();
              },

              async uploadByUrl(url: string): Promise<{ success: number; file: { url: string } }> {
                return of({
                  success: 1,
                  file: {
                    url,
                    // any other image data you want to store, such as width, height, color, extension, etc
                  },
                }).toPromise();
              },
            },
          },
        },
        inlineCode: InlineCode,
        link: {
          class: Link,
          config: {
            endpoint: `${apiRoot}/fetch-url`, // Your backend endpoint for url data fetching
          },
        },
        embed: Embed,
        paragraph: {
          class: Paragraph,
          inlineToolbar: true,
        },
        quote: {
          class: Quote,
          inlineToolbar: true,
          shortcut: 'CMD+SHIFT+O',
          config: {
            quotePlaceholder: 'Enter a quote',
            captionPlaceholder: 'Author of quote',
          },
        },
        warning: {
          class: Warning,
          inlineToolbar: true,
          shortcut: 'CMD+SHIFT+W',
          config: {
            titlePlaceholder: 'Title',
            messagePlaceholder: 'Message',
          },
        },
        raw: RawTool,
        delimiter: Delimiter,
        checkList: CheckList,
        underline: UnderLine,
        table: {
          class: Table,
          inlineToolbar: true,
          config: {
            rows: 2,
            cols: 3,
          },
        },
        /*
        personality: {
          class: Personality,
          config: {
            uploader: {
              async uploadByFile(file: File): Promise<{ success: number; file: { url: string } }> {
                const url = `${apiRoot}/files/upload/editorjs`;
                return uploadService
                  .uploadFile<{ success: number; file: { url: string } }>(url, 'image', file)
                  .toPromise();
              },
            },
          },
        },
        */
        marker: Marker,
        code: Code,
      },
    });
    // End of editor initialization
    this.editor = editor;
  }
}
