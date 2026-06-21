//! Cryptic Cruiser — `{3}{U}` 3/3 Creature — Eldrazi Processor.
//! Devoid (this card has no color). "{2}{U}, Put a card an opponent owns from
//! exile into that player's graveyard: Tap target creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cryptic Cruiser");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let processor = reg.interner_mut().intern("Processor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(processor);

    // Devoid: the card is colorless (its colorless characteristic is captured
    // by the `colors` field; "Devoid" itself is not a modeled KeywordAbility).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP: cost "Put a card an opponent owns from exile into that
            // player's graveyard" (Processor cost) has no ActivationCost
            // field; modeled with the {2}{U} mana portion only.
            text: "{2}{U}, Put a card an opponent owns from exile into that player's graveyard: Tap target creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tap_target,
        }),
    )
}

fn tap_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Tap { target: *id }]
}
