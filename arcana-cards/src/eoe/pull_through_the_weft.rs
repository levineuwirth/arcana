//! Pull Through the Weft — `{3}{G}{G}` sorcery. "Return up to two target
//! nonland permanent cards from your graveyard to your hand, then return up
//! to two target land cards from your graveyard to the battlefield tapped."
//! Engine has no "return graveyard card to battlefield tapped" — closest is
//! `ReturnFromGraveyardToBattlefield` (no tapped flag). GAP the tapped half.

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
    let name = reg.interner_mut().intern("Pull Through the Weft");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Return up to two target nonland permanent cards from your graveyard to your hand, then return up to two target land cards from your graveyard to the battlefield tapped.".into(),
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                    },
                    count: TargetCount::UpTo(2),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    },
                    count: TargetCount::UpTo(2),
                    controller: None,
                },
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: ReturnFromGraveyardToBattlefield has no "tapped" flag; lands enter untapped.
    let mut out = Vec::new();
    let mut iter = entry.targets.targets.iter().filter_map(|t| match t {
        TargetChoice::Object(id) => Some(*id),
        _ => None,
    });
    // First up-to-two are nonland-to-hand, next up-to-two are land-to-battlefield.
    // We can't distinguish here without target-group metadata; conservatively dispatch
    // by reading them in order — the engine groups targets by TargetRequirement index.
    // Take up to two for hand-return:
    for _ in 0..2 {
        if let Some(id) = iter.next() {
            out.push(Effect::ReturnFromGraveyardToHand { target: id });
        }
    }
    for id in iter {
        out.push(Effect::ReturnFromGraveyardToBattlefield { target: id });
    }
    out
}
