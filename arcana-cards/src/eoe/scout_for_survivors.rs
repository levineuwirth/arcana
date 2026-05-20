//! Scout for Survivors — `{2}{W}` sorcery. "Return up to three target
//! creature cards with total mana value 3 or less from your graveyard
//! to the battlefield. Put a +1/+1 counter on each of them."
//!
//! GAP: 'total mana value 3 or less' is an aggregate constraint
//! across multiple targets — TargetFilter::Card supports per-card
//! cmc cap, not a sum. We use a per-card cap of 3 and accept the
//! over-permissiveness.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scout for Survivors");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return up to three target creature cards with total mana value 3 or less from your graveyard to the battlefield. Put a +1/+1 counter on each of them.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().with_max_cmc(3),
                },
                count: TargetCount::UpTo(3),
                controller: None,
            }],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for t in entry.targets.targets.iter() {
        if let TargetChoice::Object(id) = t {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
        }
    }
    // GAP: 'total mv 3 or less' aggregate — per-card cap used instead.
    effects
}
