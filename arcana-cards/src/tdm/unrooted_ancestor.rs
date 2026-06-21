//! Unrooted Ancestor — `{2}{B}` 3/2 Spirit Cleric with Flash.
//! {1}, Sacrifice another creature: This creature gains indestructible until end
//! of turn. Tap it.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::targets::ObjectFilter;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unrooted Ancestor");
    let spirit = reg.interner_mut().intern("Spirit");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, Sacrifice another creature: This creature gains indestructible until end of turn. Tap it.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                sacrifice_other: Some(ObjectFilter {
                    types: Some(TypeLine::CREATURE.into()),
                    ..ObjectFilter::default()
                }),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: indestructible_and_tap,
        }),
    )
}

fn indestructible_and_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::Tap { target: ctx.source },
    ]
}
