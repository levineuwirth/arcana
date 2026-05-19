//! It Doesn't Add Up — `{3}{B}{B}` instant. "Return target creature card from
//! your graveyard to the battlefield. Suspect it. (It has menace and can't block.)"
//!
//! # GAP: Suspect keyword/status is not in the engine catalog. The reanimate
//! effect is implemented; the Suspect clause is noted as a gap.

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
    let name = reg.interner_mut().intern("It Doesn't Add Up");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature card from your graveyard to the battlefield. Suspect it.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card { zone: Zone::Graveyard(0), filter: ObjectFilter::creature() },
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
    // GAP: Suspect status/keyword is not expressible via the catalog
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
