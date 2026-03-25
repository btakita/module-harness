/**
 * # Module: api
 *
 * ## Spec
 * - Handle GET requests for resources
 *
 * ## Agentic Contracts
 * - Always returns JSON responses
 *
 * ## Evals
 * - get_success: valid endpoint → 200 with JSON body
 */

export function handler(req: Request): Response {
  return new Response(JSON.stringify({ ok: true }));
}
