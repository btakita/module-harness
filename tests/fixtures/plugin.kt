/**
 * # Module: PythonIntentionsPlugin
 *
 * ## Spec
 * - registerIntentions(): registers all intention actions with IDEA platform
 * - IntentionAction.invoke(): transforms code at caret position
 *
 * ## Agentic Contracts
 * - No ActionPromoter registered (breaks intention chain in .md files)
 * - Self-disabling via update() when HighlightInfo exists at caret
 *
 * ## Evals
 * - hook_coverage [boolean]: all extension points have registered listeners
 * - hook_excess [boolean]: no unused/dead extension registrations
 * - hook_count [range: 3..8]: number of registered hooks in optimal band
 * - interface_complexity [ordinal: 1..5]: public API surface complexity rating
 * - interface_depth [boolean]: max inheritance depth <= 3
 * - intention_chain_safety [boolean]: ActionPromoter absent from plugin.xml
 * - hot_reload_compat [continuous]: ratio of classes safe for hot reload
 */

package com.example.intentions

class PythonIntentionsPlugin {
    fun registerIntentions() {}
}
