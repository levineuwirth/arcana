//! Surge to Victory — `{4}{R}{R}` sorcery. "Exile target instant or
//! sorcery card from your graveyard. Creatures you control get +X/+0
//! until end of turn, where X is that card's mana value. Whenever a
//! creature you control deals combat damage to a player this turn, copy
//! the exiled card. You may cast the copy without paying its mana cost."
//!
//! The exile of the targeted instant/sorcery card from the graveyard is
//! expressible via `Effect::ExileFromGraveyard`. The remaining clauses
//! are GAP material:
//! - "+X/+0 until end of turn, where X is that card's mana value" — X is
//!   dynamic (the exiled card's mana value), but `script::` exposes no
//!   helper for a specific object's mana value, and a team-wide pump (a
//!   continuous effect over all creatures you control) is not a single
//!   `Effect` primitive. Emitting a fixed pump would be a wrong card.
//! - The delayed copy-on-combat-damage trigger + "cast the copy without
//!   paying its mana cost" has no Effect primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surge to Victory");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile target instant or sorcery card from your graveyard. Creatures you control get +X/+0 until end of turn, where X is that card's mana value. Whenever a creature you control deals combat damage to a player this turn, copy the exiled card. You may cast the copy without paying its mana cost.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: team-wide "+X/+0 where X is the exiled card's mana value" (no
    // per-object mana-value script helper, no team-pump primitive) and the
    // copy-on-combat-damage delayed trigger that casts the copy for free.
    vec![Effect::ExileFromGraveyard { target: *id }]
}
