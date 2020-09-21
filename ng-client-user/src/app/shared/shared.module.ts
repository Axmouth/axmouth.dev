import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NotFoundPageComponent } from './components/not-found-page/not-found-page.component';
import { LinksSideWidgetComponent } from './components/links-side-widget/links-side-widget.component';
import { EditorJsRendererComponent } from './components/editor-js-renderer/editor-js-renderer.component';
import { LinkService } from './services/link.service';
import { RestApiService } from './services/rest-api.service';
import { SafePipe } from './pipes/safe.pipe';
import { HighlightModule, HIGHLIGHT_OPTIONS } from 'ngx-highlightjs';
import { HCaptchaDirective } from './directives/h-captcha.directive';

@NgModule({
  declarations: [
    NotFoundPageComponent,
    LinksSideWidgetComponent,
    EditorJsRendererComponent,
    SafePipe,
    HCaptchaDirective,
  ],
  imports: [CommonModule, HighlightModule],
  exports: [NotFoundPageComponent, LinksSideWidgetComponent, EditorJsRendererComponent, HCaptchaDirective],
  providers: [
    SafePipe,
    HCaptchaDirective,
    LinkService,
    RestApiService,
    {
      provide: HIGHLIGHT_OPTIONS,
      useValue: {
        fullLibraryLoader: () => import('highlight.js'),
        lineNumbersLoader: () => import('highlightjs-line-numbers.js'),
      },
    },
  ],
})
export class SharedModule {}
