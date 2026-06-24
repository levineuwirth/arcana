//! Seraph of New Capenna // Seraph of New Phyrexia
//!
//! Front face: {2}{W} Creature — Angel Soldier 2/2, Flying. Activated ability:
//! {4}{B/P}: Transform this creature. Sorcery speed.
//! ({B/P} can be paid with either {B} or 2 life.)
//!
//! Back face: Creature — Phyrexian Angel, Flying. Whenever this creature attacks,
//! you may sacrifice another creature or artifact. If you do, this creature gets
//! +2/+1 until end of turn.
//!
//! GAP: the back-face attack trigger ("you may sacrifice another creature or artifact;
//! if you do, this creature gets +2/+1") is left omitted: there is no in-resolution
//! optional-sacrifice primitive (OptionalPaymentKind has only Mana/Life; no Sacrifice
//! variant) and so no way to gate the conditional pump on whether a sacrifice happened.
//! Wiring an unconditional pump would fabricate the effect; wiring the trigger with no
//! effect would be a no-op. Both are forbidden, so the whole back-face trigger is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seraph of New Capenna");
    let angel_sub = reg.interner_mut().intern("Angel");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel_sub);
    subtypes.0.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Seraph of New Phyrexia");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let angel_back_sub = reg.interner_mut().intern("Angel");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(angel_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {4}{B/P}: Transform this creature. Activate only as a sorcery.
            // ({B/P} parses as a Phyrexian colored pip — pay {B} or 2 life.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{B/P}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{B/P}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: front_transform,
            }),
        // GAP: back-face attack trigger (optional sacrifice → +2/+1) omitted; see header.
    )
}

fn front_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
