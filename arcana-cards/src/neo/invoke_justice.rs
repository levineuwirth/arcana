//! Invoke Justice — `{1}{W}{W}{W}{W}` sorcery. Return target permanent
//! card from your graveyard to the battlefield, then distribute four
//! +1/+1 counters among any number of creatures and/or Vehicles target
//! player controls.

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
    let name = reg.interner_mut().intern("Invoke Justice");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target permanent card from your graveyard to the battlefield, then distribute four +1/+1 counters among any number of creatures and/or Vehicles target player controls.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::permanent(),
                        },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::target_player(),
                ],
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
    let targets = &entry.targets.targets;
    let Some(TargetChoice::Object(card_id)) = targets.first() else { return Vec::new(); };
    let mut effects = vec![Effect::ReturnFromGraveyardToBattlefield { target: *card_id }];
    // GAP: "distribute four +1/+1 counters among any number of creatures" is
    // an interactive choice not in the catalog. Best-effort: lump 4 counters
    // onto the returned permanent.
    effects.push(Effect::AddCounters {
        target: *card_id,
        kind: CounterKind::PlusOnePlusOne,
        count: 4,
    });
    effects
}
