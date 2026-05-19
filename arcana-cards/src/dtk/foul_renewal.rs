//! Foul Renewal — `{3}{B}` instant, "Return target creature card from your
//! graveyard to your hand. Target creature gets -X/-X until end of turn,
//! where X is the toughness of the card returned this way."
//!
//! GAP: dynamic -X/-X based on the toughness of the just-returned card is
//! not expressible; only the graveyard-to-hand return is implemented.

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
    let name = reg.interner_mut().intern("Foul Renewal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target creature card from your graveyard to your hand. Target creature gets -X/-X until end of turn, where X is the toughness of the card returned this way.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::creature(),
                        },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::target_creature(),
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
    // GAP: dynamic -X/-X based on returned card's toughness not expressible
    let Some(first) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(gy_id) = first else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToHand { target: *gy_id }]
}
