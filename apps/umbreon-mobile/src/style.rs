pub const BASE_STYLES: &str = r#"
@import url("https://fonts.googleapis.com/icon?family=Material+Icons");

:root {
  font-family: "Roboto", "Inter", system-ui, -apple-system, sans-serif;
  letter-spacing: 0.2px;
}

.umbreon-shell {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--md-sys-color-background);
  color: var(--md-sys-color-on-background);
  overflow: hidden;
  position: relative;
}

.umbreon-header {
  padding: 16px 18px 10px;
  font-size: 20px;
  font-weight: 600;
  text-align: center;
  background: var(--md-sys-color-background);
  position: sticky;
  top: 0;
  z-index: 10;
}

.umbreon-shell.theme-dark {
  --md-sys-color-background: #101418;
  --md-sys-color-surface: #171b20;
  --md-sys-color-surface-container: #1f242a;
  --md-sys-color-surface-container-high: #252a31;
  --md-sys-color-outline: #3b414a;
  --md-sys-color-outline-variant: #2b3138;
  --md-sys-color-primary: #9ec3ff;
  --md-sys-color-on-primary: #0f1d35;
  --md-sys-color-secondary: #c7d2e4;
  --md-sys-color-on-surface: #e2e6ed;
  --md-sys-color-on-surface-variant: #b6bdc8;
}

.umbreon-shell.theme-light {
  --md-sys-color-background: #f2f2f7;
  --md-sys-color-surface: #ffffff;
  --md-sys-color-surface-container: #eef0f4;
  --md-sys-color-surface-container-high: #e7ebf3;
  --md-sys-color-outline: #d7dbe4;
  --md-sys-color-outline-variant: #e6e9f0;
  --md-sys-color-primary: #2b63ff;
  --md-sys-color-on-primary: #ffffff;
  --md-sys-color-secondary: #657189;
  --md-sys-color-on-surface: #1c1c1e;
  --md-sys-color-on-surface-variant: #6b7280;
}

.umbreon-sidebar {
  width: 252px;
  padding: 24px 16px;
  background: var(--md-sys-color-surface);
  border-right: 1px solid var(--md-sys-color-outline-variant);
  display: flex;
  flex-direction: column;
  gap: 20px;
  transition: width 0.2s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  flex: 0 0 auto;
  height: 100vh;
  max-height: 100vh;
  overflow: hidden;
}

.umbreon-sidebar.collapsed {
  width: 72px;
  padding: 20px 8px;
}

.umbreon-brand-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.umbreon-brand {
  font-size: 20px;
  font-weight: 600;
}

.umbreon-sidebar.collapsed .umbreon-brand-text,
.umbreon-sidebar.collapsed .nav-label,
.umbreon-sidebar.collapsed .theme-toggle-text {
  display: none;
}

.collapse-toggle {
  border: none;
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  border-radius: 12px;
  width: 40px;
  height: 40px;
  cursor: pointer;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12);
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.collapse-toggle .material-icons {
  font-size: 20px;
  line-height: 1;
}

.umbreon-nav {
  display: flex;
  flex-direction: column;
  gap: 10px;
  flex: 1;
  overflow: auto;
}

.nav-btn {
  padding: 12px 16px;
  border-radius: 14px;
  border: 1px solid transparent;
  background: transparent;
  color: var(--md-sys-color-on-surface);
  text-align: left;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  transition: background 0.2s ease, border-color 0.2s ease, color 0.2s ease;
}

.nav-icon {
  width: 22px;
  text-align: center;
  font-size: 16px;
  line-height: 1;
}

.nav-icon.material-icons {
  font-size: 20px;
}

.nav-btn.active {
  background: var(--md-sys-color-surface-container-high);
  border-color: var(--md-sys-color-outline-variant);
  color: var(--md-sys-color-primary);
}

.nav-btn:hover {
  background: var(--md-sys-color-surface-container);
}

.sidebar-footer {
  margin-top: auto;
  margin-bottom: 32px;
  display: flex;
}

