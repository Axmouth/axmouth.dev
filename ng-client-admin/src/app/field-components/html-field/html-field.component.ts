import {
  Component,
  OnInit,
  Input,
  Output,
  EventEmitter,
  AfterViewInit,
  ViewChild,
  ElementRef,
  ViewContainerRef,
} from '@angular/core';
import { AdminModelField } from 'src/app/models/definitions/admin-model-field';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import EditorJS from '@editorjs/editorjs';
import Header from '@editorjs/header';
import List from '@editorjs/list';
import * as SimpleImage from '@editorjs/simple-image';
import * as ImageTool from '@editorjs/image';
import InlineCode from '@editorjs/inline-code';
import Link from '@editorjs/link';
import * as Embed from '@editorjs/embed';
import * as Paragraph from '@editorjs/paragraph';
import Quote from '@editorjs/quote';
import Warning from '@editorjs/warning';
import RawTool from '@editorjs/raw';
import Delimiter from '@editorjs/delimiter';
import * as CheckList from '@editorjs/checklist';
import * as UnderLine from '@editorjs/underline';
import * as Table from '@editorjs/table';
import * as Personality from '@editorjs/personality';
import * as Marker from '@editorjs/marker';
import * as Code from '@editorjs/code';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { AuthService } from '../../../auth/services/auth.service';
import { of } from 'rxjs';

@Component({
  selector: 'app-html-field',
  templateUrl: './html-field.component.html',
  styleUrls: ['./html-field.component.scss'],
})
export class HtmlFieldComponent implements OnInit, AfterViewInit {
  @ViewChild('editorJs') el: ElementRef;
  @Input()
  content: any;
  @Input()
  fieldOptions: AdminModelField;
  @Output()
  contentChange: EventEmitter<any> = new EventEmitter();

  constructor(private http: HttpClient, private authService: AuthService) {}

  ngOnInit(): void {}

  ngAfterViewInit() {
    const http = this.http;
    const authService = this.authService;
    const editor = new EditorJS({
      holder: this.el.nativeElement,
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
                const _ = await authService.isAuthenticatedOrRefresh().toPromise();
                const myFormData = new FormData();
                const headers = new HttpHeaders();
                headers.append('Content-Type', 'multipart/form-data');
                headers.append('Accept', 'application/json');
                myFormData.append('image', file);
                return http
                  .post<{ success: number; file: { url: string } }>(
                    'http://localhost:39051/api/v1/files/upload/editorjs',
                    myFormData,
                    {
                      headers,
                      withCredentials: true,
                    },
                  )
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
            /*
            endpoints: {
              byFile: 'http://[::1]:39051/api/v1/files/upload', // Your backend file uploader endpoint
              byUrl: 'http://localhost:8008/fetchUrl', // Your endpoint that provides uploading by Url
            },*/
          },
        },
        inlineCode: InlineCode,
        link: Link,
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
        personality: {
          class: Personality,
          config: {
            endpoint: 'http://[::1]:39051/api/v1/files/upload', // Your backend file uploader endpoint
          },
        },
        marker: Marker,
        code: Code,
      },
    });
  }
}
