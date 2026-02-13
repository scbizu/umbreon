import { parse as parseToml } from "@iarna/toml";
import { XMLParser } from "fast-xml-parser";

type Env = {
  GIST_URL?: string;
  FEED_TITLE?: string;
  FEED_SUBTITLE?: string;
  FEED_LINK?: string;
  MAX_ITEMS?: string;
  DAYS?: string;
};

type FeedConfig = {
  name?: string;
  url: string;
  tags?: string[];
};

type GistConfig = {
  feeds?: Record<string, FeedConfig>;
};

type NormalizedEntry = {
  id: string;
  title: string;
  link: string;
  summary?: string;
  updated: string;
  author?: string;
  sourceTitle?: string;
  sourceUrl?: string;
  tags?: string[];
};

const corsHeaders: Record<string, string> = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "GET, OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type",
};

const parser = new XMLParser({
  ignoreAttributes: false,
  attributeNamePrefix: "",
  textNodeName: "text",
  cdataPropName: "cdata",
  preserveOrder: false,
  removeNSPrefix: false,
});

function escapeXml(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/\"/g, "&quot;")
    .replace(/'/g, "&apos;");
}

function cdata(value: string): string {
  if (!value) return "";
  return `<![CDATA[${value.replace(/]]>/g, "]]]]><![CDATA[>")}]]>`;
}

function toArray<T>(value: T | T[] | undefined | null): T[] {
  if (!value) return [];
  return Array.isArray(value) ? value : [value];
}

function readText(value: any): string | undefined {
  if (value === null || value === undefined) return undefined;
  if (typeof value === "string" || typeof value === "number") {
    const text = String(value).trim();
    return text.length ? text : undefined;
  }
  if (Array.isArray(value)) {
    for (const item of value) {
      const text = readText(item);
      if (text) return text;
    }
    return undefined;
  }
  if (typeof value === "object") {
    if (typeof value.text === "string") return value.text.trim() || undefined;
    if (typeof value.cdata === "string") return value.cdata.trim() || undefined;
    if (typeof value["#text"] === "string") return value["#text"].trim() || undefined;
  }
  return undefined;
}

function readLink(value: any): string | undefined {
  if (!value) return undefined;
  if (typeof value === "string") return value.trim() || undefined;
  if (Array.isArray(value)) {
    const preferred = value.find((entry) => entry?.rel === "alternate") || value[0];
    return readLink(preferred);
  }
  if (typeof value === "object") {
    if (typeof value.href === "string") return value.href.trim() || undefined;
    if (typeof value.url === "string") return value.url.trim() || undefined;
    if (typeof value.text === "string") return value.text.trim() || undefined;
  }
  return undefined;
}

function parseDate(value?: string): Date | undefined {
  if (!value) return undefined;
  const trimmed = value.trim();
  if (!trimmed) return undefined;
  const parsed = Date.parse(trimmed);
  if (!Number.isNaN(parsed)) return new Date(parsed);
  const ymd = /^(\d{4})-(\d{2})-(\d{2})$/.exec(trimmed);
  if (ymd) {
    const date = new Date(`${ymd[1]}-${ymd[2]}-${ymd[3]}T00:00:00Z`);
    if (!Number.isNaN(date.getTime())) return date;
  }
  return undefined;
}

function toIso(value?: Date): string {
  return (value ?? new Date()).toISOString();
}

function normalizeRssFeed(feed: any, tags?: string[]): NormalizedEntry[] {
  const channel = feed?.rss?.channel ?? feed?.channel ?? feed?.rss?.["channel"];
  if (!channel) return [];
  const sourceTitle = readText(channel.title);
  const sourceUrl = readLink(channel.link);
  const items = toArray(channel.item);

  return items
    .map((item) => {
      const title = readText(item.title) ?? "";
      const link = readLink(item.link) ?? "";
      const guid = readText(item.guid) || link || title;
      const pubDate = readText(item.pubDate) || readText(item["dc:date"]);
      const updated = toIso(parseDate(pubDate));
      const summary =
        readText(item["content:encoded"]) || readText(item.description) || readText(item.summary);
      const author = readText(item.author) || readText(item["dc:creator"]);
      return {
        id: guid,
        title,
        link,
        summary,
        updated,
        author,
        sourceTitle,
        sourceUrl,
        tags,
      };
    })
    .filter((entry) => entry.link && entry.title);
}

function normalizeAtomFeed(feed: any, tags?: string[]): NormalizedEntry[] {
  const atom = feed?.feed ?? feed?.["atom:feed"] ?? feed;
  if (!atom) return [];
  const sourceTitle = readText(atom.title);
  const sourceUrl = readLink(atom.link);
  const entries = toArray(atom.entry);

  return entries
    .map((entry) => {
      const title = readText(entry.title) ?? "";
      const link = readLink(entry.link) ?? "";
      const id = readText(entry.id) || link || title;
      const updated =
        toIso(parseDate(readText(entry.updated) || readText(entry.published))) || toIso(new Date());
      const summary = readText(entry.content) || readText(entry.summary);
      const author = readText(entry.author?.name) || readText(entry.author);
      return {
        id,
        title,
        link,
        summary,
        updated,
        author,
        sourceTitle,
        sourceUrl,
        tags,
      };
    })
    .filter((entry) => entry.link && entry.title);
}

function normalizeFeed(xml: string, tags?: string[]): NormalizedEntry[] {
  let parsed: any;
  try {
    parsed = parser.parse(xml);
  } catch {
    return [];
  }
  if (parsed?.rss || parsed?.channel) return normalizeRssFeed(parsed, tags);
  if (parsed?.feed || parsed?.["atom:feed"]) return normalizeAtomFeed(parsed, tags);
  return [];
}

function renderAtom(entries: NormalizedEntry[], metadata: { title: string; subtitle: string; link: string }) {
  const updated = entries[0]?.updated ?? new Date().toISOString();
  const feedId = metadata.link || "urn:uuid:umbreon-aggregated-feed";
  const items = entries
    .map((entry) => {
      const tags = (entry.tags ?? []).map((tag) => `<category term="${escapeXml(tag)}" />`).join("");
      const summary = entry.summary ? `<summary type="html">${cdata(entry.summary)}</summary>` : "";
      const author = entry.author
        ? `<author><name>${escapeXml(entry.author)}</name></author>`
        : "";
      const source = entry.sourceTitle
        ? `<source><title>${escapeXml(entry.sourceTitle)}</title>${
            entry.sourceUrl ? `<link href="${escapeXml(entry.sourceUrl)}" />` : ""
          }</source>`
        : "";
      return [
        "<entry>",
        `<id>${escapeXml(entry.id)}</id>`,
        `<title>${escapeXml(entry.title)}</title>`,
        `<link href="${escapeXml(entry.link)}" />`,
        `<updated>${escapeXml(entry.updated)}</updated>`,
        summary,
        author,
        source,
        tags,
        "</entry>",
      ]
        .filter(Boolean)
        .join("");
    })
    .join("");

  return [
    "<?xml version=\"1.0\" encoding=\"utf-8\"?>",
    "<feed xmlns=\"http://www.w3.org/2005/Atom\">",
    `<id>${escapeXml(feedId)}</id>`,
    `<title>${escapeXml(metadata.title)}</title>`,
    metadata.subtitle ? `<subtitle>${escapeXml(metadata.subtitle)}</subtitle>` : "",
    metadata.link ? `<link href=\"${escapeXml(metadata.link)}\" rel=\"alternate\" />` : "",
    `<updated>${escapeXml(updated)}</updated>`,
    items,
    "</feed>",
  ]
    .filter(Boolean)
    .join("");
}

async function fetchWithTimeout(url: string, timeoutMs: number) {
  const controller = new AbortController();
  const id = setTimeout(() => controller.abort(), timeoutMs);
  try {
    return await fetch(url, { signal: controller.signal });
  } finally {
    clearTimeout(id);
  }
}

function clampNumber(value: string | undefined, fallback: number, min: number, max: number): number {
  if (!value) return fallback;
  const parsed = Number(value);
  if (!Number.isFinite(parsed)) return fallback;
  return Math.min(max, Math.max(min, parsed));
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    if (request.method === "OPTIONS") {
      return new Response(null, { status: 204, headers: corsHeaders });
    }

    if (request.method !== "GET") {
      return new Response("Method Not Allowed", { status: 405, headers: corsHeaders });
    }

    const url = new URL(request.url);
    const gistUrl = url.searchParams.get("gist") || env.GIST_URL;
    if (!gistUrl) {
      return new Response("Missing gist url", { status: 400, headers: corsHeaders });
    }

    const title = url.searchParams.get("title") || env.FEED_TITLE || "Umbreon Aggregated Feed";
    const subtitle = url.searchParams.get("subtitle") || env.FEED_SUBTITLE || "";
    const link = url.searchParams.get("link") || env.FEED_LINK || "";
    const maxItems = clampNumber(url.searchParams.get("limit") || env.MAX_ITEMS, 200, 1, 1000);
    const days = clampNumber(url.searchParams.get("days") || env.DAYS, 60, 1, 3650);
    const cutoff = Date.now() - days * 24 * 60 * 60 * 1000;

    let gistText: string;
    try {
      const gistResponse = await fetchWithTimeout(gistUrl, 10_000);
      if (!gistResponse.ok) {
        return new Response(`Failed to load gist: ${gistResponse.status}`, {
          status: 502,
          headers: corsHeaders,
        });
      }
      gistText = await gistResponse.text();
    } catch (err) {
      return new Response(`Failed to load gist: ${String(err)}`, {
        status: 502,
        headers: corsHeaders,
      });
    }

    let config: GistConfig;
    try {
      config = parseToml(gistText) as GistConfig;
    } catch (err) {
      return new Response(`Invalid gist toml: ${String(err)}`, {
        status: 400,
        headers: corsHeaders,
      });
    }

    const feeds = config.feeds ?? {};
    const entries: NormalizedEntry[] = [];

    await Promise.all(
      Object.entries(feeds).map(async ([key, feed]) => {
        if (!feed?.url || !feed.url.trim()) return;
        const tags = feed.tags ?? [];
        try {
          const response = await fetchWithTimeout(feed.url, 12_000);
          if (!response.ok) return;
          const xml = await response.text();
          const normalized = normalizeFeed(xml, tags).map((entry) => {
            const sourceTitle = entry.sourceTitle ?? feed.name ?? key;
            return {
              ...entry,
              sourceTitle,
              author: entry.author ?? sourceTitle,
            };
          });
          entries.push(...normalized);
        } catch {
          return;
        }
      })
    );

    const filtered = entries
      .map((entry) => ({
        ...entry,
        updatedMs: Date.parse(entry.updated),
      }))
      .filter((entry) => !Number.isNaN(entry.updatedMs) && entry.updatedMs >= cutoff)
      .sort((a, b) => b.updatedMs - a.updatedMs)
      .slice(0, maxItems)
      .map(({ updatedMs, ...entry }) => entry);

    const body = renderAtom(filtered, { title, subtitle, link });
    return new Response(body, {
      status: 200,
      headers: {
        ...corsHeaders,
        "Content-Type": "application/atom+xml; charset=utf-8",
        "Cache-Control": "public, max-age=300",
      },
    });
  },
};
