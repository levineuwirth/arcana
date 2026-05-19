//! Scout for Survivors — `{2}{W}` sorcery, "Return up to three target creature
//! cards with total mana value 3 or less from your graveyard to the
//! battlefield. Put a +1/+1 counter on each of them."
//! GAP: total CMC constraint across multiple targets (hat filter) not
//! expressible; individual per-target filter applied as best effort.

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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
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

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    entry.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t { Some(*id) } else { None }
    }).flat_map(|id| {
        [
            Effect::ReturnFromGraveyardToBattlefield { target: id },
            Effect::AddCounters { target: id, kind: CounterKind::PlusOnePlusOne, count: 1 },
        ]
    }).collect()
}
