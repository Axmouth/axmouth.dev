import { Component, Input, OnInit } from '@angular/core';
import { AdminLog } from '../../models/api/admin-log';

@Component({
  selector: 'app-admin-log-preview',
  templateUrl: './admin-log-preview.component.html',
  styleUrls: ['./admin-log-preview.component.scss'],
})
export class AdminLogPreviewComponent implements OnInit {
  @Input()
  adminLog: AdminLog;

  constructor() {}

  ngOnInit(): void {}
}
