import { apiRoot } from '../../environments/environment';

const Base = apiRoot;

const Organizations: any = {
  GetAll: Base + '/organizations',
  Update: Base + '/organizations/{organizationName}',
  Get: Base + '/organizations/{organizationName}',
  Create: Base + '/organizations',
  Delete: Base + '/organizations/{organizationName}',
};

const Users: any = {
  GetAll: Base + '/users',
  Update: Base + '/users/{username}',
  Get: Base + '/users/{username}',
  Create: Base + '/users',
  Delete: Base + '/users/{username}',
};

const Projects = {
  GetAll: Base + '/projects-search',
};

Users.Projects = {
  GetAll: Users.Get + '/projects',
  Update: Users.Get + '/projects/{projectName}',
  Get: Users.Get + '/projects/{projectName}',
  Create: Users.Get + '/projects',
  Delete: Users.Get + '/projects/{projectName}',
};

Organizations.Projects = {
  GetAll: Organizations.Get + '/projects',
  Update: Organizations.Get + '/projects/{projectName}',
  Get: Organizations.Get + '/projects/{projectName}',
  Create: Organizations.Get + '/projects',
  Delete: Organizations.Get + '/projects/{projectName}',
};

Organizations.Projects.Issues = {
  GetAll: Organizations.Projects.Get + '/issues',
  Update: Organizations.Projects.Get + '/issues/{issueNumber}',
  Get: Organizations.Projects.Get + '/issues/{issueNumber}',
  Create: Organizations.Projects.Get + '/issues',
  Delete: Organizations.Projects.Get + '/issues/{issueNumber}',
};

Users.Projects.Issues = {
  GetAll: Users.Projects.Get + '/issues',
  Update: Users.Projects.Get + '/issues/{issueNumber}',
  Get: Users.Projects.Get + '/issues/{issueNumber}',
  Create: Users.Projects.Get + '/issues',
  Delete: Users.Projects.Get + '/issues/{issueNumber}',
};

Organizations.Projects.Issues.IssueStages = {
  GetAll: Organizations.Projects.Get + '/issue-stages',
  Update: Organizations.Projects.Get + '/issue-stages/{issueStageName}',
  Get: Organizations.Projects.Get + '/issue-stages/{issueStageName}',
  Create: Organizations.Projects.Get + '/issue-stages',
  Delete: Organizations.Projects.Get + '/issue-stages/{issueStageName}',
};
Organizations.Projects.IssueStages = {
  Get: Organizations.Projects.Issues.Get + '/issue-stage',
  Create: Organizations.Projects.Issues.Get + '/issue-stage',
};

Users.Projects.Issues.IssueStages = {
  GetAll: Users.Projects.Get + '/issue-stages',
  Update: Users.Projects.Get + '/issue-stages/{issueStageName}',
  Get: Users.Projects.Get + '/issue-stages/{issueStageName}',
  Create: Users.Projects.Get + '/issue-stages',
  Delete: Users.Projects.Get + '/issue-stages/{issueStageName}',
};
Users.Projects.IssueStages = {
  Get: Users.Projects.Issues.Get + '/issue-stage',
  Create: Users.Projects.Issues.Get + '/issue-stage',
};

Organizations.Projects.IssueTypes = {
  GetAll: Organizations.Projects.Issues.Get + '/issue-types',
  Update: Organizations.Projects.Issues.Get + '/issue-types/{issueTypeName}',
  Get: Organizations.Projects.Issues.Get + '/issue-types/{issueTypeName}',
  Create: Organizations.Projects.Issues.Get + '/issue-types',
  Delete: Organizations.Projects.Issues.Get + '/issue-types/{issueTypeName}',
};

Users.Projects.IssueTypes = {
  GetAll: Users.Projects.Issues.Get + '/issue-types',
  Update: Users.Projects.Issues.Get + '/issue-types/{issueTypeName}',
  Get: Users.Projects.Issues.Get + '/issue-types/{issueTypeName}',
  Create: Users.Projects.Issues.Get + '/issue-types',
  Delete: Users.Projects.Issues.Get + '/issue-types/{issueTypeName}',
};

Organizations.Projects.Issues.IssueTypes = {
  GetAll: Organizations.Projects.Get + '/issue-types',
  Update: Organizations.Projects.Get + '/issue-types/{issueTypeName}',
  Get: Organizations.Projects.Get + '/issue-types/{issueTypeName}',
  Create: Organizations.Projects.Get + '/issue-types',
  Delete: Organizations.Projects.Get + '/issue-types/{issueTypeName}',
};

Users.Projects.IssueTypes = {
  GetAll: Users.Projects.Get + '/issue-types',
  Update: Users.Projects.Get + '/issue-types/{issueTypeName}',
  Get: Users.Projects.Get + '/issue-types/{issueTypeName}',
  Create: Users.Projects.Get + '/issue-types',
  Delete: Users.Projects.Get + '/issue-types/{issueTypeName}',
};

const IssuePosts: any = {
  GetAll: Users.Get + '/issue-posts',
};

Organizations.Projects.Issues.IssuePosts = {
  GetAll: Organizations.Projects.Issues.Get + '/issue-posts',
  Update: Organizations.Projects.Issues.Get + '/issue-posts/{issuePostNumber}',
  Get: Organizations.Projects.Issues.Get + '/issue-posts/{issuePostNumber}',
  Create: Organizations.Projects.Issues.Get + '/issue-posts',
  Delete: Organizations.Projects.Issues.Get + '/issue-posts/{issuePostNumber}',
};

