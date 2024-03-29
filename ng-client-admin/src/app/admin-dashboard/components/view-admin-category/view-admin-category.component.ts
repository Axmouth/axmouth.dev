import { Component, OnInit } from '@angular/core';
import { AdminCategory } from 'src/app/admin-dashboard/definitions/admin-category';
import { AdminCategoryService } from 'src/app/admin-dashboard/services/admin-category.service';
import { ActivatedRoute } from '@angular/router';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-view-admin-category',
  templateUrl: './view-admin-category.component.html',
  styleUrls: ['./view-admin-category.component.scss'],
})
export class ViewAdminCategoryComponent implements OnInit {
  loading = true;
  categoryName: string;
  category: AdminCategory;

  constructor(private categoryService: AdminCategoryService, private route: ActivatedRoute, private title: Title) {}

  ngOnInit(): void {
    const params = this.route.snapshot.paramMap;
    this.categoryName = params.get('categoryName');
    this.category = this.categoryService.get(this.categoryName);
    this.title.setTitle(`Category: ${this.categoryName} | Axmouth's Website Admin Site`);
  }
}
