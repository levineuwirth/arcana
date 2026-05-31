//! Ojer Axonil, Deepest Might // Temple of Power — `{2}{R}{R}` transforming DFC.
//! Front (Ojer Axonil, Deepest Might — Legendary Creature — God, R, 4/4):
//!   Trample.
//!   If a red source you control would deal an amount of noncombat damage less
//!     than Ojer Axonil's power to an opponent, that source deals damage equal to
//!     Ojer Axonil's power instead.
//!   When Ojer Axonil dies, return it to the battlefield tapped and transformed
//!     under its owner's control.
//! Back (Temple of Power — Land):
//!   {T}: Add {R}.
//!   {2}{R}, {T}: Transform this land. Activate only if red sources you controlled
//!     dealt 4 or more noncombat damage this turn and only as a sorcery.
//!
//! GAPs:
//! - Damage-replacement "deals damage equal to its power instead": a dynamic
//!   noncombat-damage minimum replacement effect filtered to red sources you
//!   control is not expressible with the available Effect/replacement API; not
//!   modeled.
//! - Dies trigger: "return it tapped and transformed under its owner's control"
//!   — ReturnFromGraveyardToBattlefield has no tapped/transformed rider; the
//!   tapped + transformed-on-return aspect is not expressible. Not modeled.
//! - Back-face transform activation gate "red sources you controlled dealt 4 or
//!   more noncombat damage this turn" has no `conditions::`/`script::` predicate;
//!   the damage-this-turn precondition is not expressible (the activation is
//!   ungated beyond sorcery speed).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ojer Axonil, Deepest Might");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Temple of Power");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::LAND.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back (face 1): {T}: Add {R}.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {R}.".to_string(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: add_red,
            })
            // Back (face 1): {2}{R}, {T}: Transform this land (sorcery speed).
            // GAP: "only if red sources dealt 4+ noncombat damage this turn"
            //   precondition is not expressible.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}, {T}: Transform this land. Activate only as a sorcery."
                    .to_string(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: transform_self,
            }),
    )
}

fn add_red(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn transform_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