Users.Projects.Issues.IssuePosts = {
  GetAll: Users.Projects.Issues.Get + '/issue-posts',
  Update: Users.Projects.Issues.Get + '/issue-posts/{issuePostNumber}',
  Get: Users.Projects.Issues.Get + '/issue-posts/{issuePostNumber}',
  Create: Users.Projects.Issues.Get + '/issue-posts',
  Delete: Users.Projects.Issues.Get + '/issue-posts/{issuePostNumber}',
};

Users.IssuePosts = {
  GetAll: Users.Get + '/issue-posts',
};

Organizations.Projects.Issues.IssuePosts.IssuePostReactions = {
  GetAll: Organizations.Projects.Issues.IssuePosts.Get + '/issue-posts',
  Update: Organizations.Projects.Issues.IssuePosts.Get + '/issue-posts/{reactionId}',
  Get: Organizations.Projects.Issues.IssuePosts.Get + '/issue-posts/{reactionId}',
  Create: Organizations.Projects.Issues.IssuePosts.Get + '/issue-posts',
  Delete: Organizations.Projects.Issues.IssuePosts.Get + '/issue-posts/{reactionId}',
};

Users.Projects.Issues.IssuePosts.IssuePostReactions = {
  GetAll: Users.Projects.Issues.IssuePosts.GetAll + '/issue-posts',
  Update: Users.Projects.Issues.IssuePosts.GetAll + '/issue-posts/{reactionId}',
  Get: Users.Projects.Issues.IssuePosts.GetAll + '/issue-posts/{reactionId}',
  Create: Users.Projects.Issues.IssuePosts.GetAll + '/issue-posts',
  Delete: Users.Projects.Issues.IssuePosts.GetAll + '/issue-posts/{reactionId}',
};

Organizations.Projects.Issues.IssuePostUpdates = {
  GetAll: Organizations.Projects.Issues.Get + '/issue-post-updates',
  Update: Organizations.Projects.Issues.Get + '/issue-post-updates/{issueUpdateNumber}',
  Get: Organizations.Projects.Issues.Get + '/issue-post-updates/{issueUpdateNumber}',
  Create: Organizations.Projects.Issues.Get + '/issue-post-updates',
  Delete: Organizations.Projects.Issues.Get + '/issue-post-updates/{issueUpdateNumber}',
};

Users.Projects.Issues.IssuePostUpdates = {
  GetAll: Users.Projects.Issues.Get + '/issue-post-updates',
  Update: Users.Projects.Issues.Get + '/issue-post-updates/{issueUpdateNumber}',
  Get: Users.Projects.Issues.Get + '/issue-post-updates/{issueUpdateNumber}',
  Create: Users.Projects.Issues.Get + '/issue-post-updates',
  Delete: Users.Projects.Issues.Get + '/issue-post-updates/{issueUpdateNumber}',
};

const Identity = {
  Login: Base + '/identity/login',
  Logout: Base + '/identity/logout',
  Register: Base + '/identity/register',
  Refresh: Base + '/identity/refresh',
  Profile: Base + '/identity/profile',
  PasswordReset: Base + '/identity/password-reset',
  PasswordResetEmail: Base + '/identity/password-reset-email',
  PasswordChange: Base + '/identity/password-change',
  EmailConfirm: Base + '/identity/email-confirm',
  EmailConfirmEmail: Base + '/identity/email-confirm-email',
};

const Notifications = {
  GetAll: Base + '/notifications',
  Update: Base + '/notifications/{Id}',
  Get: Base + '/notifications/{Id}',
  Create: Base + '/notifications',
  Delete: Base + '/notifications/{Id}',
};

const AntiForgery = {
  Get: Base + '/antiforgery',
};

Organizations.Projects.ProjectAdmins = {
  Get: Organizations.Projects.Get + '/admins/{adminUserName}',
  GetAll: Organizations.Projects.Get + '/admins',
  Create: Organizations.Projects.Get + '/admins',
  Delete: Organizations.Projects.Get + '/admins/{adminUserName}',
};

Users.Projects.ProjectAdmins = {
  Get: Users.Projects.Get + '/admins/{adminUserName}',
  GetAll: Users.Projects.Get + '/admins',
  Create: Users.Projects.Get + '/admins',
  Delete: Users.Projects.Get + '/admins/{adminUserName}',
};

Organizations.Projects.ProjectMaintainers = {
  Get: Organizations.Projects.Get + '/maintainers/{maintainerUserName}',
  GetAll: Organizations.Projects.Get + '/maintainers',
  Create: Organizations.Projects.Get + '/maintainers',
  Delete: Organizations.Projects.Get + '/maintainers/{maintainerUserName}',
};
Users.Projects.ProjectMaintainers = {
  Get: Users.Projects.Get + '/maintainers/{adminUserName}',
  GetAll: Users.Projects.Get + '/maintainers',
  Create: Users.Projects.Get + '/maintainers',
  Delete: Users.Projects.Get + '/maintainers/{adminUserName}',
};

Organizations.Projects.ProjectOwners = {
  Get: Organizations.Projects.Get + '/owner',
  Create: Organizations.Projects.Get + '/owner',
};
Users.Projects.ProjectOwners = {
  Get: Users.Projects.Get + '/owner',
  Create: Users.Projects.Get + '/owner',
};

export const ApiRoutesV1 = {
  Base,
  Users,
  Organizations,
  Projects,
  IssuePosts,
  Identity,
  Notifications,
  AntiForgery,
};
