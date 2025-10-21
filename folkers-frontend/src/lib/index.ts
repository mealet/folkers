export const WATCHER_ROLE = "watcher";
export const EDITOR_ROLE = "editor";
export const ADMIN_ROLE = "admin";

export const rolesOrder: Record<string, number> = {
	[WATCHER_ROLE]: 0,
	[EDITOR_ROLE]: 1,
	[ADMIN_ROLE]: 2
};

export const selectableRoles = [
	{
		id: WATCHER_ROLE,
		label: "Читатель"
	},
	{
		id: EDITOR_ROLE,
		label: "Редактор"
	},
	{
		id: ADMIN_ROLE,
		label: "Администратор"
	}
];

export const ACCEPTABLE_MEDIA_TYPES = "image/jpeg, image/png, image/gif, image/webp";