.sidebar-footer .nav-btn {
  width: 100%;
}

.theme-toggle {
  padding: 12px 16px;
  border-radius: 14px;
  border: 1px solid var(--md-sys-color-outline-variant);
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
}

.theme-icon {
  font-size: 18px;
}

.theme-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
}

.theme-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.theme-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--md-sys-color-outline-variant);
  transition: 0.2s;
  border-radius: 999px;
}

.theme-slider::before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: var(--md-sys-color-on-primary);
  transition: 0.2s;
  border-radius: 50%;
}

.theme-switch input:checked + .theme-slider {
  background-color: var(--md-sys-color-primary);
}

.theme-switch input:checked + .theme-slider::before {
  transform: translateX(20px);
}

.umbreon-content {
  display: flex;
  flex: 1;
  min-height: 0;
  overflow: hidden;
  padding-bottom: 84px;
}

.bottom-nav {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--md-sys-color-surface);
  border-top: 1px solid var(--md-sys-color-outline-variant);
  display: flex;
  align-items: center;
  justify-content: space-around;
  padding: 8px 12px 18px;
  gap: 8px;
  z-index: 20;
  box-shadow: 0 -8px 20px rgba(0, 0, 0, 0.08);
  backdrop-filter: blur(18px);
}

.umbreon-shell.theme-dark .bottom-nav {
  background: rgba(23, 27, 32, 0.95);
}

.bottom-nav-btn {
  border: none;
  background: transparent;
  color: var(--md-sys-color-on-surface-variant);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  padding: 8px 12px;
  border-radius: 14px;
  min-width: 64px;
  cursor: pointer;
  transition: background 0.2s ease, color 0.2s ease;
}

.bottom-nav-btn .material-icons {
  font-size: 22px;
}

.bottom-nav-btn.active {
  background: rgba(43, 99, 255, 0.14);
  color: var(--md-sys-color-primary);
}

.umbreon-body {
  flex: 1;
  overflow: hidden;
  background: var(--md-sys-color-background);
}

.timeline-pane {
  flex: 1;
  overflow: auto;
  overflow-x: hidden;
  padding: 24px 18px 18px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  background: var(--md-sys-color-background);
}

.settings-pane {
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  flex: 1;
  overflow: auto;
  background: var(--md-sys-color-background);
}

.settings-pane h2 {
  margin: 0;
  font-size: 22px;
}

.settings-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.settings-row {
  display: flex;
  gap: 10px;
  align-items: center;
}

.settings-row-spread {
  justify-content: space-between;
}

.settings-label {
  font-size: 14px;
  color: var(--md-sys-color-on-surface-variant);
}

.settings-input {
  border-radius: 12px;
  padding: 10px 12px;
  border: 1px solid var(--md-sys-color-outline-variant);
  background: var(--md-sys-color-surface);
  color: var(--md-sys-color-on-surface);
  flex: 1;
}

.settings-select {
  border-radius: 12px;
  padding: 10px 12px;
  border: 1px solid var(--md-sys-color-outline-variant);
  background: var(--md-sys-color-surface);
  color: var(--md-sys-color-on-surface);
  min-height: 40px;
  height: 40px;
  line-height: 40px;
  flex: 1;
}

.settings-input:focus {
  outline: 2px solid var(--md-sys-color-primary);
}

.settings-select:focus {
  outline: 2px solid var(--md-sys-color-primary);
}

