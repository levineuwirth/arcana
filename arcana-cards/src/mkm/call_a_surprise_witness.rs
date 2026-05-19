//! Call a Surprise Witness — `{1}{W}` sorcery. "Return target creature card
//! with mana value 3 or less from your graveyard to the battlefield. That
//! creature enters with a flying counter on it and becomes a Spirit in
//! addition to its other types."
//!
//! GAP: mana value ≤ 3 filter on TargetFilter::Card not available; flying
//! counter placement not in AddCounters (no CounterKind::Flying); adding
//! Spirit subtype to an existing creature not expressible.
//! ReturnFromGraveyardToBattlefield is expressed with generic creature filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Call a Surprise Witness");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature card with mana value 3 or less from your graveyard to the battlefield. That creature enters with a flying counter on it and becomes a Spirit in addition to its other types.".into(),
                target_requirements: vec![TargetRequirement {
                    // GAP: mana value <= 3 filter not available
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
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
    // GAP: flying counter (no CounterKind::Flying) + adding Spirit subtype not expressible
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
