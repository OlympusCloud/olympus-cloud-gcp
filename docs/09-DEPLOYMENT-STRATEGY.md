# 🚀 Production Deployment & Rollback Strategy

*Last Updated: Current Session*

This document outlines the strategy for deploying, managing, and rolling back infrastructure and application changes for the Olympus Cloud platform.

## 1. Environment Promotion Path

We use a standard Git-flow-based promotion path with three distinct environments, each with its own GCP project and isolated state.

1.  **`dev` (Development)**
    *   **Trigger**: Automatic on every push/merge to the `main` branch.
    *   **Purpose**: Continuous integration and basic smoke testing. Provides developers with a live environment that reflects the latest code.
    *   **Data**: Ephemeral. Can be reset at any time.

2.  **`staging`**
    *   **Trigger**: Manual via `workflow_dispatch` in GitHub Actions.
    *   **Purpose**: Pre-production validation, QA testing, and integration testing. It should be a mirror of the production environment's configuration.
    *   **Data**: Near-production data, can be restored from a sanitized production backup.

3.  **`prod` (Production)**
    *   **Trigger**: Manual via `workflow_dispatch` in GitHub Actions, protected by a GitHub Environment rule requiring approval.
    *   **Purpose**: The live customer-facing environment.
    *   **Data**: Live production data with Point-in-Time Recovery (PITR) and regular backups enabled.

## 2. CI/CD Deployment Workflow

The entire process is automated via the `.github/workflows/deploy-infrastructure.yml` GitHub Actions workflow.

### Key Jobs

-   **`validate`**: Runs on every push and pull request. It performs:
    -   `terraform validate`: Checks syntax.
    -   `tflint`: Lints for best practices.
    -   `tfsec`: Scans for security misconfigurations.
-   **`plan`**: Runs on pull requests against the `main` branch. It generates a Terraform plan for the `dev` environment to allow for peer review of proposed changes.
-   **`deploy`**: Runs automatically on merge to `main` (for `dev`) or manually via `workflow_dispatch` (for `staging` and `prod`). It initializes Terraform with the correct environment backend, plans the changes, and applies them.

### Deployment Steps

1.  A developer creates a pull request with infrastructure changes.
2.  The `validate` and `plan` jobs run automatically, providing feedback directly in the PR.
3.  After review and approval, the PR is merged into `main`.
4.  The `deploy` job triggers automatically, deploying the changes to the **`dev`** environment.
5.  To promote to `staging`, a user with appropriate permissions navigates to the "Actions" tab in GitHub, selects the "Terraform CI/CD" workflow, and runs it, choosing `staging` from the dropdown.
6.  After successful validation in `staging`, the same manual process is followed to deploy to **`prod`**, which may require an additional approval step configured in GitHub Environments.

## 3. Rollback Strategy

### Infrastructure Rollback (Terraform)

If a Terraform `apply` introduces an issue, the primary rollback method is to revert the problematic commit.

1.  **Identify the faulty commit** in the Git history.
2.  **Revert the commit**:
    ```bash
    git revert <commit_hash>
    ```
3.  **Push the revert commit** to the `main` branch.
4.  This will trigger the CI/CD pipeline, which will "undo" the changes by applying the previous state.

### Application Rollback (Cloud Run)

Cloud Run keeps previous revisions, allowing for near-instantaneous rollbacks at the application level. This is the fastest way to mitigate a bad application deployment.

1.  **Navigate** to the Cloud Run service in the GCP Console.
2.  Go to the **"Revisions"** tab.
3.  Select a previous known-good revision.
4.  Click **"Manage Traffic"** and direct 100% of traffic to the selected stable revision.

This can also be done via the `gcloud` CLI:
```bash
# List revisions to find the name of the last good one
gcloud run revisions list --service=olympus-api-prod --region=us-central1

# Shift traffic back to the previous revision
gcloud run services update-traffic olympus-api-prod \
  --to-revisions=PREVIOUS_REVISION_NAME=100 \
  --region=us-central1
```

### Database Rollback (Cloud SQL)

Database rollbacks are the most critical and complex. Our strategy focuses on prevention and recovery.

-   **Schema Changes**: All schema changes (migrations) must be **backward-compatible**. They should never be destructive (e.g., dropping columns). A separate PR should be used to remove the old code/columns once the new version is fully deployed and stable.
-   **Data Corruption/Loss**: For major data issues, we will use Cloud SQL's **Point-in-Time Recovery (PITR)** feature. This allows us to restore the database to its state at any given minute within the retention window (7 days for production). This is a manual recovery process and will involve downtime.

## 4. Emergency "Break Glass" Procedure

If the CI/CD pipeline is non-functional and an emergency fix is required:

1.  A senior engineer with "Project Owner" or "Editor" IAM roles can make direct changes in the GCP Console.
2.  All manual changes **must be documented** in an incident report.
3.  As soon as the incident is resolved, the manual changes **must be codified** in Terraform and run through the pipeline to bring the infrastructure-as-code state back in sync with reality.