//! Recommission — `{1}{W}` sorcery. "Return target artifact or creature card
//! with mana value 3 or less from your graveyard to the battlefield. If a
//! creature enters this way, it enters with an additional +1/+1 counter on
//! it."
//!
//! GAP: TargetFilter::Card supports a graveyard filter but is limited; we use
//! it with creature() and max_cmc(3). The 'enters with +1/+1 counter' rider
//! can't be expressed as a conditional on entry — we add the counter after
//! reanimating as best effort. Artifact-card branch and 'artifact OR creature
//! card' targeting is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Recommission");
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
                text: "Return target artifact or creature card with mana value 3 or less from your graveyard to the battlefield. If a creature enters this way, it enters with an additional +1/+1 counter on it.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature().with_max_cmc(3),
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
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