.settings-sync {
  padding: 10px 12px;
  border: none;
  border-radius: 12px;
  background: var(--md-sys-color-primary);
  color: var(--md-sys-color-on-primary);
  display: inline-flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.settings-sync-icon {
  width: 40px;
  height: 40px;
  justify-content: center;
  padding: 0;
}

.settings-sync:hover {
  opacity: 0.92;
}

.settings-actions {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.settings-action {
  padding: 10px 12px;
  border: none;
  border-radius: 12px;
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  display: inline-flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.settings-action-icon {
  width: 40px;
  height: 40px;
  justify-content: center;
  padding: 0;
}

.settings-action:hover {
  background: var(--md-sys-color-surface-container-high);
}

.settings-action.is-loading .material-icons,
.settings-action-icon.is-loading .material-icons,
.settings-sync.is-loading .material-icons {
  animation: spin 1s linear infinite;
}

.settings-action:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.settings-hint {
  margin: 0;
  font-size: 12px;
  color: var(--md-sys-color-on-surface-variant);
}

.settings-status {
  color: var(--md-sys-color-secondary);
  font-size: 13px;
  margin: 0;
}

.settings-theme-toggle {
  justify-content: flex-start;
}

.toast {
  position: absolute;
  left: 50%;
  bottom: 72px;
  transform: translateX(-50%);
  padding: 12px 14px;
  border-radius: 14px;
  display: inline-flex;
  align-items: center;
  gap: 10px;
  box-shadow: 0 10px 24px rgba(0, 0, 0, 0.16);
  max-width: min(90vw, 420px);
  z-index: 5;
}

.toast-success {
  background: var(--md-sys-color-tertiary-container);
  color: var(--md-sys-color-on-tertiary-container);
}

.toast-error {
  background: var(--md-sys-color-error-container);
  color: var(--md-sys-color-on-error-container);
}

.toast-close {
  border: none;
  background: transparent;
  color: inherit;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
}

.explore-pane {
  padding: 18px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  flex: 1;
  overflow: auto;
}

.explore-subheader {
  display: flex;
  align-items: center;
  gap: 12px;
}

.explore-back {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  border: none;
  background: transparent;
  color: var(--md-sys-color-on-surface);
  cursor: pointer;
  font-size: 14px;
}

.explore-sync {
  margin-left: auto;
  width: 36px;
  height: 36px;
  border-radius: 10px;
  border: none;
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface-variant);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.explore-sync:hover {
  background: var(--md-sys-color-surface-container-high);
}

.explore-sync.is-loading {
  color: var(--md-sys-color-primary);
}

.explore-sync.is-loading .material-icons {
  animation: spin 1s linear infinite;
}

.explore-card {
  background: var(--md-sys-color-surface);
  border-radius: 22px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  box-shadow: 0 10px 24px rgba(0, 0, 0, 0.08);
}

.explore-item {
  border: none;
  background: transparent;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 12px;
  border-radius: 16px;
  cursor: pointer;
  font-size: 16px;
  color: var(--md-sys-color-on-surface);
}

.explore-item:hover {
  background: var(--md-sys-color-surface-container);
}

.explore-icon {
  width: 44px;
  height: 44px;
  border-radius: 14px;
  background: var(--md-sys-color-surface-container);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--md-sys-color-on-surface-variant);
  font-size: 22px;
  line-height: 44px;
}

.explore-icon.material-icons {
  line-height: 44px;
}

.explore-label {
  font-weight: 600;
}

.explore-chevron {
  margin-left: auto;
  color: var(--md-sys-color-on-surface-variant);
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.timeline-pane > * {
  flex-shrink: 0;
}

.feed-card {
  position: relative;
  display: flex;
  align-items: flex-start;
  gap: 14px;
  padding: 18px 16px;
  border-radius: 18px;
  border: 1px solid var(--md-sys-color-outline-variant);
  background-color: var(--md-sys-color-surface);
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  width: 100%;
  max-height: 25vh;
  overflow: hidden;
  box-sizing: border-box;
  box-shadow: 0 12px 28px rgba(0, 0, 0, 0.08);
}

.feed-summary-indicator {
  position: absolute;
  top: 12px;
  right: 12px;
  font-size: 18px;
  color: var(--md-sys-color-on-surface-variant);
  background: var(--md-sys-color-surface-container);
  border-radius: 999px;
  padding: 4px;
  box-shadow: 0 6px 14px rgba(0, 0, 0, 0.1);
}

.feed-card--summarized .feed-lang-badge {
  margin-right: 36px;
}

.feed-card .post-body {
  max-height: calc(25vh - 36px);
  overflow: hidden;
}

.feed-card .post-text {
  max-height: calc(25vh - 140px);
  overflow: hidden;
}

.feed-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 16px 32px rgba(0, 0, 0, 0.12);
}

.feed-card--marked {
  border-color: #f4a6c5;
}

.post-avatar {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  background: var(--md-sys-color-surface-container-high);
  color: var(--md-sys-color-on-surface);
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  overflow: hidden;
}

.post-avatar-fallback {
  font-size: 18px;
}

.post-avatar-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.post-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 6px;
  min-width: 0;
  max-width: 100%;
  width: 100%;
}

.post-header {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  flex-wrap: wrap;
}

.feed-lang-badge {
  margin-left: auto;
  margin-right: 0;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 999px;
  background: rgba(158, 195, 255, 0.18);
  color: var(--md-sys-color-primary);
  font-size: 12px;
  font-weight: 700;
}

.feed-lang-icon {
  width: 14px;
  height: 14px;
}

.feed-lang-label {
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.post-name {
  font-weight: 600;
}

.post-handle,
.post-time,
.post-dot {
  color: var(--md-sys-color-on-surface-variant);
}

.post-text {
  color: var(--md-sys-color-on-surface);
  line-height: 1.6;
  max-width: 100%;
  max-height: calc(1.6em * 30);
  overflow: hidden;
  word-break: break-word;
  overflow-wrap: anywhere;
  display: block;
  position: relative;
}

.post-text::after {
  display: none;
}

.post-text * {
  max-width: 100%;
}

.post-text p {
  margin: 0 0 8px;
}

.post-text a {
  word-break: break-all;
}

.post-text pre {
  margin: 8px 0;
  padding: 10px 12px;
  border-radius: 10px;
  background: var(--md-sys-color-surface-container);
  overflow-x: hidden;
  white-space: pre-wrap;
  font-size: 12px;
}

.post-title {
  margin: 0;
  color: var(--md-sys-color-on-surface-variant);
}

.post-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 12px;
  color: var(--md-sys-color-secondary);
}

.post-link {
  color: var(--md-sys-color-primary);
}

.post-actions {
  display: flex;
  gap: 12px;
  margin-top: 6px;
}

.post-action {
  border: none;
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  cursor: pointer;
  font-size: 14px;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 12px;
  text-decoration: none;
}

.post-action:hover {
  background: var(--md-sys-color-surface-container-high);
}

.post-action--marked {
  background: #f7c5d9;
  color: #a4004c;
}

.post-action--marked:hover {
  background: #f4b6cf;
}

.material-icons {
  font-family: 'Material Icons';
  font-weight: normal;
  font-style: normal;
  font-size: 18px;
  line-height: 1;
  display: inline-block;
}

.feed-modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(15, 20, 28, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  z-index: 1000;
  overflow: auto;
}

.feed-modal {
  width: min(720px, 92vw);
  max-height: calc(100vh - 48px);
  background: var(--md-sys-color-surface);
  border-radius: 20px;
  padding: 24px;
  border: 1px solid var(--md-sys-color-outline-variant);
  box-shadow: 0 16px 40px rgba(0, 0, 0, 0.25);
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow: auto;
}

.feed-modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.feed-modal-close {
  border: none;
  background: var(--md-sys-color-surface-container);
  color: var(--md-sys-color-on-surface);
  border-radius: 12px;
  padding: 8px 12px;
  cursor: pointer;
}

.feed-modal-summary {
  margin: 0;
  color: var(--md-sys-color-on-surface-variant);
  line-height: 1.6;
}

.feed-modal-meta {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 13px;
  color: var(--md-sys-color-on-surface-variant);
}

.feed-modal-link {
  color: var(--md-sys-color-primary);
}

.empty-state {
  color: var(--md-sys-color-on-surface-variant);
}

.timeline-footer {
  padding: 8px 0 4px;
  font-size: 12px;
  color: var(--md-sys-color-on-surface-variant);
  text-align: center;
}

.timeline-spacer {
  height: 0;
}
"#;
