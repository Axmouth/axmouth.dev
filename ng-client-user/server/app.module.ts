import { Module } from '@nestjs/common';
import { AngularUniversalModule } from '@nestjs/ng-universal';
import { join } from 'path';
import { AppServerModule } from '../src/main.server';

@Module({
  imports: [
    AngularUniversalModule.forRoot({
      bootstrap: AppServerModule,
      viewsPath: join(process.cwd(), 'dist/ng-user-client/browser'),
      cache: true,
    }),
  ],
})
export class AppModule {}
