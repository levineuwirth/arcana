//! Keyword glossary — the single source of truth for human-readable reminder
//! text, shared by the in-game card zoom and (later) the deckbuilder + any
//! RL feature naming. Keyed by a keyword's stable base name (parameter-free), so
//! `Ward({2})`, `Toxic(1)`, `Landwalk(Forest)` all resolve to "Ward" / "Toxic" /
//! "Landwalk". Missing keys degrade gracefully (chip with no reminder).

use crate::effects::KeywordAbility;

/// A keyword's stable lookup key: its base variant name, parameters stripped.
/// Derived from the `Debug` form so new variants need no match-arm maintenance
/// (`Ward(ManaCost{..})` -> "Ward", `Flying` -> "Flying").
pub fn keyword_key(kw: &KeywordAbility) -> String {
    let dbg = format!("{kw:?}");
    match dbg.split_once('(') {
        Some((base, _)) => base.to_string(),
        None => dbg,
    }
}

/// `(base name, reminder text)` for every keyword the engine knows. CR-style,
/// kept terse. Unknown keys simply have no entry (the chip shows without a
/// tooltip). This is the authoritative table; UIs fetch it once.
pub fn keyword_glossary() -> Vec<(&'static str, &'static str)> {
    vec![
        // Evergreen
        ("Flying", "Can only be blocked by creatures with flying or reach."),
        ("FirstStrike", "Deals combat damage before creatures without first strike."),
        ("DoubleStrike", "Deals both first-strike and regular combat damage."),
        ("Deathtouch", "Any nonzero amount of damage it deals to a creature is lethal."),
        ("Haste", "Can attack and use {T} abilities the turn it comes under your control."),
        ("Hexproof", "Can't be the target of spells or abilities your opponents control."),
        ("Shroud", "Can't be the target of any spells or abilities."),
        ("Indestructible", "Can't be destroyed by damage or by \"destroy\" effects."),
        ("Lifelink", "Damage it deals also causes its controller to gain that much life."),
        ("Menace", "Can only be blocked by two or more creatures."),
        ("Reach", "Can block creatures with flying."),
        ("Trample", "Combat damage beyond what's lethal to its blockers is dealt to the defender."),
        ("Vigilance", "Attacking doesn't cause it to tap."),
        ("Ward", "When it becomes the target of an opponent's spell or ability, counter that unless they pay the ward cost."),
        ("Flash", "You may cast it any time you could cast an instant."),
        ("Defender", "Can't attack."),
        // Evasion (per-pairing)
        ("Landwalk", "Can't be blocked if the defending player controls a land of the named type."),
        ("Fear", "Can only be blocked by artifact and/or black creatures."),
        ("Intimidate", "Can only be blocked by artifact creatures and/or creatures sharing a color with it."),
        ("Shadow", "Can only block or be blocked by creatures with shadow."),
        ("Horsemanship", "Can only be blocked by creatures with horsemanship."),
        ("Skulk", "Can't be blocked by creatures with greater power."),
        ("Flanking", "When a creature without flanking blocks it, that blocker gets -1/-1 until end of turn."),
        // Combat / counters
        ("Bushido", "When it blocks or becomes blocked, it gets +N/+N until end of turn."),
        ("Rampage", "When it becomes blocked, it gets +N/+N for each blocker beyond the first."),
        ("Exalted", "Whenever a creature you control attacks alone, it gets +1/+1 until end of turn."),
        ("BattleCry", "When it attacks, each other attacking creature gets +1/+0 until end of turn."),
        ("Mentor", "When it attacks, put a +1/+1 counter on a target attacking creature with lesser power."),
        ("Dethrone", "When it attacks the player with the most life, put a +1/+1 counter on it."),
        ("Renown", "When it deals combat damage to a player, if not renowned, put N +1/+1 counters on it and it becomes renowned."),
        ("Provoke", "When it attacks, you may make a target creature block it this turn if able."),
        ("Enlist", "As it attacks, you may tap a non-attacking creature you control to add its power to this creature."),
        ("Evolve", "When a creature with greater power or toughness enters under your control, put a +1/+1 counter on this."),
        ("Modular", "Enters with N +1/+1 counters; when it dies, move them to a target artifact creature."),
        ("Graft", "Enters with N +1/+1 counters; when another creature enters, you may move a counter onto it."),
        ("Bloodthirst", "If an opponent was dealt damage this turn, it enters with N +1/+1 counters."),
        ("Sunburst", "Enters with a counter for each color of mana spent to cast it."),
        ("Devour", "As it enters, you may sacrifice creatures; it enters with N +1/+1 counters per sacrifice."),
        ("Amplify", "As it enters, reveal cards sharing its type from hand to add +1/+1 counters."),
        ("Unleash", "May enter with a +1/+1 counter; if it has one, it can't block."),
        ("Riot", "Enters with your choice of a +1/+1 counter or haste."),
        ("Mutate", "Damage it deals to creatures is dealt as -1/-1 counters."),
        // Death / recursion
        ("Undying", "When it dies, if it had no +1/+1 counters, return it to the battlefield with one."),
        ("Persist", "When it dies, if it had no -1/-1 counters, return it to the battlefield with one."),
        ("Afterlife", "When it dies, create N 1/1 white and black Spirit creature tokens with flying."),
        ("Soulshift", "When it dies, you may return a Spirit card with mana value N or less from your graveyard to your hand."),
        ("Scavenge", "Exile it from your graveyard and pay the cost to put +1/+1 counters on a creature (sorcery speed)."),
        // Damage reshaping
        ("Wither", "Deals damage to creatures as -1/-1 counters."),
        ("Infect", "Deals damage to creatures as -1/-1 counters and to players as poison counters."),
        ("Toxic", "When it deals combat damage to a player, that player also gets N poison counters."),
        // Spell / cost modifiers
        ("Prowess", "Whenever you cast a noncreature spell, it gets +1/+1 until end of turn."),
        ("Cascade", "When you cast it, exile cards until a cheaper nonland card, which you may cast for free."),
        ("Convoke", "You may tap creatures to help pay its cost (each pays {1} or one mana of its color)."),
        ("Improvise", "You may tap artifacts to help pay its generic cost."),
        ("Delve", "You may exile cards from your graveyard, each paying {1} of its cost."),
        ("Cycling", "Pay the cycling cost and discard it to draw a card."),
        ("Flashback", "You may cast it from your graveyard for its flashback cost, then it's exiled."),
        ("Madness", "If discarded, you may cast it for its madness cost instead of putting it in the graveyard."),
        ("Kicker", "You may pay an additional cost as you cast it for a bonus effect."),
        ("Storm", "When you cast it, copy it for each spell cast before it this turn."),
        ("Equip", "Pay the equip cost to attach this Equipment to a creature you control (sorcery speed)."),
        ("Warp", "You may cast it for its warp cost; it's then exiled and returns to the battlefield on a later turn."),
        ("Changeling", "Is every creature type at all times."),
        ("Banding", "Helps attack/block as a group; its controller assigns its combat damage."),
        ("Fading", "Enters with N fade counters; remove one each upkeep, and sacrifice it when you can't."),
        ("Vanishing", "Enters with N time counters; remove one each upkeep, and sacrifice it when the last is removed."),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_strips_parameters() {
        assert_eq!(keyword_key(&KeywordAbility::Flying), "Flying");
        assert_eq!(keyword_key(&KeywordAbility::Ward(crate::mana::ManaCost::parse("{2}").unwrap())), "Ward");
        assert_eq!(keyword_key(&KeywordAbility::Toxic(1)), "Toxic");
    }

    #[test]
    fn glossary_keys_are_unique_and_nonempty() {
        let g = keyword_glossary();
        let mut keys: Vec<&str> = g.iter().map(|(k, _)| *k).collect();
        keys.sort_unstable();
        let n = keys.len();
        keys.dedup();
        assert_eq!(keys.len(), n, "glossary keys must be unique");
        assert!(g.iter().all(|(k, v)| !k.is_empty() && !v.is_empty()));
        // The evergreen keywords are covered.
        for ev in ["Flying", "Trample", "Deathtouch", "Ward", "Vigilance", "Lifelink"] {
            assert!(g.iter().any(|(k, _)| *k == ev), "{ev} should be in the glossary");
        }
    }
}
