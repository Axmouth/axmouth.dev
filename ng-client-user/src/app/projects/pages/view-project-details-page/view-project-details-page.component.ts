import { Component, OnInit, Input, OnDestroy, Inject } from '@angular/core';
import { ProjectService } from '../../services/project.service';
import { ActivatedRoute } from '@angular/router';
import { Project } from '../../../models/api/project';
import { Meta, Title } from '@angular/platform-browser';
import { Subject } from 'rxjs';
import { takeUntil } from 'rxjs/operators';
import { DOCUMENT } from '@angular/common';
import { websiteUrl } from 'src/environments/environment';

@Component({
  selector: 'app-view-project-details-page',
  templateUrl: './view-project-details-page.component.html',
  styleUrls: ['./view-project-details-page.component.scss'],
})
export class ViewProjectDetailsPageComponent implements OnInit, OnDestroy {
  ngUnsubscribe = new Subject<void>();
  @Input()
  project: Project;
  projectTechnologiesList: string[] = [];
  projectId: string;
  projectBodyData: any[];
  notFound = false;
  loading = true;

  constructor(
    private projectService: ProjectService,
    private route: ActivatedRoute,
    private title: Title,
    private meta: Meta,
    @Inject(DOCUMENT) private doc: Document,
  ) {}

  ngOnInit(): void {
    this.title.setTitle(`Loading Project | Axmouth's Website`);
    const params = this.route.snapshot.paramMap;
    this.projectId = params.get('id');
    this.projectService
      .getProject(this.projectId)
      .pipe(takeUntil(this.ngUnsubscribe))
      .subscribe(
        (result) => {
          this.project = result.data;
          this.projectBodyData = JSON.parse(result.data.body);
          this.projectTechnologiesList = result.data.technologies;
          this.loading = false;
          this.title.setTitle(`${this.project.name} | Axmouth's Website`);
          this.meta.updateTag({ name: `title`, content: this.title.getTitle() });
          this.meta.updateTag({ name: `description`, content: this?.project?.description });
          this.meta.updateTag({
            name: `keywords`,
            content: `axmouth,developer,webdev,programmer,portfolio,${this.projectTechnologiesList.join(',')}`,
          });
          this.meta.updateTag({
            property: `og:url`,
            content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
          });
          this.meta.updateTag({ property: `og:title`, content: this.title.getTitle() });
          this.meta.updateTag({ property: `og:description`, content: this?.project?.description });
          this.meta.updateTag({ property: `og:image`, content: this?.project?.coverImage });
          this.meta.updateTag({ property: `twitter:card`, content: this?.project?.coverImage });
          this.meta.updateTag({
            property: `twitter:url`,
            content: this.doc.location.href.replace(this.doc.location.origin, websiteUrl),
          });
          this.meta.updateTag({ property: `twitter:title`, content: this.title.getTitle() });
          this.meta.updateTag({ property: `twitter:description`, content: this?.project?.description });
          this.meta.updateTag({ property: `twitter:image`, content: this?.project?.coverImage });
        },
        (error) => {
          console.log(error);
          if (error.status === 404) {
            this.notFound = true;
            this.title.setTitle(`Project Not Found | Axmouth's Website`);
          }
          this.loading = false;
        },
      );
  }

  ngOnDestroy(): void {
    this.ngUnsubscribe.next();
    this.ngUnsubscribe.complete();
  }
}
