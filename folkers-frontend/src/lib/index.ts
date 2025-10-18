export const WATCHER_ROLE = 'watcher';
export const EDITOR_ROLE = 'editor';
export const ADMIN_ROLE = 'admin';

export const rolesOrder: Record<string, number> = {
  [WATCHER_ROLE]: 0,
  [EDITOR_ROLE]: 1,
  [ADMIN_ROLE]: 2
};
