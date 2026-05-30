//! Hydroelectric Specimen // Hydroelectric Laboratory
//!
//! Front face: `{2}{U}` Creature — Weird 1/4 with Flash.
//! When this creature enters, you may change the target of target instant or sorcery spell
//! with a single target to this creature. (GAP: retarget-spell effect not in catalog.)
//!
//! Back face: Land — Hydroelectric Laboratory.
//! As this land enters, you may pay 3 life. If you don't, it enters tapped.
//! {T}: Add {U}.
//!
//! GAP: ETB "pay 3 life or enter tapped" — replacement effect not in Effect catalog.
//! GAP: Front ETB "change target of target instant or sorcery with a single target" —
//! target-redirection effect not in catalog.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let front_name = reg.interner_mut().intern("Hydroelectric Specimen");
    let weird_sub = reg.interner_mut().intern("Weird");
    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(weird_sub);

    let front_chars = Characteristics {
        name: front_name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Hydroelectric Laboratory");
    let back_chars = Characteristics {
        name: back_name,
        mana_cost: None,
        colors: ColorSet::colorless(),
        types: TypeLine::LAND.into(),
        subtypes: SubtypeSet::default(),
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(front_name, front_chars)
            .with_mdfc_back(back_face)
            // Back-face land tap ability: {T}: Add {U}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {U}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: add_blue_mana,
            }),
    )
}

fn add_blue_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}
