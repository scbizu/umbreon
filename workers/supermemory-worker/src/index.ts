import { generateText } from "ai";
import { createOpenAI } from "@ai-sdk/openai";
import {
  withSupermemory,
  supermemoryTools,
  addMemoryTool,
  searchMemoriesTool,
} from "@supermemory/tools/ai-sdk";

type Env = {
  SUPERMEMORY_API_KEY: string;
  CF_AIG_TOKEN?: string;
  OPENAI_API_KEY?: string;
};

const ACCOUNT_ID = "6fd4697183fdd0026b116cb0b23d4719";
const GATEWAY_ID = "nace-ai-proxy";
const MODEL_ID = "gpt-5";

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "POST, GET, OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type",
};

export default {
  async fetch(request: Request, env: Env) {
    if (request.method === "OPTIONS") {
      return new Response(null, { status: 204, headers: corsHeaders });
    }

    if (request.method === "GET") {
      return Response.json(
        {
          ok: true,
          usage:
            "POST / { userId, message, mode?, conversationId?, addMemory?, action?, memory?, query? }",
          actions: "addMemory | searchMemories",
          mode: "profile | query | full",
          addMemory: "never | always",
        },
        { headers: corsHeaders }
      );
    }

    if (request.method !== "POST") {
      return new Response("Method Not Allowed", {
        status: 405,
        headers: corsHeaders,
      });
    }

    let body: any;
    try {
      body = await request.json();
    } catch {
      return Response.json(
        { error: "Invalid JSON body" },
        { status: 400, headers: corsHeaders }
      );
    }

    const {
      userId,
      message,
      mode = "profile",
      conversationId,
      addMemory,
      action,
      memory,
      query,
      limit,
      includeFullDocs,
      toolChoice,
    } = body ?? {};

    if (!userId) {
      return Response.json(
        { error: "userId is required" },
        { status: 400, headers: corsHeaders }
      );
    }

    if (!env.SUPERMEMORY_API_KEY) {
      return Response.json(
        { error: "SUPERMEMORY_API_KEY is not configured" },
        { status: 500, headers: corsHeaders }
      );
    }

    const containerTags = [String(userId)];

    if (action === "addMemory") {
      if (!memory) {
        return Response.json(
          { error: "memory is required" },
          { status: 400, headers: corsHeaders }
        );
      }

      const addTool = addMemoryTool(env.SUPERMEMORY_API_KEY, { containerTags });
      const exec = addTool.execute;
      if (!exec) {
        return Response.json(
          { error: "addMemory tool execute is unavailable" },
          { status: 500, headers: corsHeaders }
        );
      }

      const result = await exec(
        { memory: String(memory) },
        { toolCallId: "manual-add", messages: [] }
      );

      return Response.json(result, { headers: corsHeaders });
    }

    if (action === "searchMemories") {
      if (!query) {
        return Response.json(
          { error: "query is required" },
          { status: 400, headers: corsHeaders }
        );
      }

      const searchTool = searchMemoriesTool(env.SUPERMEMORY_API_KEY, {
        containerTags,
      });

      const exec = searchTool.execute;
      if (!exec) {
        return Response.json(
          { error: "searchMemories tool execute is unavailable" },
          { status: 500, headers: corsHeaders }
        );
      }

      const result = await exec(
        {
          informationToGet: String(query),
          limit,
          includeFullDocs,
        },
        { toolCallId: "manual-search", messages: [] }
      );

      return Response.json(result, { headers: corsHeaders });
    }

    if (!message) {
      return Response.json(
        { error: "message is required" },
        { status: 400, headers: corsHeaders }
      );
    }

    const baseURL = `https://gateway.ai.cloudflare.com/v1/${ACCOUNT_ID}/${GATEWAY_ID}/openai`;

    const apiKey = env.OPENAI_API_KEY || env.CF_AIG_TOKEN;

    if (!apiKey) {
      return Response.json(
        { error: "OPENAI_API_KEY or CF_AIG_TOKEN is not configured" },
        { status: 500, headers: corsHeaders }
      );
    }

    const openai = createOpenAI({
      apiKey,
      baseURL,
      headers: env.CF_AIG_TOKEN
        ? { "cf-aig-authorization": `Bearer ${env.CF_AIG_TOKEN}` }
        : undefined,
    });

    const baseModel = openai(MODEL_ID);
    const model = withSupermemory(baseModel, String(userId), {
      mode,
      conversationId,
      addMemory,
      apiKey: env.SUPERMEMORY_API_KEY,
    });

    const tools = supermemoryTools(env.SUPERMEMORY_API_KEY, {
      containerTags,
    });

    const result = await generateText({
      model,
      messages: [{ role: "user", content: String(message) }],
      tools,
      toolChoice,
    });

    return Response.json({ text: result.text }, { headers: corsHeaders });
  },
};
