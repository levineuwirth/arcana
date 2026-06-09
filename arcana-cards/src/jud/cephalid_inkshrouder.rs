//! Cephalid Inkshrouder — `{2}{U}` 2/1 Octopus.
//! `Discard a card: This creature gains shroud until end of turn and can't be blocked this turn.`
//! The "Discard a card" cost is modeled via the `discard_other` activation
//! cost. GAP: no Effect for "can't be blocked this turn".

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cephalid Inkshrouder");
    let octopus = reg.interner_mut().intern("Octopus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(octopus);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a card: This creature gains shroud until end of turn and can't be blocked this turn.".into(),
                cost: ActivationCost {
                    discard_other: Some(arcana_core::targets::ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: gain_shroud,
            }),
    )
}

fn gain_shroud(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: No Effect for "can't be blocked this turn".
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Shroud,
        duration: Duration::EndOfTurn,
    }]
}
