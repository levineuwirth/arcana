//! Flame Wave — `{3}{R}{R}{R}{R}` sorcery. "Flame Wave deals 4 damage to
//! target player or planeswalker and each creature that player or that
//! planeswalker's controller controls."
//
// GAP: "each creature that target player controls" requires enumerating
// creatures filtered by controller (a specific player), which ForEach does
// not support directly (no controller filter). The player/planeswalker damage
// is expressible; the per-creature sweep on that player is not.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flame Wave");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Flame Wave deals 4 damage to target player or planeswalker and each creature that player or that planeswalker's controller controls.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::PLANESWALKER)
                        )
                    ),
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
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        _ => return Vec::new(),
    };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: dt,
            amount: 4,
        },
        // GAP: deal 4 damage to each creature the target player/planeswalker's controller controls not expressible
    ]
}
