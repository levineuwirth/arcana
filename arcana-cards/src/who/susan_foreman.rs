//! Susan Foreman — `{1}{G}` 1/1 Legendary Time Lord.
//!
//! Oracle:
//! * "If you would planeswalk, instead look at the top two cards of your planar
//!   deck, ... then planeswalk." — a planar-deck (Planechase) replacement
//!   effect; the planar deck / planeswalk mechanic is not modeled. GAP'd.
//! * "{T}: Add {G}." — a basic mana ability.
//! * Doctor's companion — not a usable KeywordAbility variant (commander-format
//!   deckbuilding rule, no battlefield effect). GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Susan Foreman");
    let time_lord = reg.interner_mut().intern("Time Lord");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(time_lord);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "Doctor's companion" is not a usable KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "If you would planeswalk, instead look at the top two cards of your
    // planar deck, ..." — the planar deck / planeswalk (Planechase) mechanic is
    // not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![],
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green,
            }),
    )
}

fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}
