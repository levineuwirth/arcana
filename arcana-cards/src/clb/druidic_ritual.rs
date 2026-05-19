//! Druidic Ritual — `{2}{G}` sorcery, "You may mill three cards. Then return
//! up to one creature card and up to one land card from your graveyard to your
//! hand."
//!
//! The "you may" on mill is approximated by emitting the mill (the engine
//! does not currently gate optional effects; GAP noted). The graveyard returns
//! use TargetFilter::Card with graveyard zones — two separate UpTo(1) targets.
//!
//! GAP: "you may" qualifier on mill (no optional-effect gate); returning up to
//! one land card from graveyard uses Card zone filter but land cards in
//! graveyard targeting is best-effort.

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
    let name = reg.interner_mut().intern("Druidic Ritual");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You may mill three cards. Then return up to one creature card and up to one land card from your graveyard to your hand.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
                        },
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
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
    // GAP: "you may" qualifier on mill not expressible
    let mut effects = vec![Effect::Mill { player: entry.controller, count: 3 }];
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::ReturnFromGraveyardToHand { target: *id });
        }
    }
    effects
}
