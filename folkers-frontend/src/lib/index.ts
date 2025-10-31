import { compile } from "mdsvex";
import type { Plugin } from "unified";

import remarkBreaks from "remark-breaks";

import rehypeExternalLinks from "rehype-external-links";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import rehypeSlug from "rehype-slug";

export const WATCHER_ROLE = "watcher";
export const EDITOR_ROLE = "editor";
export const ADMIN_ROLE = "admin";

export async function renderMarkdown(text: string): Promise<string> {
	const compiled = await compile(text, {
		remarkPlugins: [remarkBreaks as unknown as Plugin],
		rehypePlugins: [
			rehypeExternalLinks as unknown as Plugin,
			rehypeSlug as unknown as Plugin,
			rehypeAutolinkHeadings as unknown as Plugin
		]
	});

	if (!compiled) return "";
	return compiled.code;
}

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
